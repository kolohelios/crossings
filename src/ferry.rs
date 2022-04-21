use machine::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FerryType {
    pub id: u8
}

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum Ferry<FerryType> {
        Docked {
            ferry_type: FerryType,
        },
        Loading,
        Departing,
        Crossing,
        Arriving,
        Unloading
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct Load;

transitions!(Ferry, [
    (Docked, Load) => Loading
]);

impl Docked {
    pub fn on_load(self, _: Load) -> Loading {
        Loading {

        }
    }
}

pub mod ferry {
    use super::*;

    pub fn create_ferry(ferry_type: FerryType) -> Ferry {
        let f = Ferry::Docked(Docked { ferry_type: ferry_type });
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry() {
        let ferry_type = FerryType { id: 0 };
        let mut f = ferry::create_ferry(ferry_type);
        assert_eq!(f, Ferry::docked(ferry_type));
        f = f.on_load(Load);
        assert_eq!(f, Ferry::loading());
    }
}
