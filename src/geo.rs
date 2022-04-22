use geo::{geodesic_distance::GeodesicDistance, *};

pub fn winslow_terminal_coordinates() -> Coordinate<f64> {
    Coordinate {
        x: -122.50970639545716,
        y: 47.62248868450702,
    }
}

pub fn colman_dock_coordinates() -> Coordinate<f64> {
    Coordinate {
        x: -122.33982407135487,
        y: 47.60284792284787,
    }
}

pub fn calculate_distance_meters(point1: Point<f64>, point2: Point<f64>) -> f64 {
    point1.geodesic_distance(&point2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry() {
        let distance = calculate_distance_meters(
            winslow_terminal_coordinates().into(),
            colman_dock_coordinates().into(),
        );
        assert_eq!(distance, 12957.46475162827);
    }
}
