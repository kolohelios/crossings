
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FerryType {
    pub id: u8,
    pub passenger_capacity: u16,
    pub car_length_capacity_feet: u16,
    pub max_speed_knots: f32,
    pub cruise_speed_knots: f32,
}
