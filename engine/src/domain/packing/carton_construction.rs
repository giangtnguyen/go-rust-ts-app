#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartonConstruction {
    top_allowance: f64,
    bottom_allowance: f64,
}

impl CartonConstruction {
    pub fn new(top_allowance: f64, bottom_allowance: f64) -> Option<Self> {
        if !top_allowance.is_finite()
            || !bottom_allowance.is_finite()
            || top_allowance < 0.0
            || bottom_allowance < 0.0
        {
            return None;
        }

        Some(Self {
            top_allowance,
            bottom_allowance,
        })
    }

    pub fn outer_height(&self, inner_height: f64) -> Option<f64> {
        if !inner_height.is_finite() || inner_height <= 0.0 {
            return None;
        }

        Some(inner_height + self.top_allowance + self.bottom_allowance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_carton_construction() {
        assert_eq!(
            CartonConstruction::new(10.0, 10.0),
            Some(CartonConstruction {
                top_allowance: 10.0,
                bottom_allowance: 10.0,
            })
        );
    }

    #[test]
    fn rejects_invalid_carton_allowances() {
        assert_eq!(CartonConstruction::new(-1.0, 10.0), None);
        assert_eq!(CartonConstruction::new(10.0, -1.0), None);
        assert_eq!(CartonConstruction::new(f64::NAN, 10.0), None);
        assert_eq!(CartonConstruction::new(10.0, f64::INFINITY), None);
    }

    #[test]
    fn calculates_outer_height_from_inner_height_and_flaps() {
        let construction = CartonConstruction::new(10.0, 10.0).unwrap();

        assert_eq!(construction.outer_height(214.0), Some(234.0));
    }

    #[test]
    fn rejects_invalid_inner_height() {
        let construction = CartonConstruction::new(10.0, 10.0).unwrap();

        assert_eq!(construction.outer_height(0.0), None);
        assert_eq!(construction.outer_height(-1.0), None);
        assert_eq!(construction.outer_height(f64::NAN), None);
        assert_eq!(construction.outer_height(f64::INFINITY), None);
    }
}
