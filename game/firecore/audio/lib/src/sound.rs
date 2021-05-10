use firecore_dependencies::tinystr::TinyStr8;

pub type SoundId = TinyStr8;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct Sound {

    pub name: SoundId,
    pub variant: Option<u16>,

}

impl Sound {

    pub fn named(name: TinyStr8) -> Self {
        Self {
            name,
            variant: None,
        }
    }

    pub fn variant(name: TinyStr8, variant: Option<u16>) -> Self {
        Self {
            name,
            variant,
        }
    }

}

impl core::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.variant.map(|var| String::from("#") + &var.to_string()).unwrap_or(String::new()))
    }
}