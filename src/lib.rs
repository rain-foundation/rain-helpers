use std::collections::HashMap;

use anchor_lang::{prelude::borsh, AnchorDeserialize, Discriminator};
use solana_account_decoder::UiAccountEncoding;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::{
    client_error::Result as ClientResult,
    config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    filter::{Memcmp, RpcFilterType},
};

use crate::state::{Loan, Pool};

pub mod state;

anchor_lang::declare_id!("RainEraPU5yDoJmTrHdYynK9739GkEfDsE4ffqce2BR");

#[derive(Debug)]
pub struct RainSupplier {
    pub user: Pubkey,
    pub supply: u64,
}

#[derive(Debug)]
pub struct RainBorrower {
    pub user: Pubkey,
    pub borrow: u64,
}

pub async fn fetch_rain_pools(
    client: &RpcClient,
    token: &Pubkey,
) -> ClientResult<Vec<RainSupplier>> {
    // Create filter for fecthing pools providing given token
    let filters = vec![
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, Pool::DISCRIMINATOR.to_vec())),
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(41, token.to_bytes().to_vec())),
    ];

    let accounts = client
        .get_program_accounts_with_config(
            &crate::ID,
            RpcProgramAccountsConfig {
                filters: Some(filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        )
        .await?;

    let pools: Result<Vec<Pool>, borsh::maybestd::io::Error> = accounts
        .into_iter()
        .map(|(_pubkey, account)| {
            let mut data = &account.data[8..];
            Pool::deserialize(&mut data)
        })
        .collect();

    Ok(pools?
        .into_iter()
        .map(|pool| RainSupplier {
            user: pool.owner,
            supply: pool.total_amount,
        })
        .collect())
}

pub async fn fetch_rain_borrowers(
    client: &RpcClient,
    token: &Pubkey,
) -> ClientResult<Vec<RainBorrower>> {
    // Create filter for fecthing pools providing given token
    let filters = vec![
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, Loan::DISCRIMINATOR.to_vec())),
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(138, token.to_bytes().to_vec())),
    ];

    let accounts = client
        .get_program_accounts_with_config(
            &crate::ID,
            RpcProgramAccountsConfig {
                filters: Some(filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        )
        .await?;

    let loans: Result<Vec<Loan>, borsh::maybestd::io::Error> = accounts
        .into_iter()
        .map(|(_pubkey, account)| {
            let mut data = &account.data[8..];
            Loan::deserialize(&mut data)
        })
        .collect();

    let mut map: HashMap<Pubkey, u64> = HashMap::new();

    for loan in loans? {
        *map.entry(loan.borrower).or_insert(0) += loan.amount;
    }

    Ok(map
        .into_iter()
        .map(|(user, borrow)| RainBorrower { user, borrow })
        .collect())
}
