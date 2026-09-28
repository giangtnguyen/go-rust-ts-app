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
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Placement {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    fn contains(&self, item: Dimensions) -> bool {
        item.width <= self.width && item.height <= self.height
    }
}
fn placement_fits_container(placement: Placement, container: Dimensions) -> bool {
    placement.x >= 0.0
        && placement.y >= 0.0
        && placement.x + placement.width <= container.width
        && placement.y + placement.height <= container.height
}
fn placements_overlap(a: Placement, b: Placement) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}
fn split_free_rectangles(free_rect: Rect, item: Dimensions) -> Vec<Rect> {
    if !free_rect.contains(item) {
        return Vec::new();
    }

    let mut result = Vec::new();

    let remaining_width = free_rect.width - item.width;
    let remaining_height = free_rect.height - item.height;

    // Phần bên phải item
    if remaining_width > 0.0 {
        result.push(Rect {
            x: free_rect.x + item.width,
            y: free_rect.y,
            width: remaining_width,
            height: free_rect.height,
        });
    }

    // Phần phía dưới item
    if remaining_height > 0.0 {
        result.push(Rect {
            x: free_rect.x,
            y: free_rect.y + item.height,
            width: item.width,
            height: remaining_height,
        });
    }

    result
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

    let (uniform_capacity, uniform_item) =
        choose_orientation(container, item, request.allow_rotation);

    let mixed_placements = if request.allow_rotation {
        calculate_mixed_capacity(container, item)
    } else {
        Vec::new()
    };

    let mixed_capacity = if request.allow_rotation {
        Some(mixed_placements.len() as u32)
    } else {
        uniform_capacity
    };
    let (max_items, result_item) = match (uniform_capacity, mixed_capacity) {
        (Some(uniform), Some(mixed)) if mixed > uniform => {
            (mixed, Dimensions::new(item.width, item.height).unwrap())
        }
        (Some(uniform), _) => (uniform, uniform_item),
        (None, Some(mixed)) => (mixed, item),
        (None, None) => {
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
        item_width: result_item.width,
        item_height: result_item.height,
        utilization,
    }
}
fn calculate_mixed_capacity(container: Dimensions, item: Dimensions) -> Vec<Placement> {
    let orientations = [item, item.rotated()];

    let initial_free_rect = Rect {
        x: 0.0,
        y: 0.0,
        width: container.width,
        height: container.height,
    };

    let mut placements = Vec::new();
    let mut best_placements = Vec::new();

    search_mixed(
        vec![initial_free_rect],
        &orientations,
        &mut placements,
        &mut best_placements,
    );

    best_placements
}
fn search_mixed(
    free_rects: Vec<Rect>,
    orientations: &[Dimensions],
    placements: &mut Vec<Placement>,
    best_placements: &mut Vec<Placement>,
) {
    // Nếu nghiệm hiện tại tốt hơn nghiệm tốt nhất
    if placements.len() > best_placements.len() {
        *best_placements = placements.clone();
    }

    if free_rects.is_empty() {
        return;
    }

    for index in 0..free_rects.len() {
        let free_rect = free_rects[index];

        for &orientation in orientations {
            if !free_rect.contains(orientation) {
                continue;
            }

            let placement = Placement {
                x: free_rect.x,
                y: free_rect.y,
                width: orientation.width,
                height: orientation.height,
            };

            let mut next_free_rects = Vec::new();

            for (other_index, &other_rect) in free_rects.iter().enumerate() {
                if other_index != index {
                    next_free_rects.push(other_rect);
                }
            }

            next_free_rects.extend(split_free_rectangles(free_rect, orientation));

            placements.push(placement);

            search_mixed(next_free_rects, orientations, placements, best_placements);

            placements.pop();
        }
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
    #[test]
    fn mixed_orientation_can_exceed_uniform_orientation() {
        let request = PackingRequest {
            container: Container {
                width: 5.0,
                height: 5.0,
            },
            item: Item {
                width: 2.0,
                height: 3.0,
            },
            allow_rotation: true,
        };

        let result = calculate(&request);

        assert_eq!(result.max_items, 3);
    }
    #[test]
    fn splits_free_rectangle_after_placement() {
        let free_rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0,
        };

        let item = Dimensions::new(2.0, 3.0).unwrap();

        let result = split_free_rectangles(free_rect, item);

        assert_eq!(result.len(), 2);

        assert_eq!(
            result[0],
            Rect {
                x: 2.0,
                y: 0.0,
                width: 3.0,
                height: 5.0,
            }
        );

        assert_eq!(
            result[1],
            Rect {
                x: 0.0,
                y: 3.0,
                width: 2.0,
                height: 2.0,
            }
        );
    }
    #[test]
    fn splits_free_rectangle_with_rotated_item() {
        let free_rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0,
        };

        let item = Dimensions::new(3.0, 2.0).unwrap();

        let result = split_free_rectangles(free_rect, item);

        assert_eq!(result.len(), 2);

        assert_eq!(
            result[0],
            Rect {
                x: 3.0,
                y: 0.0,
                width: 2.0,
                height: 5.0,
            }
        );

        assert_eq!(
            result[1],
            Rect {
                x: 0.0,
                y: 2.0,
                width: 3.0,
                height: 3.0,
            }
        );
    }
    #[test]
    fn splitting_when_item_does_not_fit_returns_empty() {
        let free_rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 2.0,
            height: 2.0,
        };

        let item = Dimensions::new(2.0, 3.0).unwrap();

        let result = split_free_rectangles(free_rect, item);

        assert!(result.is_empty());
    }
    #[test]
    fn search_mixed_finds_three_items() {
        let container = Rect {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0,
        };

        let item = Dimensions::new(2.0, 3.0).unwrap();
        let orientations = [item, item.rotated()];
        let mut placements = Vec::new();
        let mut best_placements = Vec::new();
        search_mixed(
            vec![container],
            &orientations,
            &mut placements,
            &mut best_placements,
        );

        assert_eq!(best_placements.len(), 3);
    }
    #[test]
    fn search_mixed_returns_best_placements() {
        let container = Dimensions::new(5.0, 5.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let result = calculate_mixed_capacity(container, item);
        assert_eq!(result.len(), 3);

        for placement in &result {
            assert!(
                (placement.width == 2.0 && placement.height == 3.0)
                    || (placement.width == 3.0 && placement.height == 2.0)
            );
        }
    }
    #[test]
    fn mixed_placements_fit_inside_container() {
        let container = Dimensions::new(5.0, 5.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = calculate_mixed_capacity(container, item);

        assert_eq!(placements.len(), 3);

        for placement in placements {
            assert!(placement_fits_container(placement, container));
        }
    }
    #[test]
    fn mixed_placements_do_not_overlap() {
        let container = Dimensions::new(5.0, 5.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = calculate_mixed_capacity(container, item);

        for i in 0..placements.len() {
            for j in (i + 1)..placements.len() {
                assert!(
                    !placements_overlap(placements[i], placements[j]),
                    "placements overlap: {:?} and {:?}",
                    placements[i],
                    placements[j]
                );
            }
        }
    }
    #[test]
    fn mixed_packing_finds_four_items_in_two_by_two_layout() {
        let container = Dimensions::new(4.0, 6.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = calculate_mixed_capacity(container, item);

        assert_eq!(placements.len(), 4);

        for placement in &placements {
            assert!(placement_fits_container(*placement, container));
        }

        for i in 0..placements.len() {
            for j in (i + 1)..placements.len() {
                assert!(!placements_overlap(placements[i], placements[j]));
            }
        }
    }
    #[test]
    fn mixed_packing_finds_two_items_in_one_by_two_layout() {
        let container = Dimensions::new(2.0, 6.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = calculate_mixed_capacity(container, item);

        assert_eq!(placements.len(), 2);
    }
    #[test]
    fn mixed_packing_finds_six_items_in_three_by_two_layout() {
        let container = Dimensions::new(6.0, 6.0).unwrap();
        let item = Dimensions::new(2.0, 3.0).unwrap();

        let placements = calculate_mixed_capacity(container, item);

        assert_eq!(placements.len(), 6);

        for placement in &placements {
            assert!(placement_fits_container(*placement, container));
        }
    }
}
