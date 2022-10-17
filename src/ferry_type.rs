
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FerryType {
    pub id: u8,
    pub passenger_capacity: u16,
    pub car_length_capacity_feet: u16,
    pub speed_knots: f32,
}
