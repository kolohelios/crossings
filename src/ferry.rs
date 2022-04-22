use machine::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FerryType {
    pub id: u8,
    pub passenger_capacity: u16,
    pub car_length_capacity_feet: u16,
    pub speed_knots: f32,
}

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum Ferry {
        Docked {
            ferry_type: FerryType,
        },
        Loading {
            ferry_type: FerryType,
            passengers: u16,
        },
        Crossing {
            ferry_type: FerryType,
            passengers: u16,
        },
        Unloading {
            ferry_type: FerryType,
            passengers: u16,
        },
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct Load {
    pub amount: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Depart;

transitions!(Ferry, [
    (Docked, Load) => Loading,
    (Loading, Depart) => [ Loading, Departing ]
]);

impl Docked {
    pub fn on_load(self, input: Load) -> Loading {
        Loading {
            ferry_type: self.ferry_type,
            passengers: input.amount,
        }
    }
}

impl Loading {
    pub fn on_depart(self, _: Depart) -> Ferry {
        // Departing {
        //     ferry_type: self.
        // }
        if (self.ferry_type
            == FerryType {
                id: 0,
                passenger_capacity: 2500,
                car_length_capacity_feet: 0,
                speed_knots: 18.0,
            })
        {
            Ferry::crossing(self.ferry_type, self.passengers);
        }
        Ferry::loading(self.ferry_type, self.passengers)
    }
}

pub mod ferry {
    use super::*;

    pub fn create_ferry(ferry_type: FerryType) -> Ferry {
        let f = Ferry::Docked(Docked {
            ferry_type: ferry_type,
        });
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry() {
        let ferry_type = FerryType {
            id: 0,
            passenger_capacity: 2500,
            car_length_capacity_feet: 0,
            speed_knots: 18.0,
        };
        let mut f = ferry::create_ferry(ferry_type);
        assert_eq!(f, Ferry::docked(ferry_type));
        f = f.on_load(Load { amount: 100 });
        assert_eq!(f, Ferry::loading(ferry_type, 100));
    }
}
