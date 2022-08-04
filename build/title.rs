use std::{path::Path, fs::read};

pub fn build(root: impl AsRef<Path>, assets: &Path) {
    crate::write::<TitleAsset, _>(
        &root,
        "title",
        title(assets.join("scenes/title")).unwrap(),
    );
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TitleAsset {
    title: Vec<u8>,
    trademark: Vec<u8>,
    subtitle: Vec<u8>,
    charizard: Vec<u8>,
    start: Vec<u8>,
    copyright: Vec<u8>,
}

fn title(path: impl AsRef<Path>) -> Result<TitleAsset, std::io::Error> {
    let path = path.as_ref();
    Ok(TitleAsset {
        title: read(path.join("title.png"))?,
        trademark: read(path.join("trademark.png"))?,
        subtitle: read(path.join("subtitle.png"))?,
        charizard: read(path.join("charizard.png"))?,
        start: read(path.join("start.png"))?,
        copyright: read(path.join("copyright.png"))?,
    })
}