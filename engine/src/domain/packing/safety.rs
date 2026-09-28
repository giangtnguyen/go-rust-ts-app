#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SafetyAllowance {
    amount: f64,
}

impl SafetyAllowance {
    pub fn new(amount: f64) -> Option<Self> {
        if !amount.is_finite() || amount < 0.0 {
            return None;
        }

        Some(Self { amount })
    }

    pub fn apply(&self, compressed_height: f64) -> Option<f64> {
        if !compressed_height.is_finite() || compressed_height <= 0.0 {
            return None;
        }

        Some(compressed_height + self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_safety_allowance() {
        assert_eq!(
            SafetyAllowance::new(10.0),
            Some(SafetyAllowance { amount: 10.0 })
        );
    }

    #[test]
    fn rejects_invalid_safety_allowance() {
        assert_eq!(SafetyAllowance::new(-1.0), None);
        assert_eq!(SafetyAllowance::new(f64::NAN), None);
        assert_eq!(SafetyAllowance::new(f64::INFINITY), None);
    }

    #[test]
    fn adds_fixed_safety_allowance_to_compressed_height() {
        let safety = SafetyAllowance::new(10.0).unwrap();

        assert_eq!(safety.apply(204.0), Some(214.0));
    }

    #[test]
    fn rejects_invalid_compressed_height() {
        let safety = SafetyAllowance::new(10.0).unwrap();

        assert_eq!(safety.apply(0.0), None);
        assert_eq!(safety.apply(-1.0), None);
        assert_eq!(safety.apply(f64::NAN), None);
        assert_eq!(safety.apply(f64::INFINITY), None);
    }
}
