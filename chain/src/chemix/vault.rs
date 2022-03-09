use crate::k256::ecdsa::SigningKey;
use anyhow::Result;
use chemix_models::order::BookOrder;
use chrono::Local;
use common::env;
use common::env::CONF as ENV_CONF;
use common::types::order::Side;
use common::types::*;
use common::utils::algorithm::{sha256, u8_arr_to_str};
use common::utils::math::MathOperation;
use ethers::prelude::*;
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;
use crate::chemix::ChemixContractClient;
use crate::gen_contract_client;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThawBalances {
    pub token: Address,
    pub from: Address,
    pub amount: U256,
    pub decimal: u32,
}

#[derive(Clone, Debug)]
pub struct SettleValues3 {
    pub user: Address,
    pub token: Address,
    pub isPositive: bool,
    pub incomeTokenAmount: U256,
}

#[derive(Clone)]
pub struct Vault {}

abigen!(
    ChemixVault,
    "../contract/Vault.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

lazy_static! {
    static ref VAULT_ADDR : Address = {
       let vault = ENV_CONF.chemix_vault.to_owned().unwrap();
        Address::from_str(vault.to_str().unwrap()).unwrap()
    };
}


impl ChemixContractClient<Vault>{
    pub fn new(prikey: &str) -> ChemixContractClient<Vault> {
        ChemixContractClient {
            client: gen_contract_client(prikey),
            contract_addr: VAULT_ADDR.clone(),
            phantom: PhantomData,
        }
    }

    pub async fn thaw_balances(
        &self,
        users: Vec<ThawBalances>,
        cancel_id: [u8; 32],
    ) -> Result<Option<TransactionReceipt>> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let _now = Local::now().timestamp_millis() as u64;

        let users2 = users
            .iter()
            .map(|x| ThawInfos {
                token: x.token,
                addr: x.from,
                thaw_amount: x.amount,
            })
            .collect::<Vec<ThawInfos>>();

        let result: Option<TransactionReceipt> = contract
            .thaw_balance(cancel_id, users2)
            .legacy()
            .send()
            .await?
            .await?;
        info!("thaw_balance res = {:?}", result);
        Ok(result)
    }

    pub async fn settlement_trades2(
        &self,
        last_index: u32,
        last_hash: [u8; 32],
        trades: Vec<SettleValues3>,
    ) -> Result<Option<TransactionReceipt>> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let trades2 = trades
            .iter()
            .map(|x| SettleValues {
                user: x.user,
                token: x.token,
                is_positive: x.isPositive,
                income_token_amount: x.incomeTokenAmount,
            })
            .collect::<Vec<SettleValues>>();
        //todo:只await一次
        let result: Option<TransactionReceipt> = contract
            .settlement(U256::from(last_index), last_hash, trades2)
            .legacy()
            .send()
            .await?
            .await?;
        info!("settlement_trades res = {:?}", result);
        Ok(result)
    }

    pub async fn filter_settlement_event(&mut self, height: U64) -> Result<Vec<String>> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let new_orders: Vec<SettlementFilter> = contract
            .settlement_filter()
            .from_block(height)
            .query()
            .await
            .unwrap();

        let settlement_flag = new_orders
            .iter()
            .map(|x| u8_arr_to_str(x.hash_data))
            .collect::<Vec<String>>();
        Ok(settlement_flag)
    }

    //thaws
    pub async fn filter_thaws_event(&mut self, height: U64) -> Result<Vec<String>> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let new_orders: Vec<ThawBalanceFilter> = contract
            .thaw_balance_filter()
            .from_block(height)
            .query()
            .await
            .unwrap();

        let thaws_flag = new_orders
            .iter()
            .map(|x| u8_arr_to_str(x.flag))
            .collect::<Vec<String>>();
        Ok(thaws_flag)
    }

    pub async fn vault_balance_of(
        &mut self,
        token: String,
        from: String,
    ) -> Result<(U256, U256)> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let token = Address::from_str(token.as_str()).unwrap();
        let from = Address::from_str(from.as_str()).unwrap();
        let value = contract.balance_of(token, from).call().await?;
        info!("vault_balance_of result  {:?}", value);
        Ok(value)
    }

    pub async fn vault_total_withdraw_volume(&self, token: String) -> Result<U256> {
        let contract = ChemixVault::new(self.contract_addr, self.client.clone());
        let token = Address::from_str(token.as_str()).unwrap();
        let value = contract.total_withdraw(token).call().await?;
        info!("vault_balance_of result  {:?}", value);
        Ok(value)
    }
}