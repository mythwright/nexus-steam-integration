use nexus::{
    log,
    AddonFlags, UpdateProvider,
};
use steamworks::{Client};

// The APP ID for Guild Wars 2 https://steamdb.info/app/1284210/
static STEAM_APP_ID: u32 = 1284210;

nexus::export! {
    name: "Steam Integration",
    signature: -50603,
    load,
    unload,
    flags: AddonFlags::DisableHotloading,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/mythwright/nexus-steam-integration",
}

fn load() {
    match Client::init_app(STEAM_APP_ID) {
        Err(err) => {
            log::log(
                log::LogLevel::Warning,
                "Steam Integration",
                format!("Unable to initialize steam api: {err}"),
            );
        }
        _ => {}
    };
}

fn unload() {}


