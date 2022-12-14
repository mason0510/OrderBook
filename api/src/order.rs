use crate::trade::Trade2;
use crate::OrderSide;
use chemix_models::market::get_markets;
use chemix_models::order::OrderInfoPO;
use chemix_models::trade::{list_trades, TradeFilter};

use common::utils::math::{u256_to_f64, U256_ZERO};
use common::utils::time::time2unix;
use ethers_core::types::U256;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Trade {
    code: u8,
    msg: String, //200 default success
    data: String,
}
//*               [50000.0,10.0001],

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OrderDetail {
    pub id: String,
    pub market_id: String,
    pub index: String,
    pub transaction_hash: String,
    pub account: String,
    pub price: f64,
    pub average_price: f64,
    pub total_amount: f64,
    pub canceled_amount: f64,
    pub matched_amount: f64,
    pub total_volume: f64,
    pub side: OrderSide,
    pub status: String,
    pub trades: Vec<Trade2>,
    pub created_at: u64,
}

pub fn get_order_detail(order: &OrderInfoPO) -> OrderDetail {
    let market_info = get_markets(order.market_id.as_str()).unwrap();
    let (base_decimal, quote_decimal) = (
        market_info.base_contract_decimal as u32,
        market_info.quote_contract_decimal as u32,
    );
    let trades = list_trades(TradeFilter::OrderId(&order.id));
    let mut total_volume = U256_ZERO;
    let mut trades2 = Vec::<Trade2>::new();
    for trade in trades {
        total_volume += trade.amount * trade.price / u256_power!(10u32,base_decimal);

        trades2.push(Trade2 {
            id: trade.id.clone(),
            market_id: trade.market_id.clone(),
            price: u256_to_f64(trade.price, quote_decimal),
            amount: u256_to_f64(trade.amount, base_decimal),
            height: trade.block_height as u32,
            status: trade.status.as_str().to_string(),
            taker_side: trade.taker_side.clone(),
            updated_at: time2unix(trade.created_at.clone()),
        });
    }
    let average_price = if total_volume != U256_ZERO {
        info!(
            "___total_volume={},order.matched_amount={}",
            total_volume, order.matched_amount
        );
        u256_to_f64(total_volume, quote_decimal)
            / u256_to_f64(order.matched_amount, base_decimal)
    } else {
        0.0
    };
    OrderDetail {
        id: order.id.clone(),
        market_id: order.market_id.clone(),
        index: order.index.to_string(),
        transaction_hash: order.transaction_hash.clone(),
        account: order.account.to_string(),
        total_amount: u256_to_f64(order.amount, base_decimal),
        canceled_amount: u256_to_f64(order.canceled_amount, base_decimal),
        matched_amount: u256_to_f64(order.matched_amount, base_decimal),
        price: u256_to_f64(order.price, quote_decimal),
        average_price,
        total_volume: u256_to_f64(total_volume, quote_decimal),
        side: order.side.clone(),
        status: order.status.as_str().to_string(),
        trades: trades2,
        created_at: time2unix(order.created_at.clone()),
    }
}
