use std::path::Path;

use rayon::iter::{ParallelBridge, ParallelIterator};
use firecore_world_gen::*;

const PARSED: &str = "output/parsed.bin";

fn main() -> anyhow::Result<()> {
    let mappings = ron::from_str(&std::fs::read_to_string("./mappings.ron")?)?;

    let edits = ron::from_str(&std::fs::read_to_string("./edits.ron")?)?;

    fn load() -> anyhow::Result<ParsedData> {
        anyhow::Result::<ParsedData>::Ok(postcard::from_bytes::<ParsedData>(&std::fs::read(PARSED)?)?)
    }

    let data = load().or_else::<anyhow::Error, _>(|_| {
        let data = create_data()?;
        std::fs::write(PARSED, &postcard::to_allocvec(&data)?)?;
        Ok(data)
    })?;

    let data = compile(mappings, edits, data).unwrap();

    let root = Path::new("output");

    let mapdir = root.join("maps");

    let files = mapdir.join("files");
    let copies = mapdir.join("copies");

    if files.exists() {
        std::fs::remove_dir_all(&files)?;
    }

    std::fs::create_dir_all(&files)?;

    std::fs::create_dir_all(&copies)?;

    data.maps.iter().par_bridge().try_for_each::<_, anyhow::Result<()>>(|r| {
        let location = r.0;
        let map = r.1;
        let data = postcard::to_allocvec(&map)?;

        let path = match location.map {
            Some(map) => format!("{}-{}.world", map.as_str(), location.index.as_str()),
            None => format!("{}.world", location.index.as_str()),
        };

        let file = files.join(&path);

        std::fs::write(file, &data)?;

        let copy = copies.join(&path);

        let str = ron::ser::to_string_pretty(&map, Default::default())?;

        std::fs::write(copy, str.as_bytes())?;
        Ok(())
    })?;

    let scriptdir = root.join("scripts");

    if !scriptdir.exists() {
        std::fs::create_dir_all(&scriptdir)?;
    }

    std::fs::write(
        scriptdir.join("world_scripts.bin"),
        postcard::to_allocvec(&data.scripts)?,
    )?;

    std::fs::write(
        scriptdir.join("scripts.ron"),
        ron::ser::to_string_pretty(&data.scripts, Default::default())?,
    )?;

    Ok(())
}
