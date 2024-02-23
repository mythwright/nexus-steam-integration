use std::ffi::{c_char, c_ulong};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use nexus_rs::raw_structs::{AddonAPI, AddonDefinition, AddonVersion, EAddonFlags, ELogLevel, LPVOID};
use steamworks::Client;
use windows::core::s;
use windows::Win32::Foundation::HINSTANCE;

// The APP ID for Guild Wars 2 https://steamdb.info/app/1284210/
static STEAM_APP_ID: u32 = 1284210;
static mut API: MaybeUninit<&'static AddonAPI> = MaybeUninit::uninit();

#[no_mangle]
unsafe extern "C" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: c_ulong,
    _lpv_reserveded: LPVOID,
) -> bool {
    match fdw_reason {
        _ => {}
    }
    true
}

unsafe extern "C" fn load(api: *mut AddonAPI) {
    let api = &*api;
    API.write(api);

    match Client::init_app(STEAM_APP_ID) {
        Ok(c) => Some(c),
        Err(err) => {
            log(ELogLevel::WARNING, format!("Unable to initialize steam api: {err}"));
            None
        }
    };
}


unsafe extern "C" fn unload() {}

pub fn log(level: ELogLevel, s: String) {
    unsafe {
        let api = API.assume_init();
        (api.log)(
            level,
            (s + "\0").as_ptr() as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn GetAddonDef() -> *mut AddonDefinition {
    static AD: AddonDefinition = AddonDefinition {
        signature: -50603,
        apiversion: nexus_rs::raw_structs::NEXUS_API_VERSION,
        name: b"Steam Integration\0".as_ptr() as *const c_char,
        version: AddonVersion {
            major: 1,
            minor: 0,
            build: 1,
            revision: 0,
        },
        author: s!("Zyian").0 as _,
        description: s!("Initializes the Steam API to show you are playing GW2 on Steam to track time played.").0 as _,
        load,
        unload: Some(unsafe { NonNull::new_unchecked(unload as _) }),
        flags: EAddonFlags::DisableHotloading,
        provider: nexus_rs::raw_structs::EUpdateProvider::GitHub,
        update_link: Some(unsafe {
            NonNull::new_unchecked(s!("https://github.com/mythwright/nexus-steam-integration").0 as _)
        }),
    };

    &AD as *const _ as _
}