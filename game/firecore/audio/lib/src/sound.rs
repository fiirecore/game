use deps::str::TinyStr8;

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
        match &self.variant {
            Some(variant) => write!(f, "{} #{}", self.name, variant),
            None => core::fmt::Display::fmt(&self.name, f)
        }
    }
}