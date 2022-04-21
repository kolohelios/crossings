use machine::*;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum Ferry {
        Docked,
        Loading,
        Departing,
        Crossing,
        Arriving,
        Unloading
    }
);

impl Docked {

}

pub mod ferry {
    use super::*;
    pub fn create_ferry() -> Ferry {
        let f = Ferry::Docked(Docked {});
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ferry() {
        let mut f = ferry::create_ferry();
        assert_eq!(f, Ferry::docked());
    }
}
