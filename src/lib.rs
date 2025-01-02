use nexus::alert::send_alert;
use nexus::gui::RawGuiRender;
use nexus::{log, paths, render, AddonFlags, UpdateProvider};
use once_cell::sync::Lazy;
use sharedlib::{Func, Lib, Symbol};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr::{addr_of, addr_of_mut};
use nexus::quick_access::add_quick_access_context_menu;

// The APP ID for Guild Wars 2 https://steamdb.info/app/1284210/
static STEAM_APP_ID: u32 = 1284210;
static SHORTCUT_ID: &str = "QAS_STEAM_INIT";

nexus::export! {
    name: "Steam Integration",
    signature: -50603,
    load,
    unload,
    flags: AddonFlags::DisableHotloading,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/mythwright/nexus-steam-integration",
}

static mut STEAM_API: Lazy<Lib> = unsafe { Lazy::new(|| Lib::new(get_steam_dll()).unwrap()) };

fn grab_global() -> &'static mut Lazy<Lib> {
    unsafe { &mut *addr_of_mut!(STEAM_API) }
}

fn get_steam_dll() -> PathBuf {
    let steam_path = paths::get_addon_dir("steam_integration").unwrap();
    steam_path.join("steam_api64.dll")
}

const STEAM_DLL: &'static [u8] = include_bytes!("steam_api64.dll");

#[derive(Debug)]
#[allow(dead_code)]
enum SteamInitResult {
    OK = 0,
    GenericFailure = 1,
    NoSteamClient = 2,
    VersionMismatch = 3,
}

fn load() {
    // we need to set these env vars so that the steam api knows which game we're working with
    std::env::set_var("SteamAppId", &STEAM_APP_ID.to_string());
    std::env::set_var("SteamGameId", STEAM_APP_ID.to_string());

    add_quick_access_context_menu(SHORTCUT_ID, None::<&str>, addon_shortcut()).revert_on_unload();

    let addon_path = paths::get_addon_dir("steam_integration").unwrap();
    if !addon_path.exists() {
        fs::create_dir_all(&addon_path).unwrap();
    }

    let steam_path = get_steam_dll();
    if !steam_path.exists() {
        let mut f = File::create(steam_path).unwrap();
        f.write_all(STEAM_DLL).unwrap();
    }

    if !is_steam_running() {
        send_alert("Steam is not running");
        return;
    }
    init_steam();
}

fn is_steam_running() -> bool {
    let steam = grab_global();
    let is_steam_running_symbol: Func<extern "C" fn() -> bool> =
        unsafe { steam.find_func("SteamAPI_IsSteamRunning").unwrap() };
    let is_steam_running_func = unsafe { is_steam_running_symbol.get() };
    log::log(
        log::LogLevel::Debug,
        "Steam Integration",
        format!("Is Steam Running? {}", is_steam_running_func()),
    );
    is_steam_running_func()
}

fn init_steam() {
    let err_buffer: SteamErrMsg = [0; 1024];
    let err_buffer_ptr = addr_of!(err_buffer);
    let init_symbol: Func<extern "C" fn(*const SteamErrMsg) -> SteamInitResult> =
        unsafe { grab_global().find_func("SteamAPI_InitFlat").unwrap() };
    let init_func = unsafe { init_symbol.get() };

    let res = init_func(err_buffer_ptr);
    log::log(
        log::LogLevel::Debug,
        "Steam Integration",
        format!("Init Steam SDK result {:?}", res),
    );
}

fn addon_shortcut() -> RawGuiRender {
    render!(|ui| {
        if ui.button("Initialize Steam") {
            if !is_steam_running() {
                send_alert("Steam is not running");
                return;
            }
            init_steam();
        }
    })
}

type SteamErrMsg = [c_char; 1024];

fn unload() {
    let steam = grab_global();
    unsafe {
        let shutdown_steam_symbol: Func<extern "C" fn()> =
            steam.find_func("SteamAPI_Shutdown").unwrap();
        let shutdown_steam_func = shutdown_steam_symbol.get();

        shutdown_steam_func();
    }
}
