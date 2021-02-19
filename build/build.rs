#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("build/icon.ico");
    res.compile().expect("Could not compile winres!");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    std::fs::copy("./build/icon.ico", "./favicon.ico");
}

#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
fn main() {}