// use tinystr::{Error, TinyStr16};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct Sound {

    pub name: String,
    pub variant: Option<u16>,

}

impl Sound {

    pub fn named(name: &str) -> 
        // Result<Self, Error> 
        Self
    {
        // name.parse().map(|name| Self {
        //     name,
        //     variant: None,
        // })
        Self {
            name: name.to_owned(),
            variant: None,
        }
    }

    pub fn variant(name: &str, variant: Option<u16>) -> 
        // Result<Self, Error> 
        Self
    {
        // name.parse().map(|name| Self {
        //     name,
        //     variant,
        // })
        Self {
            name: name.to_owned(),
            variant,
        }
    }

}

impl core::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.variant.map(|var| String::from("#") + &var.to_string()).unwrap_or(String::new()))
    }
}