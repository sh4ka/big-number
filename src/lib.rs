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
            BigNumber::new(self.mantissa + other.mantissa, self.exponent, self.decimals)
        } else if self.exponent > other.exponent {
            BigNumber::new(
                self.mantissa + other.mantissa / 10f64.powi(self.exponent - other.exponent),
                self.exponent,
                self.decimals
            )
        } else {
            BigNumber::new(
                self.mantissa / 10f64.powi(other.exponent - self.exponent) + other.mantissa,
                other.exponent,
                self.decimals
            )
        }
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
        BigNumber::new(self.mantissa / other.mantissa, self.exponent - other.exponent, self.decimals)
    }

    pub fn to_string(&self) -> String {
        self.to_string_with_precision(self.decimals as usize)
    }

    pub fn to_string_with_precision(&self, precision: usize) -> String {
        if self.mantissa == 0.0 {
            return "0".to_string();
        }
        let val = self.mantissa * 10f64.powi(self.exponent);
        // Helper: formatea y recorta ceros y punto final
        let format_trim = |num: f64| {
            let s = format!("{:.*}", precision, num);
            if let Some(_) = s.find('.') {
                s.trim_end_matches('0').trim_end_matches('.').to_string()
            } else {
                s
            }
        };

        if val < 1e3 {
            format_trim(val)
        } else if val < 1e6 {
            let v = val / 1e3;
            format!("{}K", format_trim(v))
        } else if val < 1e9 {
            let v = val / 1e6;
            format!("{}M", format_trim(v))
        } else {
            let m_str = format_trim(self.mantissa);
            format!("{}e{}", m_str, self.exponent)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = BigNumber::new(1.0, 10, 2);
        let b = BigNumber::new(2.0, 10, 2);
        assert_eq!(a.add(b).to_string(), "3e10");
    }

    #[test]
    fn test_subtraction() {
        let a = BigNumber::new(5.0, 10, 2);
        let b = BigNumber::new(3.0, 10, 2);
        assert_eq!(a.sub(b).to_string(), "2e10");
    }

    #[test]
    fn test_multiplication() {
        let a = BigNumber::new(2.0, 5, 2);
        let b = BigNumber::new(3.0, 6, 2);
        assert_eq!(a.mul(b).to_string(), "6e11");
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
        assert_eq!(c.to_string(), "1e9");
        assert_eq!(c.to_string_with_precision(2), "1e9");
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
}
