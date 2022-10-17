use machine::*;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum FerryState {
        Docked,
        Loading {
            passengers: u16,
        },
        Crossing {
            passengers: u16,        
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

// #[derive(Clone, Debug, PartialEq)]
// pub struct Dock;

transitions!(FerryState, [
    (Docked, Load) => Loading,
    (Docked, Reposition) => Crossing,
    (Loading, Depart) => [ Loading, Departing ]
    // (Crossing, Dock) => Docked,
]);

impl Docked {
    pub fn on_load(self, input: Load) -> Loading {
        Loading {
            passengers: input.amount,
        }
    }

    pub fn on_reposition(self, _: Reposition) -> Crossing {
        Crossing { 
            passengers: 0
        }
    }
}

impl Loading {
    pub fn on_depart(self, _: Depart) -> FerryState {
        let f = FerryState::crossing(self.passengers);
        f
    }
}

// impl Crossing {
//     pub fn on_dock(self, _: Dock) -> Docked {
//         FerryState::docked()
//     }
// }

pub mod ferry_state {
    use super::*;

    pub fn create_ferry_state() -> FerryState {
        let f = FerryState::Docked(Docked {});
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry_state() {
        let mut f = ferry_state::create_ferry_state();
        assert_eq!(f, FerryState::docked());
        
        f = f.on_reposition(Reposition {});
        assert_eq!(f, FerryState::crossing(0));
        
        // f = f.on_dock(Dock);
        // assert_eq!(f, FerryState::docked());
        // f = f.on_load(Load { amount: 100 });
        // assert_eq!(f, FerryState::loading(100));
        
    }
}
