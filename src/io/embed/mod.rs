use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "build"]
pub struct Asset;

impl Asset {

    pub fn get_dir(path: &str) -> Directory {
        Directory {
            
        }
    }

}

pub struct Directory {

    //files: Vec<String>,

}