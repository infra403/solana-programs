pub mod pb;
mod utils;

use std::collections::HashMap;
use account_sol_balances;
use substreams_solana_system_program_transfers_only;
use spl_token;

use pb::sf::solana::account_balance::v1::{AccountStats, Output};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, TransactionStatusMeta, ConfirmedTransaction};
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
        let sig = get_signature(trx);
        update_latest_stats(&mut latest_stats, meta, &accounts, block_slot, &block_date, &sig);
    }

    Ok(Output {
        token_balances: latest_stats.into_values().collect(),
        sol_balances: account_sol_balances.unwrap().data,
        spl_token_transfer: spl_transfer.unwrap().data,
        // system_transfers: vec![]
        system_transfers: system_program_transfers_only.unwrap().data,
    })
}

fn update_latest_stats(
    latest_stats: &mut HashMap<(String, String), AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
    sig: &str,
) {
    // ---------------------------------------
    // 1. 先处理 pre_token_balances，填充 pre_balance
    // ---------------------------------------
    meta.pre_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());

        let ui_amount = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(0.0, |amt| amt.ui_amount);
        let owner = token_balance.owner.to_string();
        let decimals = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(0, |amt| amt.decimals);
        let ui_amount_string = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(String::new(), |amt| amt.ui_amount_string.clone());

        latest_stats
            .entry(key)
            // 如果已经存在，则仅修改 pre_balance
            .and_modify(|stats| stats.pre_balance = ui_amount)
            // 如果不存在，则新建并默认 post_balance = 0
            .or_insert_with(|| AccountStats {
                block_slot,
                block_date: block_date.to_string(),
                token_account: account.clone(),
                owner,
                mint: token_balance.mint.to_string(),
                pre_balance: ui_amount,
                post_balance: 0.0,  // 默认先设置为 0
                decimals,
                ui_amount_string,
                transaction: sig.to_string(),
            });
    });

    // ---------------------------------------
    // 2. 为了像以前一样区分哪些账户有 post_balance，我们先记录下 post_balance 的账户
    // ---------------------------------------
    let mut post_balance_keys: HashMap<String, bool> = HashMap::new();

    // ---------------------------------------
    // 3. 再处理 post_token_balances，填充 post_balance
    // ---------------------------------------
    meta.post_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());

        let ui_amount = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(0.0, |amt| amt.ui_amount);
        let owner = token_balance.owner.to_string();
        let decimals = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(0, |amt| amt.decimals);
        let ui_amount_string = token_balance
            .ui_token_amount
            .as_ref()
            .map_or(String::new(), |amt| amt.ui_amount_string.clone());

        // 标记这个账户有 post_balance
        post_balance_keys.insert(account.clone(), true);

        latest_stats
            .entry(key)
            // 如果已经有这个 key，则只更新 post_balance
            .and_modify(|stats| {
                stats.post_balance = ui_amount;
                // 如果需要，这里也可以更新其他字段，但要注意是否被覆盖
            })
            // 如果还没有，则新建 AccountStats，pre_balance = 0
            .or_insert_with(|| AccountStats {
                block_slot,
                block_date: block_date.to_string(),
                token_account: account.clone(),
                owner,
                mint: token_balance.mint.to_string(),
                pre_balance: 0.0,  // 没在 pre_token_balances 出现过则默认设为 0
                post_balance: ui_amount,
                decimals,
                ui_amount_string,
                transaction: sig.to_string(),
            });
    });

    // ---------------------------------------
    // 4. 对于仅在 pre_token_balances 中出现而不在 post_token_balances 中出现的账户，post_balance 设置为 0
    // ---------------------------------------
    meta.pre_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());

        // 如果不在 post_balance_keys 里，说明该账号 post_balance = 0
        if !post_balance_keys.contains_key(account) {
            let owner = token_balance.owner.to_string();

            latest_stats
                .entry(key.clone())
                .and_modify(|stats| stats.post_balance = 0.0)
                .or_insert_with(|| AccountStats {
                    block_slot,
                    block_date: block_date.to_string(),
                    token_account: account.clone(),
                    owner,
                    mint: token_balance.mint.to_string(),
                    pre_balance: 0.0,
                    post_balance: 0.0,
                    decimals: 0,
                    ui_amount_string: "".to_string(),
                    transaction: sig.to_string(),
                });
        }
    });
}


pub fn get_signature(transaction: &ConfirmedTransaction) -> String {
    bs58::encode(transaction.transaction.as_ref().unwrap().signatures.get(0).unwrap()).into_string()
}