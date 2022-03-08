use ethers_core::types::{I256, U256};
use std::collections::HashMap;
use std::ops::{Add, Sub};

use serde::Serialize;

use crate::{AddBook, AddBook2};
//use ethers::{prelude::*,types::{U256}};

use chemix_chain::chemix::storage::CancelOrderState2;

use chemix_models::order::IdOrIndex::Index;
use chemix_models::order::{get_order, BookOrder, OrderInfo};
use chemix_models::trade::TradeInfo;
use common::types::order::{Side, Status as OrderStatus};

use common::types::order::Side as OrderSide;
use common::utils::math::u256_to_f64;
use common::utils::time::get_unix_time;

#[derive(RustcEncodable, Clone, Serialize)]
pub struct EventOrder {
    pub market_id: String,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
}

pub fn match_order(
    mut taker_order: &mut OrderInfo,
    trades: &mut Vec<TradeInfo>,
    marker_reduced_orders: &mut HashMap<String, U256>,
) -> U256 {
    let u256_zero = U256::from(0i32);
    let book = &mut crate::BOOK.lock().unwrap();
    let mut total_matched_amount = U256::from(0i32);
    'marker_orders: loop {
        match &taker_order.side {
            OrderSide::Buy => {
                //不能吃单的直接挂单
                if book.sell.is_empty() || taker_order.price < book.sell.first().unwrap().price
                {
                    //insert this order by compare price and created_at
                    //fixme:tmpcode,优化，还有时间排序的问题
                    book.buy.push(BookOrder{
                        id: taker_order.id.clone(),
                        account: taker_order.account.clone(),
                        side: taker_order.side.clone(),
                        price: taker_order.price,
                        amount: taker_order.available_amount,
                        created_at: get_unix_time()
                    });
                    book.buy
                        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                    book.buy.reverse();
                    break 'marker_orders;
                } else {
                    let mut marker_order = book.sell[0].clone();
                    let matched_amount = std::cmp::min(taker_order.available_amount, marker_order.amount);

                    trades.push(TradeInfo::new(
                        crate::MARKET.id.clone(),
                        taker_order.account.clone(),
                        marker_order.account.clone(),
                        marker_order.price.clone(),
                        matched_amount.clone(),
                        taker_order.side.clone(),
                        marker_order.id.clone(),
                        taker_order.id.clone(),
                    ));

                    //get marker_order change value
                    marker_reduced_orders.insert(marker_order.id.clone(), matched_amount);
                    marker_order.amount = marker_order.amount.sub(matched_amount);
                    //todo: 不在去减，用total_matched_amount 判断
                    taker_order.available_amount = taker_order.available_amount.sub(matched_amount);
                    total_matched_amount = total_matched_amount.add(matched_amount);
                    if marker_order.amount != u256_zero && taker_order.available_amount == u256_zero {
                        book.sell[0] = marker_order;
                        break 'marker_orders;
                    } else if marker_order.amount == u256_zero
                        && taker_order.available_amount != u256_zero
                    {
                        book.sell.remove(0);
                    } else if marker_order.amount == u256_zero
                        && taker_order.available_amount == u256_zero
                    {
                        book.sell.remove(0);
                        break 'marker_orders;
                    } else {
                        unreachable!()
                    }
                }
            }
            OrderSide::Sell => {
                if book.buy.is_empty() || taker_order.price > book.buy.first().unwrap().price {
                    //insert this order by compare price and created_at
                    //fixme:tmpcode,优化，还有时间的问题
                    book.sell.push(BookOrder{
                        id: taker_order.id.clone(),
                        account: taker_order.account.clone(),
                        side: taker_order.side.clone(),
                        price: taker_order.price,
                        amount: taker_order.available_amount,
                        created_at: get_unix_time()
                    });
                    book.sell
                        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                    break 'marker_orders;
                } else {
                    let mut marker_order = book.buy[0].clone();
                    let matched_amount = std::cmp::min(taker_order.available_amount, marker_order.amount);

                    trades.push(TradeInfo::new(
                        crate::MARKET.id.clone(),
                        taker_order.account.clone(),
                        marker_order.account.clone(),
                        marker_order.price.clone(),
                        matched_amount,
                        taker_order.side.clone(),
                        marker_order.id.clone(),
                        taker_order.id.clone(),
                    ));

                    //get change marker order
                    marker_reduced_orders.insert(marker_order.id.clone(), matched_amount);

                    marker_order.amount = marker_order.amount.sub(matched_amount);
                    taker_order.available_amount = taker_order.available_amount.sub(matched_amount);
                    total_matched_amount = total_matched_amount.add(matched_amount);
                    if marker_order.amount != u256_zero && taker_order.available_amount == u256_zero {
                        //todo: 本身此时也是在index0
                        book.buy[0] = marker_order;
                        break 'marker_orders;
                    } else if marker_order.amount == u256_zero
                        && taker_order.available_amount != u256_zero
                    {
                        book.buy.remove(0);
                    } else if marker_order.amount == u256_zero
                        && taker_order.available_amount == u256_zero
                    {
                        book.buy.remove(0);
                        break 'marker_orders;
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }

    info!("current book = {:?}", book);
    total_matched_amount
}

pub fn cancel(new_cancel_orders: Vec<CancelOrderState2>) -> Vec<CancelOrderState2> {
    let mut legal_orders = Vec::new();
    for new_cancel_order in new_cancel_orders {
        //todo: 处理异常
        let order = get_order(Index(new_cancel_order.order_index.as_u32())).unwrap();
        match order.status {
            OrderStatus::FullFilled => {
                warn!("Have already matched");
            }
            OrderStatus::PartialFilled => {
                //todo: side 处理
                match order.side {
                    OrderSide::Buy => {
                        crate::BOOK.lock().unwrap().buy.retain(|x| x.id != order.id);
                        legal_orders.push(new_cancel_order);
                    }
                    OrderSide::Sell => {
                        crate::BOOK
                            .lock()
                            .unwrap()
                            .sell
                            .retain(|x| x.id != order.id);
                        legal_orders.push(new_cancel_order);
                    }
                }
            }
            OrderStatus::Pending => match order.side {
                OrderSide::Buy => {
                    crate::BOOK.lock().unwrap().buy.retain(|x| x.id != order.id);
                    legal_orders.push(new_cancel_order);
                }
                OrderSide::Sell => {
                    crate::BOOK
                        .lock()
                        .unwrap()
                        .sell
                        .retain(|x| x.id != order.id);
                    legal_orders.push(new_cancel_order);
                }
            },
            OrderStatus::Canceled => {
                warn!("Have already Canceled");
            },
            OrderStatus::PreCanceled => {
                warn!("Have already Canceled");
            }
            OrderStatus::Abandoned => {
                todo!()
            }
        }
    }
    legal_orders
}

//根据未成交的订单生成深度数据
pub fn gen_depth_from_order(orders: Vec<OrderInfo>) -> HashMap<String,AddBook>{
    let mut raw_depth = AddBook2 {
        asks: HashMap::new(),
        bids: HashMap::new(),
    };

    for order in orders {
        match order.side {
            Side::Buy => {
                let stat = raw_depth
                    .bids
                    .entry(order.price)
                    .or_insert(I256::from(0i32));
                *stat += I256::from_raw(order.amount);
            }
            Side::Sell => {
                let stat = raw_depth
                    .asks
                    .entry(order.price)
                    .or_insert(I256::from(0i32));
                *stat += I256::from_raw(order.amount);
            }
        }
    }

    let base_decimal = crate::MARKET.base_contract_decimal as u32;
    let quote_decimal = crate::MARKET.quote_contract_decimal as u32;

    let asks = raw_depth
        .asks
        .iter()
        .map(|(x, y)| {
            let user_price = u256_to_f64(x.to_owned(), quote_decimal);
            let user_volume = if y < &I256::from(0u32) {
                u256_to_f64(y.abs().into_raw(), base_decimal) * -1.0f64
            } else {
                u256_to_f64(y.abs().into_raw(), base_decimal)
            };

            (user_price, user_volume)
        })
        .filter(|(p, v)| p != &0.0 && v != &0.0)
        .collect::<Vec<(f64, f64)>>();

    let bids = raw_depth
        .bids
        .iter()
        .map(|(x, y)| {
            let user_price = u256_to_f64(x.to_owned(), quote_decimal);
            let user_volume = if y < &I256::from(0u32) {
                u256_to_f64(y.abs().into_raw(), base_decimal) * -1.0f64
            } else {
                u256_to_f64(y.abs().into_raw(), base_decimal)
            };
            (user_price, user_volume)
        })
        .filter(|(p, v)| p != &0.0 && v != &0.0)
        .collect::<Vec<(f64, f64)>>();

    let mut market_add_depth = HashMap::new();
    market_add_depth.insert(crate::MARKET.id.clone(), AddBook { asks, bids});
    market_add_depth
}
