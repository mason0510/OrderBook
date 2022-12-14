extern crate rustc_serialize;

//#[derive(Serialize)]
use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct MarketInfoPO {
    pub id: String,
    pub base_token_address: String,
    pub base_token_symbol: String,
    pub base_contract_decimal: u32,
    pub base_front_decimal: u32,
    pub quote_token_address: String,
    pub quote_token_symbol: String,
    pub quote_contract_decimal: u32,
    pub quote_front_decimal: u32,
}

pub fn list_markets() -> Vec<MarketInfoPO> {
    let sql = "select id,base_token_address,base_token_symbol,base_contract_decimal,\
    base_front_decimal,quote_token_address,quote_token_symbol,quote_contract_decimal,\
    quote_front_decimal from chemix_markets where online=true";

    let mut markets: Vec<MarketInfoPO> = Vec::new();
    let rows = crate::query(sql).unwrap();
    for row in rows {
        let info = MarketInfoPO {
            id: row.get(0),
            base_token_address: row.get(1),
            base_token_symbol: row.get(2),
            base_contract_decimal: row.get::<usize, i32>(3) as u32,
            base_front_decimal: row.get::<usize, i32>(4) as u32,
            quote_token_address: row.get(5),
            quote_token_symbol: row.get(6),
            quote_contract_decimal: row.get::<usize, i32>(7) as u32,
            quote_front_decimal: row.get::<usize, i32>(8) as u32,
        };
        markets.push(info);
    }
    markets
}

pub fn get_markets(id: &str) -> Option<MarketInfoPO> {
    let sql = format!(
        "select id,base_token_address,base_token_symbol,base_contract_decimal,\
    base_front_decimal,quote_token_address,quote_token_symbol,quote_contract_decimal,\
    quote_front_decimal from chemix_markets where online=true and id=\'{}\'",
        id
    );
    let rows = crate::query(sql.as_str()).unwrap();
    if rows.is_empty() {
        return None;
    }
    info!("get_markets: raw sql {}", sql);
    //id只有一个
    //rows[0].get::<usize, i32>(2) as u32
    Some(MarketInfoPO {
        id: rows[0].get(0),
        base_token_address: rows[0].get(1),
        base_token_symbol: rows[0].get(2),
        base_contract_decimal: rows[0].get::<usize, i32>(3) as u32,
        base_front_decimal: rows[0].get::<usize, i32>(4) as u32,
        quote_token_address: rows[0].get(5),
        quote_token_symbol: rows[0].get(6),
        quote_contract_decimal: rows[0].get::<usize, i32>(7) as u32,
        quote_front_decimal: rows[0].get::<usize, i32>(8) as u32,
    })
}
