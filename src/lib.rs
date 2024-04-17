use std::{any::Any, collections::HashMap, time::SystemTime};

use anchor_lang::{prelude::borsh, AnchorDeserialize, Discriminator};
use solana_account_decoder::UiAccountEncoding;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::{
    client_error::Result as ClientResult,
    config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    filter::{Memcmp, RpcFilterType},
};
use state::compute_dynamic_interest;

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
    let pool_filters = vec![
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, Pool::DISCRIMINATOR.to_vec())),
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(41, token.to_bytes().to_vec())),
    ];

    // Create filter for fecthing ongoing loans with given token
    let loan_filters = vec![
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, Loan::DISCRIMINATOR.to_vec())),
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(138, token.to_bytes().to_vec())),
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(9, vec![1])),
    ];

    let (pool_accounts, loan_accounts) = tokio::try_join!(
        client.get_program_accounts_with_config(
            &crate::ID,
            RpcProgramAccountsConfig {
                filters: Some(pool_filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        ),
        client.get_program_accounts_with_config(
            &crate::ID,
            RpcProgramAccountsConfig {
                filters: Some(loan_filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        )
    )?;

    let pools: Result<Vec<Pool>, borsh::maybestd::io::Error> = pool_accounts
        .into_iter()
        .map(|(_pubkey, account)| {
            let mut data = &account.data[8..];
            Pool::deserialize(&mut data)
        })
        .collect();

    let loans: Result<Vec<Loan>, borsh::maybestd::io::Error> = loan_accounts
        .into_iter()
        .map(|(_pubkey, account)| {
            let mut data = &account.data[8..];
            Loan::deserialize(&mut data)
        })
        .collect();

    let mut map: HashMap<Pubkey, u64> = HashMap::new();

    for pool in pools? {
        *map.entry(pool.owner).or_insert(0) += pool.total_amount;
    }

    let now: i64 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Incorrect system time")
        .as_secs()
        .try_into()
        .expect("Conversion to u64 failed");

    for loan in loans? {
        let ongoing_interest = compute_dynamic_interest(&loan, now);
        *map.entry(loan.lender).or_insert(0) += ongoing_interest;
    }


    Ok(map
        .into_iter()
        .map(|(user, supply)| RainSupplier { user, supply })
        .collect())
}

pub async fn fetch_rain_borrowers(
    client: &RpcClient,
    token: &Pubkey,
) -> ClientResult<Vec<RainBorrower>> {
    // Create filter for fecthing loans with given token
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
