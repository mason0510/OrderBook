mod util;

extern crate ethers_contract_abigen;
extern crate num;
extern crate rand;
extern crate rsmq_async;

use ethers::{prelude::*, types::U256};

//use ethers::providers::Ws;
use ethers_contract_abigen::Address;

use rsmq_async::{Rsmq, RsmqConnection};

use std::env;

use std::str::FromStr;

use tokio::time;

use rand::Rng;
use util::MathOperation;

abigen!(
    SimpleContract,
    "../contract/chemix_trade_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

async fn new_order(side: String, price: f64, amount: f64) {
    let mut rsmq = Rsmq::new(Default::default())
        .await
        .expect("connection failed");
    let price_nano = (price * 100000000.0) as u64;
    let amount_nano = (amount * 100000000.0) as u64;

    let event = NewOrderFilter {
        user: Address::from_str("0xbc1Bd19FD1b220e989F8bF75645b9B7028Fc255B").unwrap(),
        base_token: "USDT".to_string(),
        quote_token: "BTC".to_string(),
        side,
        amount: U256::from(amount_nano),
        price: U256::from(price_nano),
    };
    let events = vec![event];

    let json_str = serde_json::to_string(&events).unwrap();
    let channel_name = match env::var_os("CHEMIX_MODE") {
        None => "bot_local".to_string(),
        Some(mist_mode) => {
            format!("bot_{}", mist_mode.into_string().unwrap())
        }
    };

    rsmq.send_message(channel_name.as_str(), json_str, None)
        .await
        .expect("failed to send message");
}

//fn cancle_order() {}

//todo: send bsc
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let base_price = 40000.0f64;
    let base_amount = 1.0f64;

    loop {
        let mut rng = rand::thread_rng();
        let price_add: f64 = rng.gen_range(-1000.0..1000.0);
        let amount_add: f64 = rng.gen_range(-1.0..1.0);
        let side_random: u8 = rng.gen_range(0..=1);
        let side = match side_random {
            0 => "buy".to_string(),
            _ => "sell".to_string(),
        };

        let price = (base_price + price_add).to_fix(8);
        let amount = (base_amount + amount_add).to_fix(8);
        println!(
            "[newOrder]: side {} price {},amount {}",
            side, price, amount
        );
        new_order(side, price, amount).await;
        tokio::time::sleep(time::Duration::from_millis(1000)).await;
    }

    //[newOrder]: side buy price 40503.19859207,amount 0.36172409
    // [newOrder]: side sell price 39036.04489557,amount 1.91700874
    //new_order("buy".to_string(),40503.19859207,0.36172409).await;
    //tokio::time::sleep(time::Duration::from_millis(5000)).await;
    //new_order("sell".to_string(),39036.04489557,1.91700874).await;

    Ok(())
}
