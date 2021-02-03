#[derive(PartialEq)]
pub enum Args {

    DisableAudio

}

pub fn parse_args() -> Vec<Args> {
    #[cfg(feature = "args")]
    return getopts();
    #[cfg(not(feature = "args"))]
    return Vec::new();
}

#[cfg(feature = "args")]
fn getopts() -> Vec<Args> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    let mut list = Vec::new();

    opts.optflag("a", "disable-audio", "Disable audio");

    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("a") {
                list.push(Args::DisableAudio);
            }
        }
        Err(f) => {
            macroquad::prelude::warn!("Could not parse command line arguments with error {}", f.to_string());
        }
    };
    return list;
}