/// A simple BigNumber implementation using scientific notation
/// for incremental/idle games, with suffixes for thousands and millions,
/// configurable precision and trimming of trailing zeros.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BigNumber {
    pub mantissa: f64,
    pub exponent: i32,
    /**
     * Number of decimal places to show when representing this number with to_string(). It does not modify the underlying number's precision.
     */
    pub decimals: u8,
}

impl BigNumber {
    pub fn new(mantissa: f64, exponent: i32, decimals: u8) -> Self {
        let mut m = mantissa;
        let mut e = exponent;
        while m >= 10.0 && e < i32::MAX {
            m /= 10.0;
            e += 1;
        }
        while m < 1.0 && m != 0.0 && e > i32::MIN {
            m *= 10.0;
            e -= 1;
        }
        BigNumber { mantissa: m, exponent: e, decimals }
    }
    pub fn zero() -> Self {
        let mut m = 0.0;
        let mut e = 1;
        BigNumber { mantissa: m, exponent: e, decimals: 2 }
    }
    pub fn one() -> Self {
        let mut m = 1.0;
        let mut e = 0;
        BigNumber { mantissa: m, exponent: e, decimals: 2 }
    }

    pub fn add(self, other: BigNumber) -> BigNumber {
        if self.exponent == other.exponent {
            return BigNumber::new(self.mantissa + other.mantissa, self.exponent, self.decimals);
        }

        let (high, low) = if self.exponent > other.exponent {
            (self, other)
        } else {
            (other, self)
        };

        let diff = high.exponent - low.exponent;

        if diff > 308 { // past this number we do not really care about the lowest; we are in wild territory
            return high;
        }

        let scaled_low = low.mantissa / 10f64.powi(diff);
        let result_mantissa = high.mantissa + scaled_low;

        BigNumber::new(result_mantissa, high.exponent, self.decimals)
    }

    pub fn sub(self, other: BigNumber) -> BigNumber {
        if self.exponent == other.exponent {
            BigNumber::new(self.mantissa - other.mantissa, self.exponent, self.decimals)
        } else if self.exponent > other.exponent {
            BigNumber::new(
                self.mantissa - other.mantissa / 10f64.powi(self.exponent - other.exponent),
                self.exponent,
                self.decimals
            )
        } else {
            BigNumber::new(
                self.mantissa / 10f64.powi(other.exponent - self.exponent) - other.mantissa,
                other.exponent,
                self.decimals
            )
        }
    }

    pub fn mul(self, other: BigNumber) -> BigNumber {
        BigNumber::new(self.mantissa * other.mantissa, self.exponent + other.exponent, self.decimals)
    }

    pub fn div(self, other: BigNumber) -> BigNumber {
        if other.mantissa == 0.0 {
            panic!("Division by zero");
        }

        let new_mantissa = self.mantissa / other.mantissa;
        let new_exponent = self.exponent - other.exponent;

        BigNumber::new(new_mantissa, new_exponent, self.decimals)
    }

    pub fn to_string(&self) -> String {
        self.to_string_with_precision(self.decimals as usize)
    }

    pub fn to_string_with_precision(&self, precision: usize) -> String {
        if self.mantissa == 0.0 {
            return "0".to_string();
        }

        if self.exponent > 12 || self.mantissa.abs() >= 10.0 {
            let mut mantissa_str = format!("{:.*}", precision, self.mantissa.abs());
            if mantissa_str.contains('.') {
                mantissa_str = mantissa_str.trim_end_matches('0').trim_end_matches('.').to_string();
            }
            if self.mantissa < 0.0 {
                mantissa_str = format!("-{}", mantissa_str);
            }
            return format!("{}e{}", mantissa_str, self.exponent);
        }

        let val = self.mantissa * 10f64.powi(self.exponent);

        let (scaled_val, suffix) = if val.abs() < 1e3 {
            (val, "")
        } else if val.abs() < 1e6 {
            (val / 1e3, "K")
        } else if val.abs() < 1e9 {
            (val / 1e6, "M")
        } else if val.abs() < 1e12 {
            (val / 1e9, "B")
        } else {
            let mut mantissa_str = format!("{:.*}", precision, self.mantissa.abs());
            if mantissa_str.contains('.') {
                mantissa_str = mantissa_str.trim_end_matches('0').trim_end_matches('.').to_string();
            }
            if self.mantissa < 0.0 {
                mantissa_str = format!("-{}", mantissa_str);
            }
            return format!("{}e{}", mantissa_str, self.exponent);
        };

        let mut s = format!("{:.*}", precision, scaled_val);
        if s.contains('.') {
            s = s.trim_end_matches('0').trim_end_matches('.').to_string();
        }

        s + suffix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = BigNumber::new(1.0, 10, 2);
        let b = BigNumber::new(2.0, 10, 2);
        assert_eq!(a.add(b).to_string(), "30B");
    }

    #[test]
    fn test_subtraction() {
        let a = BigNumber::new(5.0, 10, 2);
        let b = BigNumber::new(3.0, 10, 2);
        assert_eq!(a.sub(b).to_string(), "20B");
    }

    #[test]
    fn test_multiplication() {
        let a = BigNumber::new(2.0, 5, 2);
        let b = BigNumber::new(3.0, 6, 2);
        assert_eq!(a.mul(b).to_string(), "600B");
    }

    #[test]
    fn test_division() {
        let a = BigNumber::new(6.0, 10, 2);
        let b = BigNumber::new(2.0, 5, 2);
        assert_eq!(a.div(b).to_string(), "300K");
    }

    #[test]
    fn test_small_values() {
        let a = BigNumber::new(5.0, 0, 2);
        assert_eq!(a.to_string(), "5");
        let b = BigNumber::new(1.23456, 0, 2);
        assert_eq!(b.to_string(), "1.23");
        assert_eq!(b.to_string_with_precision(1), "1.2");
    }

    #[test]
    fn test_thousands_and_millions() {
        let k = BigNumber::new(1.234, 3, 2);
        assert_eq!(k.to_string(), "1.23K");
        let m = BigNumber::new(5.0, 6, 2);
        assert_eq!(m.to_string(), "5M");
        assert_eq!(k.to_string_with_precision(2), "1.23K");
        let t = BigNumber::new(300.0, 3, 2); // 300K
        assert_eq!(t.to_string(), "300K");
    }

    #[test]
    fn test_scientific_after_threshold() {
        let c = BigNumber::new(1.0, 9, 2);
        assert_eq!(c.to_string(), "1B");
        assert_eq!(c.to_string_with_precision(2), "1B");
    }

    #[test]
    fn test_zero_helper() {
        let c = BigNumber::zero();
        assert_eq!(c.to_string(), "0");
    }

    #[test]
    fn test_one_helper() {
        let c = BigNumber::one();
        assert_eq!(c.to_string(), "1");
    }
    #[test]
    fn test_overflow_creation() {
        let max_i32 = i32::MAX;
        let max_f64 = f64::MAX;
        let _one = BigNumber::new(max_f64, max_i32, 2);
    }
    #[test]
    fn test_underflow_creation() {
        let max_i32 = i32::MIN;
        let max_f64 = 0.1;
        let _one = BigNumber::new(max_f64, max_i32, 2);
    }
    #[test]
    fn test_max_representable_number() {
        let big = BigNumber::new(f64::MAX, i32::MAX, 2);
        let s = big.to_string_with_precision(2);
        assert_eq!(s, "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368e2147483647");
    }
    #[test]
    fn test_big_representable_number() {
        let big = BigNumber::new(f64::MAX-1.0, i32::MAX-1, 2);
        let s = big.to_string_with_precision(2);
        assert_eq!(s, "17976931348623157580412819756850388593900235011794141176754562789180111453639664485361928830517704263393537268510363518759043843737070229269956251768752166883397940628862983287625967246810352023792017211936260189893797509826303293149283469713429932049693599732425511693654044437030940398714664210204414967808e2147483647");
    }
}
