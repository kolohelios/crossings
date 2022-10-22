use libm::{asin, atan2, cos, sin};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Navigation {
    pub bearing: u16,
    pub velocity_in_knots: f32,
    pub location: Location,
}

impl Navigation {
    pub fn get_distance_in_km(speed_in_knots: f64, time_in_seconds: i64) -> f64 {
        let km_per_knot = 1.852;
        let speed_in_kmh = speed_in_knots * km_per_knot;
        let time_in_hours = (time_in_seconds as f64 / 60.0 / 60.0) as f64;

        speed_in_kmh * time_in_hours
    }

    pub fn get_location(
        distance: f64,
        bearing: f64,
        starting_lat: f64,
        starting_lng: f64,
    ) -> Location {
        let earth_radius = 6378.1;

        let starting_lat_in_rads = starting_lat.to_radians();
        let starting_lng_in_rads = starting_lng.to_radians();
        let bearing_in_rads = bearing.to_radians();

        let new_lat = asin(
            sin(starting_lat_in_rads) * cos(distance / earth_radius)
                + cos(starting_lat_in_rads as f64)
                    * sin(distance / earth_radius)
                    * cos(bearing_in_rads),
        );
        let new_lng = starting_lng_in_rads
            + atan2(
                sin(bearing_in_rads) * sin(distance / earth_radius) * cos(starting_lat),
                cos(distance / earth_radius) - sin(starting_lat_in_rads) * sin(new_lat),
            );

        Location {
            latitude: new_lat.to_degrees(),
            longitude: new_lng.to_degrees(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_distance() {
        let calc = Navigation::get_distance_in_km(20.0, 300);
        assert_eq!(calc, 3.0866666666666664);
    }

    #[test]
    fn test_get_location() {
        let mut calc = Navigation::get_location(10.0, 180.0, 90.0, 90.0);
        assert_eq!(
            calc,
            Location {
                latitude: 89.91016795046504,
                longitude: 0.0
            }
        );

        calc = Navigation::get_location(100.0, 35.5, 10.0, -10.0);
        assert_eq!(
            calc,
            Location {
                latitude: 10.730906831486875,
                longitude: -10.45236252065217
            }
        );

        calc = Navigation::get_location(5.0, 90.0, 45.0, 110.0);
        assert_eq!(
            calc,
            Location {
                latitude: 44.999982394437374,
                longitude: 110.04719074991667
            }
        );
    }
}
