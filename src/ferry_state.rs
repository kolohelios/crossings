use chrono::{DateTime, NaiveDateTime, Utc};
use machine::*;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum FerryState {
        Docked {
            next_departure: DateTime<Utc>,
        },
        Loading {
            passengers: u16,
        },
        Crossing {
            passengers: u16,
            location_lat: f32,
            location_lng: f32,
        },
        Unloading {
            passengers: u16,
        },
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct Load {
    pub amount: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Reposition;

#[derive(Clone, Debug, PartialEq)]
pub struct Depart;

#[derive(Clone, Debug, PartialEq)]
pub struct Dock;

transitions!(FerryState, [
    // Do not use trailing commas; the Machine macros will cause things to break in strange ways
    (Docked, Load) => Loading,
    (Docked, Reposition) => Crossing,
    (Loading, Depart) => Crossing,
    (Crossing, Dock) => Docked
]);

impl Docked {
    pub fn on_load(self, input: Load) -> Loading {
        Loading {
            passengers: input.amount,
        }
    }

    pub fn on_reposition(self, _: Reposition) -> Crossing {
        Crossing {
            passengers: 0,
            location_lat: 0.0,
            location_lng: 0.0,
        }
    }
}

impl Loading {
    pub fn on_depart(self, _: Depart) -> Crossing {
        Crossing {
            passengers: self.passengers,
            location_lat: 0.0,
            location_lng: 0.0,
        }
    }
}

impl Crossing {
    pub fn on_dock(self, _: Dock) -> Docked {
        Docked {
            next_departure: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }
}

pub mod ferry_state {
    use super::*;

    pub fn create_ferry_state() -> FerryState {
        let f = FerryState::Docked(Docked {
            next_departure: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        });
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry_state() {
        let mut f = ferry_state::create_ferry_state();
        assert_eq!(
            f,
            FerryState::docked(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(0, 0),
                Utc
            ))
        );

        f = f.on_reposition(Reposition {});
        assert_eq!(f, FerryState::crossing(0, 0.0, 0.0));

        f = f.on_dock(Dock);
        assert_eq!(
            f,
            FerryState::docked(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(0, 0),
                Utc
            ))
        );

        f = f.on_load(Load { amount: 100 });
        assert_eq!(f, FerryState::loading(100));

        f = f.on_depart(Depart);
        assert_eq!(f, FerryState::crossing(100, 0.0, 0.0));
    }
}
