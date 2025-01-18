pub mod pb;
mod utils;

use std::collections::HashMap;
use account_sol_balances;
use substreams_solana_system_program_transfers_only;
use spl_token;

use pb::sf::solana::account_balance::v1::{AccountStats, Output};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, TransactionStatusMeta};
use utils::convert_to_date;


#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let block_slot = block.slot;

    let block_date = match block.block_time.as_ref() {
        Some(block_time) => match convert_to_date(block_time.timestamp) {
            Ok(date) => date,
            Err(_) => "Error converting block time to date".to_string(),
        },
        None => "Block time is not available".to_string(),
    };

    let account_block = block.clone();
    let account_sol_block = block.clone();
    let system_program_transfer_block = block.clone();
    let account_sol_balances_result = account_sol_balances::map_block(account_block);

    let (account_sol_balances, account_sol_balances_error) = match account_sol_balances_result {
        Ok(output) => (Some(output), None), // 成功时，output 不为空
        Err(error) => (None, Some(error)), // 失败时，error 不为空
    };

    let spl_transfer_result = spl_token::map_block(account_sol_block);
    let (spl_transfer, spl_transfer_error) = match spl_transfer_result {
        Ok(output) => (Some(output), None), // 成功时，output 不为空
        Err(error) => (None, Some(error)), // 失败时，error 不为空
    };

    let system_program_transfers_only_result = substreams_solana_system_program_transfers_only::map_block(system_program_transfer_block);
    let (system_program_transfers_only, system_program_transfers_only_error) = match system_program_transfers_only_result {
        Ok(output) => (Some(output), None), // 成功时，output 不为空
        Err(error) => (None, Some(error)), // 失败时，error 不为空
    };


    let mut latest_stats: HashMap<(String, String), AccountStats> = HashMap::new();

    for trx in block.transactions.iter() {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => continue,
        };

        if meta.err.is_some() { 
            continue;
        }

        let accounts = trx.resolved_accounts_as_strings();
        update_latest_stats(&mut latest_stats, meta, &accounts, block_slot, &block_date);
    }

    Ok(Output {
        token_balances: latest_stats.into_values().collect(),
        sol_balances: account_sol_balances.unwrap().data,
        spl_token_transfer: spl_transfer.unwrap().data,
        system_transfers: system_program_transfers_only.unwrap().data,
    })
}


fn update_latest_stats(
    latest_stats: &mut HashMap<(String, String), AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
) {
    // Keep track of accounts in post_token_balances
    let mut post_balance_keys: HashMap<String, bool> = HashMap::new();

    // Update the stats for post_token_balances
    meta.post_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());
        let ui_amount = token_balance.ui_token_amount.as_ref().map_or(0.0, |amt| amt.ui_amount);
        let owner = token_balance.owner.to_string();

        post_balance_keys.insert(account.clone(), true);  // Mark this account as having a post balance

        latest_stats
            .entry(key)
            .and_modify(|stats| stats.post_balance = ui_amount)
            .or_insert_with(|| AccountStats {
                block_slot,
                block_date: block_date.to_string(),
                token_account: account.clone(),
                owner,
                mint: token_balance.mint.to_string(),
                post_balance: ui_amount,
            });
    });

    // Set post_balance to 0 for accounts in pre_token_balances but not in post_token_balances
    meta.pre_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());
        
        if !post_balance_keys.contains_key(account) {
            let owner = token_balance.owner.to_string();

            // If the account exists in pre_token_balances but not in post_token_balances, set post_balance to 0
            latest_stats
                .entry(key.clone())
                .and_modify(|stats| stats.post_balance = 0.0)
                .or_insert_with(|| AccountStats {
                    block_slot,
                    block_date: block_date.to_string(),
                    token_account: account.clone(),
                    owner,
                    mint: token_balance.mint.to_string(),
                    post_balance: 0.0,
                });
        }
    });
}
