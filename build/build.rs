#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("build/windows/icon.ico");
    res.compile().expect("Could not compile winres!");
}

#[cfg(not(target_os = "windows"))]
fn main() {}