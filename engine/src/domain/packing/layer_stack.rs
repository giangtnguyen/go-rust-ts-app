use super::PackingLayout;
use super::compression::CompressionModel;

pub struct LayerStack {
    layers: Vec<u32>,
    compression: CompressionModel,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerCapacity {
    pub pieces: u32,
}
impl LayerStack {
    pub fn new(layers: Vec<u32>, compression: CompressionModel) -> Self {
        Self {
            layers,
            compression,
        }
    }
    pub fn from_total_pieces(
        total_pieces: u32,
        pieces_per_layer: u32,
        compression: CompressionModel,
    ) -> Option<Self> {
        if total_pieces == 0 || pieces_per_layer == 0 {
            return None;
        }

        let mut layers = Vec::new();
        let mut remaining = total_pieces;

        while remaining > 0 {
            let pieces = remaining.min(pieces_per_layer);
            layers.push(pieces);
            remaining -= pieces;
        }

        Some(Self::new(layers, compression))
    }
    pub fn from_layout(
        total_pieces: u32,
        layout: &PackingLayout,
        compression: CompressionModel,
    ) -> Option<Self> {
        Self::from_total_pieces(total_pieces, layout.item_count(), compression)
    }
    pub fn required_height(&self) -> Option<f64> {
        let mut total = 0.0;

        for &pieces in &self.layers {
            let height = self.compression.height_for(pieces)?;
            total += height;
        }

        Some(total)
    }
}

#[cfg(test)]
mod tests {
    use super::super::compression::CompressionMeasurement;
    use super::super::dimensions::Dimensions;
    use super::super::safety::SafetyAllowance;
    use super::*;

    #[test]
    fn calculates_total_height_from_layers() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::new(vec![3, 3], compression);

        assert_eq!(stack.required_height(), Some(204.0));
    }

    #[test]
    fn calculates_height_for_layers_with_different_piece_counts() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::new(vec![3, 2], compression);

        assert_eq!(stack.required_height(), Some(172.0));
    }
    #[test]
    fn returns_none_when_layer_height_cannot_be_determined() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::new(vec![3, 6], compression);

        assert_eq!(stack.required_height(), None);
    }
    #[test]
    fn calculates_required_height_with_safety_allowance() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::new(vec![3, 3], compression);
        let safety = SafetyAllowance::new(10.0).unwrap();

        let compressed_height = stack.required_height().unwrap();
        let required_height = safety.apply(compressed_height);

        assert_eq!(required_height, Some(214.0));
    }
    #[test]
    fn builds_layers_from_total_pieces_and_capacity() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let stack = LayerStack::from_total_pieces(7, 3, compression).unwrap();

        assert_eq!(stack.required_height(), Some(242.0));
    }
    #[test]
    fn rejects_zero_total_pieces() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
        ])
        .unwrap();

        assert!(LayerStack::from_total_pieces(0, 3, compression).is_none());
    }

    #[test]
    fn rejects_zero_pieces_per_layer() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
        ])
        .unwrap();

        assert!(LayerStack::from_total_pieces(7, 0, compression).is_none());
    }
    #[test]
    fn builds_layers_from_layer_capacity() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let capacity = 3;

        let stack = LayerStack::from_total_pieces(7, capacity, compression).unwrap();

        assert_eq!(stack.required_height(), Some(242.0));
    }
    #[test]
    fn builds_layers_from_packing_layout() {
        let compression = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        let container = Dimensions::new(5.0, 5.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = super::super::calculate_mixed_capacity(container, item);

        let layout = super::super::PackingLayout::from_placements(placements);

        let stack = LayerStack::from_layout(7, &layout, compression).unwrap();

        assert_eq!(stack.required_height(), Some(242.0));
    }
}
