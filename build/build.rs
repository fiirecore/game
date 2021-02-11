#[cfg(windows)]
fn main() {
    // let mut res = winres::WindowsResource::new();
    // res.set_icon("build/windows/icon.ico");
    // res.compile().expect("Could not compile winres!");
}

#[cfg(not(windows))]
fn main() {}