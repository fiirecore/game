#[derive(PartialEq)]
pub enum Args {
    DisableAudio,
    Debug,
    #[cfg(debug_assertions)]
    NoSeed,
}

pub fn args() -> Vec<Args> {

    #[cfg(target_arch = "wasm32")]
    {
        Vec::new()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut list = Vec::new();
        let mut args = pico_args::Arguments::from_env();

        if args.contains("-a") {
            list.push(Args::DisableAudio);
        }

        if args.contains("-d") {
            list.push(Args::Debug);
        }

        #[cfg(debug_assertions)]
        if args.contains("-s") {
            list.push(Args::NoSeed);
        }
        list
    }

}
