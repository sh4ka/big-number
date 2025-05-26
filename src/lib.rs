/// A simple BigNumber implementation using scientific notation
/// for incremental/idle games.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BigNumber {
    pub mantissa: f64,
    pub exponent: i32,
}

impl BigNumber {
    pub fn new(mantissa: f64, exponent: i32) -> Self {
        let mut m = mantissa;
        let mut e = exponent;

        while m >= 10.0 {
            m /= 10.0;
            e += 1;
        }
        while m < 1.0 && m != 0.0 {
            m *= 10.0;
            e -= 1;
        }

        BigNumber { mantissa: m, exponent: e }
    }

    pub fn zero() -> Self {
        BigNumber { mantissa: 0.0, exponent: 0 }
    }

    pub fn one() -> Self {
        BigNumber { mantissa: 1.0, exponent: 0 }
    }

    pub fn add(self, other: BigNumber) -> BigNumber {
        if self.mantissa == 0.0 {
            return other;
        }
        if other.mantissa == 0.0 {
            return self;
        }

        let (larger, smaller) = if self.exponent >= other.exponent {
            (self, other)
        } else {
            (other, self)
        };

        let exponent_diff = (larger.exponent - smaller.exponent) as f64;

        if exponent_diff > 15.0 {
            return larger;
        }

        let added_mantissa = larger.mantissa + smaller.mantissa * 10f64.powf(-exponent_diff);
        BigNumber::new(added_mantissa, larger.exponent)
    }

    pub fn sub(self, other: BigNumber) -> BigNumber {
        if other.mantissa == 0.0 {
            return self;
        }
        if self.mantissa == 0.0 {
            return BigNumber::new(-other.mantissa, other.exponent);
        }

        let (larger, smaller, sign) = if self.exponent >= other.exponent {
            (self, other, 1.0)
        } else {
            (other, self, -1.0)
        };

        let exponent_diff = (larger.exponent - smaller.exponent) as f64;

        if exponent_diff > 15.0 {
            return self;
        }

        let result_mantissa = larger.mantissa - sign * smaller.mantissa * 10f64.powf(-exponent_diff);
        BigNumber::new(result_mantissa, larger.exponent)
    }

    pub fn mul(self, other: BigNumber) -> BigNumber {
        BigNumber::new(self.mantissa * other.mantissa, self.exponent + other.exponent)
    }

    pub fn div(self, other: BigNumber) -> BigNumber {
        if other.mantissa == 0.0 {
            panic!("Division by zero in BigNumber::div");
        }
        BigNumber::new(self.mantissa / other.mantissa, self.exponent - other.exponent)
    }

    pub fn to_string(&self) -> String {
        if self.mantissa == 0.0 {
            "0".to_string()
        } else {
            format!("{:.3}e{}", self.mantissa, self.exponent)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = BigNumber::new(1.0, 10);
        let b = BigNumber::new(2.0, 10);
        assert_eq!(a.add(b).to_string(), "3.000e10");
    }

    #[test]
    fn test_multiplication() {
        let a = BigNumber::new(2.0, 10);
        let b = BigNumber::new(3.0, 5);
        assert_eq!(a.mul(b).to_string(), "6.000e15");
    }

    #[test]
    fn test_subtraction() {
        let a = BigNumber::new(5.0, 10);
        let b = BigNumber::new(3.0, 10);
        assert_eq!(a.sub(b).to_string(), "2.000e10");
    }

    #[test]
    fn test_division() {
        let a = BigNumber::new(6.0, 10);
        let b = BigNumber::new(2.0, 5);
        assert_eq!(a.div(b).to_string(), "3.000e5");
    }
} 
