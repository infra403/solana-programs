use spl_token::pb::sf::solana::spl::v1::SplTokenMeta;
use substreams_solana_system_program_transfers_only::pb::sf::solana::block_meta::v1::SystemPrograpTransferOnlyMeta;

// @generated
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
    #[prost(double, required, tag="6")]
    pub post_balance: f64,
}
// @@protoc_insertion_point(module)
