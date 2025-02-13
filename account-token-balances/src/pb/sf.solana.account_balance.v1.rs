use account_sol_balances::pb::sf::solana::account_sol_balance::v1::SolAccountStats;
use spl_token::pb::sf::solana::spl::v1::SplTokenMeta;
use substreams_solana_system_program_transfers_only::pb::sf::solana::block_meta::v1::SystemPrograpTransferOnlyMeta;

// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub token_balances: ::prost::alloc::vec::Vec<AccountStats>,
    #[prost(message, repeated, tag="2")]
    pub sol_balances: ::prost::alloc::vec::Vec<SolAccountStats>,
    #[prost(message, repeated, tag="3")]
    pub spl_token_transfer: ::prost::alloc::vec::Vec<SplTokenMeta>,
    #[prost(message, repeated, tag="4")]
    pub system_transfers: ::prost::alloc::vec::Vec<SystemPrograpTransferOnlyMeta>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAccountStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub token_account: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub mint: ::prost::alloc::string::String,
    #[prost(double, required, tag="6")]
    pub post_balance: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub token_account: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub mint: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="6")]
    pub decimals: u32,
    #[prost(double, required, tag="7")]
    pub post_balance: f64,
    #[prost(string, required, tag="8")]
    pub ui_amount_string: ::prost::alloc::string::String,
    #[prost(string, required, tag="9")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(double, required, tag="10")]
    pub pre_balance: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolOutput {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<SolAccountStats>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenOutput {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<SplTokenMeta>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenAccounts {
    #[prost(string, optional, tag="1")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub rent_sysvar: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub signer_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub delegate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub fund_relocation_sys_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub funding_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub mint_funding_sys_program: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenArg {
    #[prost(uint64, optional, tag="1")]
    pub amount: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub authority_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub freeze_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub freeze_authority_option: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub new_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="7")]
    pub new_authority_option: ::core::option::Option<i32>,
    #[prost(string, optional, tag="8")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="9")]
    pub decimals: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub extension_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="11")]
    pub ui_amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="12")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbTransfer {
    #[prost(uint64, required, tag="1")]
    pub lamports: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTransferWithSeed {
    #[prost(uint64, required, tag="1")]
    pub lamports: u64,
    #[prost(string, required, tag="2")]
    pub from_seed: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub from_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemPrograpTransferOnlyArg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transfer: ::core::option::Option<PbTransfer>,
    #[prost(message, optional, tag="3")]
    pub transfer_with_seed: ::core::option::Option<PbTransferWithSeed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub funding_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub recipient_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub base_account: ::core::option::Option<::prost::alloc::string::String>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemPrograpTransferOnlyOutput {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<SystemPrograpTransferOnlyMeta>,
}
// @@protoc_insertion_point(module)
