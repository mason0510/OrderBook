pub mod chain;
pub mod market;
pub mod order;
pub mod snapshot;
pub mod thaws;
pub mod tokens;
pub mod trade;

#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use postgres::{Client, NoTls, Row};
use std::any::Any;
use std::env;

use std::fmt::Debug;

use anyhow::anyhow;
use std::sync::Mutex;

extern crate chrono;
extern crate postgres;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate common;

use chrono::Local;

use crate::trade::TradeInfo;

use crate::thaws::Thaws;

extern crate rustc_serialize;
use crate::snapshot::Snapshot;
use serde::Deserialize;
use serde::Serialize;

#[derive(RustcEncodable, Deserialize, Debug, PartialEq, Clone, Serialize)]
pub enum TimeScope {
    NoLimit,
    SevenDay,
    OneDay,
}

impl TimeScope {
    pub fn filter_str(&self) -> &'static str {
        match self {
            TimeScope::NoLimit => "",
            TimeScope::SevenDay => "where created_at > NOW() - INTERVAL '7 day'",
            TimeScope::OneDay => "where created_at > NOW() - INTERVAL '24 hour'",
        }
    }
}

lazy_static! {
    static ref CLIENTDB: Mutex<postgres::Client> = Mutex::new(connetDB().unwrap());
}

pub fn restartDB() -> bool {
    let now = Local::now();

    println!("restart postgresql {:?}", now);
    // let client =  connetDB();
    if let Some(client) = connetDB() {
        *crate::CLIENTDB.lock().unwrap() = client;
        return true;
    }
    false
}

fn connetDB() -> Option<postgres::Client> {
    let dbname = match env::var_os("CHEMIX_MODE") {
        None => "chemix_local".to_string(),
        Some(mist_mode) => {
            format!("chemix_{}", mist_mode.into_string().unwrap())
        }
    };

    let url = format!(
        "host=localhost user=postgres port=5432 password=postgres dbname={}",
        dbname
    );

    match Client::connect(&url, NoTls) {
        Ok(client) => {
            eprintln!("connect postgresql successfully");
            Some(client)
        }
        Err(error) => {
            eprintln!("connect postgresql failed,{:?}", error);
            None
        }
    }
}

pub fn query(raw_sql: &str) -> anyhow::Result<Vec<Row>> {
    let mut try_times = 5;
    loop {
        match crate::CLIENTDB.lock().unwrap().query(raw_sql, &[]) {
            Ok(data) => {
                return Ok(data);
            }
            Err(error) => {
                if try_times == 0 {
                    //Err(anyhow!("Missing attribute: {}", missing));
                    return Err(anyhow!("retry query failed"));
                } else {
                    info!("error {:?}", error);
                    crate::restartDB();
                    try_times -= 1;
                    continue;
                }
            }
        }
    }
}

pub fn execute(raw_sql: &str) -> anyhow::Result<u64> {
    let mut try_times = 5;
    loop {
        match crate::CLIENTDB.lock().unwrap().execute(raw_sql, &[]) {
            Ok(data) => {
                return Ok(data);
            }
            Err(error) => {
                if try_times == 0 {
                    //Err(anyhow!("Missing attribute: {}", missing));
                    return Err(anyhow!("retry execute failed"));
                } else {
                    info!("error {:?}", error);
                    crate::restartDB();
                    try_times -= 1;
                    continue;
                }
            }
        }
    }
}

pub trait FormatSql {
    fn string4sql(&self) -> String;
}

impl FormatSql for String {
    fn string4sql(&self) -> String {
        format!("'{}'", self)
    }
}

pub fn struct2array<T: Any + Debug>(value: &T) -> Vec<String> {
    let mut values: Vec<String> = vec![];
    let value = value as &dyn Any;

    match value.downcast_ref::<TradeInfo>() {
        Some(trade) => {
            values.push(trade.id.string4sql());
            values.push(trade.block_height.to_string());
            values.push(trade.transaction_hash.string4sql());
            values.push(trade.hash_data.string4sql());
            values.push(trade.status.as_str().to_string().string4sql());
            values.push(trade.market_id.string4sql());
            values.push(trade.maker.string4sql());
            values.push(trade.taker.string4sql());
            values.push(trade.price.to_string());
            values.push(trade.amount.to_string());
            values.push(trade.taker_side.as_str().to_string().string4sql());
            values.push(trade.maker_order_id.string4sql());
            values.push(trade.taker_order_id.string4sql());
            values.push(trade.updated_at.string4sql());
            values.push(trade.created_at.string4sql());
        }
        None => (),
    };
    match value.downcast_ref::<order::OrderInfo>() {
        Some(trade) => {
            values.push(trade.id.string4sql());
            values.push(trade.index.to_string());
            values.push(trade.transaction_hash.string4sql());
            values.push(trade.block_height.to_string());
            values.push(trade.hash_data.string4sql());
            values.push(trade.market_id.string4sql());
            values.push(trade.account.string4sql());
            values.push(trade.side.as_str().to_string().string4sql());
            values.push(trade.price.to_string());
            values.push(trade.amount.to_string());
            values.push(trade.status.as_str().to_string().string4sql());
            values.push(trade.available_amount.to_string());
            values.push(trade.matched_amount.to_string());
            values.push(trade.canceled_amount.to_string());
            values.push(trade.updated_at.string4sql());
            values.push(trade.created_at.string4sql());
        }
        None => (),
    };

    match value.downcast_ref::<Thaws>() {
        Some(thaw) => {
            values.push(thaw.order_id.string4sql());
            values.push(thaw.account.string4sql());
            values.push(thaw.market_id.string4sql());
            values.push(thaw.transaction_hash.string4sql());
            values.push(thaw.block_height.to_string().string4sql());
            values.push(thaw.thaws_hash.string4sql());
            values.push(thaw.side.as_str().to_string().string4sql());
            values.push(thaw.status.as_str().to_string().string4sql());
            values.push(thaw.amount.to_string());
            values.push(thaw.price.to_string());
            values.push(thaw.updated_at.string4sql());
            values.push(thaw.created_at.string4sql());
        }
        None => (),
    };

    /***
        pub traders: i32,
    pub transactions: i32,
    pub order_volume: U256,
    pub withdraw: i32,
    pub trade_volume: U256,
    pub trading_pairs: i32,
    pub cec_price: U256,
    pub snapshot_time: i64
    */
    match value.downcast_ref::<Snapshot>() {
        Some(dash) => {
            values.push(dash.traders.to_string().string4sql());
            values.push(dash.transactions.to_string().string4sql());
            values.push(dash.order_volume.to_string().string4sql());
            values.push(dash.withdraw.to_string().string4sql());
            values.push(dash.trade_volume.to_string().string4sql());
            values.push(dash.trading_pairs.to_string().string4sql());
            values.push(dash.cec_price.to_string().string4sql());
            values.push(dash.snapshot_time.to_string().string4sql());
        }
        None => (),
    };

    values
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
