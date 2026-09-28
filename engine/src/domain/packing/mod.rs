mod dimensions;
use dimensions::Dimensions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PackingRequest {
    pub container: Container,
    pub item: Item,
    pub allow_rotation: bool,
}

#[derive(Debug, Deserialize)]
pub struct Container {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize)]
pub struct PackingResult {
    pub max_items: u32,
    pub item_width: f64,
    pub item_height: f64,
    pub utilization: f64,
}
pub fn calculate(request: &PackingRequest) -> PackingResult {
    let container = match Dimensions::new(request.container.width, request.container.height) {
        Some(value) => value,
        None => {
            return PackingResult {
                max_items: 0,
                item_width: request.item.width,
                item_height: request.item.height,
                utilization: 0.0,
            };
        }
    };

    let item = match Dimensions::new(request.item.width, request.item.height) {
        Some(value) => value,
        None => {
            return PackingResult {
                max_items: 0,
                item_width: request.item.width,
                item_height: request.item.height,
                utilization: 0.0,
            };
        }
    };

    let item_width = item.width;
    let item_height = item.height;
    let (max_items, result_item) = choose_orientation(container, item, request.allow_rotation);

    let result_width = result_item.width;
    let result_height = result_item.height;
    let max_items = match max_items {
        Some(value) => value,
        None => {
            return PackingResult {
                max_items: 0,
                item_width,
                item_height,
                utilization: 0.0,
            };
        }
    };
    let used_area = max_items as f64 * item.area();
    let container_area = container.area();
    let utilization = (used_area / container_area).clamp(0.0, 1.0);

    PackingResult {
        max_items,
        item_width: result_width,
        item_height: result_height,
        utilization,
    }
}
fn is_better_capacity(candidate: Option<u32>, current: Option<u32>) -> bool {
    match (candidate, current) {
        (Some(candidate), Some(current)) => candidate > current,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => false,
    }
}
fn choose_orientation(
    container: Dimensions,
    item: Dimensions,
    allow_rotation: bool,
) -> (Option<u32>, Dimensions) {
    let normal = calculate_orientation(container.width, container.height, item.width, item.height);
    if !allow_rotation {
        return (normal, item);
    }
    let rotated_item = item.rotated();
    let rotated = calculate_orientation(
        container.width,
        container.height,
        rotated_item.width,
        rotated_item.height,
    );

    if is_better_capacity(rotated, normal) {
        (rotated, rotated_item)
    } else {
        (normal, item)
    }
}
fn calculate_orientation(
    container_width: f64,
    container_height: f64,
    item_width: f64,
    item_height: f64,
) -> Option<u32> {
    if ![container_width, container_height, item_width, item_height]
        .iter()
        .all(|value| value.is_finite() && *value > 0.0)
    {
        return Some(0);
    }

    let columns = (container_width / item_width).floor();
    let rows = (container_height / item_height).floor();

    let count = columns * rows;

    if !count.is_finite() || count > u32::MAX as f64 {
        return None;
    }

    Some(count as u32)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_basic_packing() {
        let request = PackingRequest {
            container: Container {
                width: 100.0,
                height: 60.0,
            },
            item: Item {
                width: 25.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 12);
        assert_eq!(result.item_width, 25.0);
        assert_eq!(result.item_height, 20.0);
        assert_eq!(result.utilization, 1.0);
    }

    #[test]
    fn floors_partial_fit() {
        let request = PackingRequest {
            container: Container {
                width: 100.0,
                height: 60.0,
            },
            item: Item {
                width: 30.0,
                height: 20.0,
            },
            allow_rotation: false,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 9);
    }

    #[test]
    fn rotation_can_change_result() {
        let request = PackingRequest {
            container: Container {
                width: 50.0,
                height: 100.0,
            },
            item: Item {
                width: 60.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);
        assert_eq!(result.max_items, 2);
        assert_eq!(result.item_width, 20.0);
        assert_eq!(result.item_height, 60.0);
    }
    #[test]
    fn rotation_tie_keeps_original_orientation() {
        let request = PackingRequest {
            container: Container {
                width: 100.0,
                height: 100.0,
            },
            item: Item {
                width: 50.0,
                height: 50.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 4);
        assert_eq!(result.item_width, 50.0);
        assert_eq!(result.item_height, 50.0);
    }
    #[test]
    fn invalid_dimensions_return_zero() {
        let request = PackingRequest {
            container: Container {
                width: 0.0,
                height: 60.0,
            },
            item: Item {
                width: 25.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
    }
    #[test]
    fn rotation_disabled_keeps_original_orientation() {
        let request = PackingRequest {
            container: Container {
                width: 50.0,
                height: 100.0,
            },
            item: Item {
                width: 60.0,
                height: 20.0,
            },
            allow_rotation: false,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
        assert_eq!(result.item_width, 60.0);
        assert_eq!(result.item_height, 20.0);
    }
    #[test]
    fn calculates_utilization() {
        let request = PackingRequest {
            container: Container {
                width: 100.0,
                height: 60.0,
            },
            item: Item {
                width: 30.0,
                height: 20.0,
            },
            allow_rotation: false,
        };
        let result = calculate(&request);
        assert_eq!(result.max_items, 9);
        assert!((result.utilization - 0.9).abs() < 1e-10);
    }
    #[test]
    fn nan_dimensions_return_zero() {
        let request = PackingRequest {
            container: Container {
                width: f64::NAN,
                height: 60.0,
            },
            item: Item {
                width: 25.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
        assert_eq!(result.utilization, 0.0);
    }

    #[test]
    fn infinite_dimensions_return_zero() {
        let request = PackingRequest {
            container: Container {
                width: f64::INFINITY,
                height: 60.0,
            },
            item: Item {
                width: 25.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
        assert_eq!(result.utilization, 0.0);
    }

    #[test]
    fn negative_dimensions_return_zero() {
        let request = PackingRequest {
            container: Container {
                width: 100.0,
                height: 60.0,
            },
            item: Item {
                width: -25.0,
                height: 20.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
        assert_eq!(result.utilization, 0.0);
    }
    #[test]
    fn extremely_large_capacity_returns_zero() {
        let request = PackingRequest {
            container: Container {
                width: 1.0e20,
                height: 1.0e20,
            },
            item: Item {
                width: 1.0,
                height: 1.0,
            },
            allow_rotation: false,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 0);
        assert_eq!(result.utilization, 0.0);
    }
    #[test]
    fn valid_capacity_beats_overflow() {
        assert!(is_better_capacity(Some(10), None));
        assert!(!is_better_capacity(None, Some(10)));
        assert!(!is_better_capacity(None, None));
    }
    #[test]
    fn capacity_at_u32_max_is_valid() {
        let result = calculate_orientation(u32::MAX as f64, 1.0, 1.0, 1.0);

        assert_eq!(result, Some(u32::MAX));
    }
    #[test]
    fn capacity_above_u32_max_returns_none() {
        let result = calculate_orientation(u32::MAX as f64 + 1.0, 1.0, 1.0, 1.0);

        assert_eq!(result, None);
    }
    #[test]
    fn rotation_selects_better_capacity() {
        let request = PackingRequest {
            container: Container {
                width: 10.0,
                height: 7.0,
            },
            item: Item {
                width: 6.0,
                height: 4.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 2);
        assert_eq!(result.item_width, 4.0);
        assert_eq!(result.item_height, 6.0);
    }
}
