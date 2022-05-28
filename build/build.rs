fn write<S: serde::Serialize>(file: &str, data: &S) {
    let dest = firecore_storage::directory(false, PUBLISHER, APPLICATION)
        .unwrap()
        .join("assets");
    if !dest.exists() {
        std::fs::create_dir_all(&dest).unwrap();
    }

    std::fs::write(
        dest.join(format!("{}.bin", file)),
        firecore_storage::to_bytes(data).unwrap(),
    )
    .unwrap()
}

const PUBLISHER: Option<&str> = Some("fiirecore");
const APPLICATION: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("cargo:rerun-if-changed=assets");

    if std::fs::read_dir("assets/game")
        .map(|d| d.count())
        .unwrap_or_default()
        == 0
    {
        std::process::Command::new("git")
            .args(["submodule", "init"])
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not initialize git submodules using git command with error {}",
                    err
                )
            })
            .wait()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not wait on git submodule init task to complete with error {}",
                    err
                )
            });

        std::process::Command::new("git")
            .args(["submodule", "update", "--remote"])
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not update git submodules using git command with error {}",
                    err
                )
            }).wait().unwrap_or_else(|err| {
                panic!(
                    "Could not wait on git submodule update --remote task to complete with error {}",
                    err
                )
            });
    }

    #[cfg(feature = "audio")]
    write("audio", &firecore_audio_builder::compile("assets/music"));

    let (pokedex, movedex, itemdex) = firecore_pokedex_builder::compile(
        "assets/game/pokedex/pokemon",
        "assets/game/pokedex/moves",
        "assets/game/pokedex/items",
    );

    write("pokedex", &pokedex);
    write("movedex", &movedex);
    write("itemdex", &itemdex);

    let dex_engine = firecore_pokedex_engine_builder::compile(
        "assets/game/pokedex/client/pokemon",
        "assets/game/pokedex/client/items",
        "assets/game/pokedex/client/trainers",
    );

    write("dex_engine", &dex_engine);

    let world = firecore_world_builder::compile("assets/world");

    write("world", &world);

    let battle = std::path::Path::new("assets/game/pokedex/battle");

    let battle = firecore_battle_builder::compile(battle, &battle.join("scripts"));

    write("battle", &battle);

    #[cfg(windows)]
    if std::env::var("PROFILE").unwrap() == "release" && std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        embed_resource::compile("build/resource.rc");
    }
}
