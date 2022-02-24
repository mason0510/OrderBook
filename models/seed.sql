insert into chemix_markets (id,base_token_address,base_token_symbol,base_front_decimal,base_contract_decimal,quote_token_address,quote_token_symbol,quote_front_decimal,quote_contract_decimal,online,up_at,down_at,created_at) values ('BTC-USDT', '0x3e1A99f4Ebdec4F6Da224D54a4a25b7B1445e1ea','BTC', 18,8,'0x707c73B9425276c0c0adcdd0d1178bB541792049','USDT', 15,8, true,NOW(),NOW() + '10 years',NOW());

--insert into chemix_markets (id,base_token_address,base_token_symbol,base_front_decimal,base_contract_decimal,quote_token_address,quote_token_symbol,base_contract_decimal,quote_contract_decimal,up_at,down_at,created_at) values ('AAA-CCC', '0x18D5034280703EA96e36a50f6178E43565eaDc67','AAA', 11,8,'0x7E62F80cA349DB398983E2Ee1434425f5B888f42','CCC',true,NOW(),NOW() + '10 years',NOW());

--insert into chemix_tokens (address, symbol, name,decimals,front_decimal) values ('0x18D5034280703EA96e36a50f6178E43565eaDc67', 'AAA','AAAA', 11,8);
--insert into chemix_tokens (address, symbol, name,decimals,front_decimal) values ('0x7E62F80cA349DB398983E2Ee1434425f5B888f42', 'BBB','BBBB', 22,8);
--insert into chemix_tokens (address, symbol, name,decimals,front_decimal) values ('0x7E62F80cA349DB398983E2Ee1434425f5B888f42', 'CCC','CCCC', 18,8);
