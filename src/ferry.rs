mod ferry_type;
mod ferry_state;

#[derive(Clone, Debug, PartialEq)]
pub struct Ferry {
    pub id: u8,
    pub ferry_type: FerryType,
    pub name: String,
    pub state: FerryState,
}
