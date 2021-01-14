//extern crate includedir_codegen;

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    winres();
    //includedir();
}

#[cfg(unix)]
fn main() {
    //includedir();
}

#[cfg(windows)]
fn winres() {
	let mut res = winres::WindowsResource::new();
    res.set_icon("build/icon.ico");
    res.compile().unwrap();
}

/*

fn includedir() {
    includedir_codegen::start("FILES")
        .dir("assets", includedir_codegen::Compression::Gzip)
        .build("assets.rs")
        .unwrap();
}

*/