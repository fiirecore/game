pub enum Nature {

    Adamant,

}

impl Nature {

    pub fn change(&self) -> (bool, bool, bool, bool, bool) {
        match self {
            Self::Adamant => (true, false, true, false, false),
        }
    }

}