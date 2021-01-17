use std::path::{Path, PathBuf};

//pub static UNKNOWN_FILENAME_ERR: &str = "unknown file (could not unwrap filename)";

static DIR: &str = "assets";

pub fn asset_as_pathbuf<P>(path: P) -> PathBuf where P: AsRef<Path> {
	let path = path.as_ref();
	let mut pathbuf = PathBuf::from(DIR);
	pathbuf.push(path);
	pathbuf
}