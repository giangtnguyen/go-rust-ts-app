use super::carton_construction::CartonConstruction;
use super::layer_stack::LayerStack;
use super::safety::SafetyAllowance;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartonHeightResult {
    pub compressed_height: f64,
    pub inner_height: f64,
    pub outer_height: f64,
}

pub struct CartonHeightEngine {
    safety: SafetyAllowance,
    construction: CartonConstruction,
}

impl CartonHeightEngine {
    pub fn new(safety: SafetyAllowance, construction: CartonConstruction) -> Self {
        Self {
            safety,
            construction,
        }
    }

    pub fn calculate(&self, stack: &LayerStack) -> Option<CartonHeightResult> {
        let compressed_height = stack.required_height()?;
        let inner_height = self.safety.apply(compressed_height)?;
        let outer_height = self.construction.outer_height(inner_height)?;

        Some(CartonHeightResult {
            compressed_height,
            inner_height,
            outer_height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::compression::{CompressionMeasurement, CompressionModel};
    use super::*;

    #[test]
    fn calculates_inner_and_outer_height() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::new(vec![3, 3], compression);

        let safety = SafetyAllowance::new(10.0).unwrap();
        let construction = CartonConstruction::new(10.0, 10.0).unwrap();

        let engine = CartonHeightEngine::new(safety, construction);

        assert_eq!(
            engine.calculate(&stack),
            Some(CartonHeightResult {
                compressed_height: 204.0,
                inner_height: 214.0,
                outer_height: 234.0,
            })
        );
    }
}
