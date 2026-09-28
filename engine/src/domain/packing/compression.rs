#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompressionMeasurement {
    pub pieces: u32,
    pub height: f64,
}

impl CompressionMeasurement {
    pub fn new(pieces: u32, height: f64) -> Option<Self> {
        if pieces == 0 {
            return None;
        }

        if !height.is_finite() || height <= 0.0 {
            return None;
        }

        Some(Self { pieces, height })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompressionModel {
    measurements: Vec<CompressionMeasurement>,
}

impl CompressionModel {
    pub fn from_measurements(measurements: Vec<CompressionMeasurement>) -> Option<Self> {
        if measurements.is_empty() {
            return None;
        }

        if measurements
            .iter()
            .any(|measurement| measurement.pieces == 0 || measurement.height <= 0.0)
        {
            return None;
        }

        Some(Self { measurements })
    }
    pub fn height_for(&self, pieces: u32) -> Option<f64> {
        if pieces == 0 {
            return None;
        }

        if let Some(measurement) = self
            .measurements
            .iter()
            .find(|measurement| measurement.pieces == pieces)
        {
            return Some(measurement.height);
        }

        let lower = self
            .measurements
            .iter()
            .filter(|measurement| measurement.pieces < pieces)
            .max_by_key(|measurement| measurement.pieces);

        let upper = self
            .measurements
            .iter()
            .filter(|measurement| measurement.pieces > pieces)
            .min_by_key(|measurement| measurement.pieces);

        match (lower, upper) {
            (Some(lower), Some(upper)) => {
                let piece_range = (upper.pieces - lower.pieces) as f64;
                let height_range = upper.height - lower.height;
                let position = (pieces - lower.pieces) as f64;

                Some(lower.height + height_range * position / piece_range)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_compression_measurement() {
        let measurement = CompressionMeasurement::new(1, 38.0);

        assert_eq!(
            measurement,
            Some(CompressionMeasurement {
                pieces: 1,
                height: 38.0,
            })
        );
    }

    #[test]
    fn rejects_invalid_compression_measurement() {
        assert_eq!(CompressionMeasurement::new(0, 38.0), None);
        assert_eq!(CompressionMeasurement::new(1, 0.0), None);
        assert_eq!(CompressionMeasurement::new(1, -1.0), None);
        assert_eq!(CompressionMeasurement::new(1, f64::NAN), None);
        assert_eq!(CompressionMeasurement::new(1, f64::INFINITY), None);
    }

    #[test]
    fn creates_compression_model_from_measurements() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ]);

        assert!(model.is_some());
    }

    #[test]
    fn rejects_empty_compression_model() {
        let model = CompressionModel::from_measurements(Vec::new());

        assert_eq!(model, None);
    }

    #[test]
    fn returns_measured_height_for_known_piece_count() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        assert_eq!(model.height_for(1), Some(38.0));
        assert_eq!(model.height_for(3), Some(102.0));
    }

    #[test]
    fn returns_none_for_unmeasured_piece_count() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();
        assert_eq!(model.height_for(2), Some(70.0));
        assert_eq!(model.height_for(4), None);
    }
    #[test]
    fn measured_height_has_priority_over_model_prediction() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(2, 69.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
            CompressionMeasurement::new(4, 132.0).unwrap(),
        ])
        .unwrap();

        assert_eq!(model.height_for(1), Some(38.0));
        assert_eq!(model.height_for(2), Some(69.0));
        assert_eq!(model.height_for(3), Some(102.0));
        assert_eq!(model.height_for(4), Some(132.0));
    }
    #[test]
    fn unmeasured_height_is_not_treated_as_ground_truth() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
            CompressionMeasurement::new(4, 132.0).unwrap(),
        ])
        .unwrap();

        assert_eq!(model.height_for(1), Some(38.0));
        assert_eq!(model.height_for(3), Some(102.0));
        assert_eq!(model.height_for(4), Some(132.0));
        assert_eq!(model.height_for(2), Some(70.0));
    }
    #[test]
    fn interpolates_height_between_measurements() {
        let model = CompressionModel::from_measurements(vec![
            CompressionMeasurement::new(1, 38.0).unwrap(),
            CompressionMeasurement::new(3, 102.0).unwrap(),
        ])
        .unwrap();

        assert_eq!(model.height_for(1), Some(38.0));
        assert_eq!(model.height_for(3), Some(102.0));

        // Linear interpolation:
        // 1 pc = 38 mm
        // 3 pcs = 102 mm
        // 2 pcs = 70 mm
        assert_eq!(model.height_for(2), Some(70.0));
    }
}
