use rust_decimal::Decimal;
use ethers_core::types::U256;
use rust_decimal::prelude::ToPrimitive;
use std::ops::Div;

///U256 default value
pub const U256_ZERO: U256 = U256([0; 4]);

///u256 power calculate
#[macro_export]
macro_rules! u256_power {
    ( $a:expr,$b:expr) => {{
        U256::from($a).pow(U256::from($b))
    }};
}

/// Digital processing
pub trait MathOperation {
    ///
    fn to_fix(&self, precision: u32) -> f64;
    ///
    fn to_nano(&self) -> u64;
}

//todo: 再次检查丢精度问题
impl MathOperation for f64 {
    /// Keep decimal significant digits
    fn to_fix(&self, precision: u32) -> f64 {
        let times = 10_u32.pow(precision);
        let number = self * times as f64;
        let real_number = number.round();
        let decimal_number = Decimal::new(real_number as i64, precision);
        decimal_number.to_f64().unwrap()
    }

    /// A billion times larger
    fn to_nano(&self) -> u64 {
        let test1 = *self * 100_000_000.00f64;
        //test1.to_fix(8) as u64
        test1.floor() as u64
    }
}

//fixme:考虑用其他库,硬编码精度为8位，decimal超过37的话仍溢出，目前业务不会触发,
/// f64的有效精度为16位,当前业务做一定的取舍，总账对上就行
pub fn u256_to_f64(ori: U256, decimal: u32) -> f64 {
    let decimal_value = U256::from(10u32).pow(U256::from(decimal - 8));
    let dist_int = ori.div(decimal_value);
    let mut dist = Decimal::from(dist_int.as_u128());
    let _set_res = dist.set_scale(8).unwrap();
    dist.to_f64().unwrap()
}

#[cfg(test)]
mod tests {
    use ethers_core::types::U256;
    use crate::utils::math::u256_to_f64;

    #[test]
    fn test_u256_to_f64() {
        //let a = U256::from_str_radix("123456789012345178901234567890012345678901234567890",10).unwrap();
        let a = U256::from_str_radix("1234567890123451789012345678912", 10).unwrap();
        let res1 = u256_to_f64(a, 22);
        assert_eq!(res1, 123456789.01234517);
        let a = U256::from_str_radix("1", 10).unwrap();
        let res2 = u256_to_f64(a, 22);
        assert_eq!(res2, 0.0);
    }
}
