//! EVM RPC client.
use std::time::Duration;

use crate::errors::{Error, Result};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{U256,Address};
use reqwest::Url;

/// Fetches the chain Id from "{http_rpc}/ext/bc/{chain_id_alias}/rpc".
/// "chain_id_alias" is "C" for C-chain, and blockchain Id for subnet-evm.
pub async fn chain_id(rpc_ep: &str) -> Result<U256> {
    let provider = ProviderBuilder::new()
        .on_http(Url::parse(rpc_ep).map_err(|e| {
            // TODO: check retryable
            Error::API {
                message: format!("failed to parse URL '{}': {}", rpc_ep, e),
                retryable: false,
            }
        })?);

    // Fetch the chain ID
    let chain_id = provider.get_chain_id().await.map_err(|e| {
        // Convert the error to your custom Error type
        Error::API {
            message: format!("failed to get chain ID: {}", e),
            retryable: false,
        }
    })?;
    Ok(U256::from(chain_id))
}

/// Fetches the balance from "{http_rpc}/ext/bc/{chain_id_alias}/rpc".
/// "chain_id_alias" is "C" for C-chain, and blockchain Id for subnet-evm.
/// ref. <https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance>
pub async fn get_balance(rpc_ep: &str, eth_addr: Address) -> Result<U256> {
    let provider = ProviderBuilder::new()
        .on_http(Url::parse(rpc_ep).map_err(|e| {
            // TODO: check retryable
            Error::API {
                message: format!("failed to parse URL '{}': {}", rpc_ep, e),
                retryable: false,
            }
        })?);       
        let balance = provider.get_balance(eth_addr).await.map_err(|e| {
            // Convert the error to your custom Error type
            Error::API {
                message: format!("failed to get balance: {}", e),
                retryable: false,
            }
        })?;
        Ok(balance)   
}
