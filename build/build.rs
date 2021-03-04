fn main() {

    #[cfg(windows)] {
        if !cfg!(debug_assertions) {
            let mut res = winres::WindowsResource::new();
            res.set_icon("build/icon.ico");
            res.compile().expect("Could not compile winres!");
        }        
    }
    
}