#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

impl Dimensions {
    pub fn new(width: f64, height: f64) -> Option<Self> {
        if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 {
            Some(Self { width, height })
        } else {
            None
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn rotated(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_dimensions() {
        let dimensions = Dimensions::new(100.0, 50.0);

        assert_eq!(
            dimensions,
            Some(Dimensions {
                width: 100.0,
                height: 50.0,
            })
        );
    }

    #[test]
    fn rejects_invalid_dimensions() {
        assert_eq!(Dimensions::new(0.0, 50.0), None);
        assert_eq!(Dimensions::new(100.0, 0.0), None);
        assert_eq!(Dimensions::new(-100.0, 50.0), None);
        assert_eq!(Dimensions::new(f64::NAN, 50.0), None);
        assert_eq!(Dimensions::new(f64::INFINITY, 50.0), None);
    }

    #[test]
    fn calculates_area() {
        let dimensions = Dimensions::new(100.0, 50.0).unwrap();

        assert_eq!(dimensions.area(), 5000.0);
    }

    #[test]
    fn rotates_dimensions() {
        let dimensions = Dimensions::new(100.0, 50.0).unwrap();

        assert_eq!(
            dimensions.rotated(),
            Dimensions {
                width: 50.0,
                height: 100.0,
            }
        );
    }
}
