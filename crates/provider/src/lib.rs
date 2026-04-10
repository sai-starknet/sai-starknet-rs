//! Clients for interacting with Starknet nodes and sequencers.
//!
//! This crate provides the [`Provider`] trait for abstraction over means of accessing the Starknet
//! network. The most commonly used implementation is [`JsonRpcClient`] with
//! [`HttpTransport`](jsonrpc::HttpTransport).

#![deny(missing_docs)]

mod provider;
pub use provider::{
    Provider, ProviderError, ProviderImplError, ProviderRequestData, ProviderResponseData,
    StreamUpdateData,
};

/// Module containing types related to JSON-RPC clients and servers.
pub mod jsonrpc;
pub use jsonrpc::JsonRpcClient;

// Re-export
pub use url::Url;
