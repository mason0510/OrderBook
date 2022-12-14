extern crate rustc_serialize;

use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct TokenPO {
    pub symbol: String,
    pub name: String,
    pub address: String,
    pub front_decimals: i32,
    pub base_contract_decimal: i32,
    pub cvt_url: String,
    pub show_cvt: bool,
}

pub fn list_tokens() -> Vec<TokenPO> {
    let sql = "select symbol,name,address,front_decimals,\
    base_contract_decimal,cvt_url,show_cvt from chemix_tokens "
        .to_string();
    let rows = crate::query(sql.as_str()).unwrap();
    let mut tokens = Vec::new();
    info!("get_snapshot: raw sql {}", sql);
    for row in rows {
        tokens.push(TokenPO {
            symbol: row.get(0),
            name: row.get(1),
            address: row.get(2),
            front_decimals: row.get(3),
            base_contract_decimal: row.get(4),
            cvt_url: row.get(5),
            show_cvt: row.get(6),
        });
    }
    tokens
}

pub fn get_token(symbol: &str) -> Option<TokenPO> {
    let sql = format!(
        "select symbol,name,address,front_decimals,\
    base_contract_decimal,cvt_url,show_cvt from chemix_tokens where symbol='{}'",
        symbol
    );
    let rows = crate::query(sql.as_str()).unwrap();
    if rows.is_empty() {
        return None;
    }
    info!("get_snapshot: raw sql {}", sql);
    Some(TokenPO {
        symbol: rows[0].get(0),
        name: rows[0].get(1),
        address: rows[0].get(2),
        front_decimals: rows[0].get(3),
        base_contract_decimal: rows[0].get(4),
        cvt_url: rows[0].get(5),
        show_cvt: rows[0].get(6),
    })
}
