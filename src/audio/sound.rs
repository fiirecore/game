#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {

    CryCharizard,

}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Sound::CryCharizard => "cry_charizard",
        })
    }
}