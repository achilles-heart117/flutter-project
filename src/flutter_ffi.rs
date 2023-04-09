#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::common::get_default_sound_input;
use crate::{
    client::file_trait::FileManager,
    common::is_keyboard_mode_supported,
    common::make_fd_to_json,
    flutter::{self, SESSIONS},
    flutter::{session_add, session_start_},
    ui_interface::{self, *},
};
use flutter_rust_bridge::{StreamSink, SyncReturn};
use hbb_common::{
    config::{self, LocalConfig, PeerConfig, PeerInfoSerde, ONLINE},
    fs, log,
    message_proto::KeyboardMode,
    ResultType,
};
use serde_json::json;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    os::raw::c_char,
    str::FromStr,
    time::SystemTime,
};

// use crate::hbbs_http::account::AuthResult;

fn initialize(app_dir: &str) {
    *config::APP_DIR.write().unwrap() = app_dir.to_owned();
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Debug) // limit log level
                .with_tag("ffi"), // logs will show under mytag tag
        );
        #[cfg(feature = "mediacodec")]
        scrap::mediacodec::check_mediacodec();
        crate::common::test_rendezvous_server();
        crate::common::test_nat_type();
        crate::common::check_software_update();
    }
    #[cfg(target_os = "ios")]
    {
        use hbb_common::env_logger::*;
        init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));
    }
}

pub enum EventToUI {
    Event(String),
    Rgba,
}

pub fn start_global_event_stream(s: StreamSink<String>, app_type: String) -> ResultType<()> {
    if let Some(_) = flutter::GLOBAL_EVENT_STREAM
        .write()
        .unwrap()
        .insert(app_type.clone(), s)
    {
        log::warn!(
            "Global event stream of type {} is started before, but now removed",
            app_type
        );
    }
    Ok(())
}

pub fn stop_global_event_stream(app_type: String) {
    let _ = flutter::GLOBAL_EVENT_STREAM
        .write()
        .unwrap()
        .remove(&app_type);
}

pub fn host_stop_system_key_propagate(_stopped: bool) {
    #[cfg(windows)]
    crate::platform::windows::stop_system_key_propagate(_stopped);
}

// FIXME: -> ResultType<()> cannot be parsed by frb_codegen
// thread 'main' panicked at 'Failed to parse function output type `ResultType<()>`', $HOME\.cargo\git\checkouts\flutter_rust_bridge-ddba876d3ebb2a1e\e5adce5\frb_codegen\src\parser\mod.rs:151:25
pub fn session_add_sync(
    id: String,
    is_file_transfer: bool,
    is_port_forward: bool,
    switch_uuid: String,
    force_relay: bool,
    password: String,
) -> SyncReturn<String> {
    if let Err(e) = session_add(
        &id,
        is_file_transfer,
        is_port_forward,
        &switch_uuid,
        force_relay,
        password,
    ) {
        SyncReturn(format!("Failed to add session with id {}, {}", &id, e))
    } else {
        SyncReturn("".to_owned())
    }
}

pub fn session_start(events2ui: StreamSink<EventToUI>, id: String) -> ResultType<()> {
    session_start_(&id, events2ui)
}

pub fn session_get_remember(id: String) -> Option<bool> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_remember())
    } else {
        None
    }
}

pub fn session_get_toggle_option(id: String, arg: String) -> Option<bool> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_toggle_option(arg))
    } else {
        None
    }
}

pub fn session_get_toggle_option_sync(id: String, arg: String) -> SyncReturn<bool> {
    let res = session_get_toggle_option(id, arg) == Some(true);
    SyncReturn(res)
}

pub fn session_get_option(id: String, arg: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_option(arg))
    } else {
        None
    }
}

pub fn session_login(
    id: String,
    os_username: String,
    os_password: String,
    password: String,
    remember: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.login(os_username, os_password, password, remember);
    }
}

pub fn session_close(id: String) {
    if let Some(mut session) = SESSIONS.write().unwrap().remove(&id) {
        session.close_event_stream();
        session.close();
    }
}

pub fn session_refresh(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.refresh_video();
    }
}

pub fn session_record_screen(id: String, start: bool, width: usize, height: usize) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.record_screen(start, width as _, height as _);
    }
}

pub fn session_reconnect(id: String, force_relay: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.reconnect(force_relay);
    }
}

pub fn session_toggle_option(id: String, value: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        log::warn!("toggle option {}", &value);
        session.toggle_option(value.clone());
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    if SESSIONS.read().unwrap().get(&id).is_some() && value == "disable-clipboard" {
        crate::flutter::update_text_clipboard_required();
    }
}

pub fn session_get_flutter_config(id: String, k: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_flutter_config(k))
    } else {
        None
    }
}

pub fn session_set_flutter_config(id: String, k: String, v: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_flutter_config(k, v);
    }
}

pub fn get_local_flutter_config(k: String) -> SyncReturn<String> {
    SyncReturn(ui_interface::get_local_flutter_config(k))
}

pub fn set_local_flutter_config(k: String, v: String) {
    ui_interface::set_local_flutter_config(k, v);
}

pub fn get_local_kb_layout_type() -> SyncReturn<String> {
    SyncReturn(ui_interface::get_kb_layout_type())
}

pub fn set_local_kb_layout_type(kb_layout_type: String) {
    ui_interface::set_kb_layout_type(kb_layout_type)
}

pub fn session_get_view_style(id: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_view_style())
    } else {
        None
    }
}

pub fn session_set_view_style(id: String, value: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_view_style(value);
    }
}

pub fn session_get_scroll_style(id: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_scroll_style())
    } else {
        None
    }
}

pub fn session_set_scroll_style(id: String, value: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_scroll_style(value);
    }
}

pub fn session_get_image_quality(id: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_image_quality())
    } else {
        None
    }
}

pub fn session_set_image_quality(id: String, value: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_image_quality(value);
    }
}

pub fn session_get_keyboard_mode(id: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_keyboard_mode())
    } else {
        None
    }
}

pub fn session_set_keyboard_mode(id: String, value: String) {
    let mut _mode_updated = false;
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_keyboard_mode(value);
        _mode_updated = true;
    }
    #[cfg(windows)]
    if _mode_updated {
        crate::keyboard::update_grab_get_key_name();
    }
}

pub fn session_get_custom_image_quality(id: String) -> Option<Vec<i32>> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_custom_image_quality())
    } else {
        None
    }
}

pub fn session_is_keyboard_mode_supported(id: String, mode: String) -> SyncReturn<bool> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        if let Ok(mode) = KeyboardMode::from_str(&mode[..]) {
            SyncReturn(is_keyboard_mode_supported(
                &mode,
                session.get_peer_version(),
            ))
        } else {
            SyncReturn(false)
        }
    } else {
        SyncReturn(false)
    }
}

pub fn session_set_custom_image_quality(id: String, value: i32) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.save_custom_image_quality(value);
    }
}

pub fn session_set_custom_fps(id: String, fps: i32) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.set_custom_fps(fps);
    }
}

pub fn session_lock_screen(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.lock_screen();
    }
}

pub fn session_ctrl_alt_del(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.ctrl_alt_del();
    }
}

pub fn session_switch_display(id: String, value: i32) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.switch_display(value);
    }
}

pub fn session_handle_flutter_key_event(
    id: String,
    name: String,
    platform_code: i32,
    position_code: i32,
    lock_modes: i32,
    down_or_up: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.handle_flutter_key_event(&name, platform_code, position_code, lock_modes, down_or_up);
    }
}

pub fn session_enter_or_leave(_id: String, _enter: bool) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    if let Some(session) = SESSIONS.read().unwrap().get(&_id) {
        if _enter {
            session.enter();
        } else {
            session.leave();
        }
    }
}

pub fn session_input_key(
    id: String,
    name: String,
    down: bool,
    press: bool,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        // #[cfg(any(target_os = "android", target_os = "ios"))]
        session.input_key(&name, down, press, alt, ctrl, shift, command);
    }
}

pub fn session_input_string(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        // #[cfg(any(target_os = "android", target_os = "ios"))]
        session.input_string(&value);
    }
}

// chat_client_mode
pub fn session_send_chat(id: String, text: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.send_chat(text);
    }
}

pub fn session_peer_option(id: String, name: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.set_option(name, value);
    }
}

pub fn session_get_peer_option(id: String, name: String) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.get_option(name);
    }
    "".to_string()
}

pub fn session_input_os_password(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.input_os_password(value, true);
    }
}

// File Action
pub fn session_read_remote_dir(id: String, path: String, include_hidden: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.read_remote_dir(path, include_hidden);
    }
}

pub fn session_send_files(
    id: String,
    act_id: i32,
    path: String,
    to: String,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.send_files(act_id, path, to, file_num, include_hidden, is_remote);
    }
}

pub fn session_set_confirm_override_file(
    id: String,
    act_id: i32,
    file_num: i32,
    need_override: bool,
    remember: bool,
    is_upload: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.set_confirm_override_file(act_id, file_num, need_override, remember, is_upload);
    }
}

pub fn session_remove_file(id: String, act_id: i32, path: String, file_num: i32, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_file(act_id, path, file_num, is_remote);
    }
}

pub fn session_read_dir_recursive(
    id: String,
    act_id: i32,
    path: String,
    is_remote: bool,
    show_hidden: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_dir_all(act_id, path, is_remote, show_hidden);
    }
}

pub fn session_remove_all_empty_dirs(id: String, act_id: i32, path: String, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_dir(act_id, path, is_remote);
    }
}

pub fn session_cancel_job(id: String, act_id: i32) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.cancel_job(act_id);
    }
}

pub fn session_create_dir(id: String, act_id: i32, path: String, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.create_dir(act_id, path, is_remote);
    }
}

pub fn session_read_local_dir_sync(_id: String, path: String, show_hidden: bool) -> String {
    if let Ok(fd) = fs::read_dir(&fs::get_path(&path), show_hidden) {
        return make_fd_to_json(fd.id, path, &fd.entries);
    }
    "".to_string()
}

pub fn session_get_platform(id: String, is_remote: bool) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.get_platform(is_remote);
    }
    "".to_string()
}

pub fn session_load_last_transfer_jobs(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.load_last_jobs();
    } else {
        // a tip for flutter dev
        eprintln!(
            "cannot load last transfer job from non-existed session. Please ensure session \
        is connected before calling load last transfer jobs."
        );
    }
}

pub fn session_add_job(
    id: String,
    act_id: i32,
    path: String,
    to: String,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.add_job(act_id, path, to, file_num, include_hidden, is_remote);
    }
}

pub fn session_resume_job(id: String, act_id: i32, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.resume_job(act_id, is_remote);
    }
}

pub fn session_elevate_direct(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.elevate_direct();
    }
}

pub fn session_elevate_with_logon(id: String, username: String, password: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.elevate_with_logon(username, password);
    }
}

pub fn session_switch_sides(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.switch_sides();
    }
}

pub fn session_change_resolution(id: String, width: i32, height: i32) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.change_resolution(width, height);
    }
}

pub fn session_set_size(_id: String, _width: i32, _height: i32) {
    #[cfg(feature = "flutter_texture_render")]
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&_id) {
        session.set_size(_width, _height);
    }
}

pub fn main_get_sound_inputs() -> Vec<String> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return get_sound_inputs();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    vec![String::from("")]
}

pub fn main_get_default_sound_input() -> Option<String> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return get_default_sound_input();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    None
}

pub fn main_get_hostname() -> SyncReturn<String> {
    SyncReturn(crate::common::hostname())
}

pub fn main_change_id(new_id: String) {
    change_id(new_id)
}

pub fn main_get_async_status() -> String {
    get_async_job_status()
}

pub fn main_get_option(key: String) -> String {
    get_option(key)
}

pub fn main_get_error() -> String {
    get_error()
}

pub fn main_set_option(key: String, value: String) {
    if key.eq("custom-rendezvous-server") {
        set_option(key, value);
        #[cfg(target_os = "android")]
        crate::rendezvous_mediator::RendezvousMediator::restart();
        #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
        crate::common::test_rendezvous_server();
    } else {
        set_option(key, value);
    }
}

pub fn main_get_options() -> String {
    get_options()
}

pub fn main_set_options(json: String) {
    let map: HashMap<String, String> = serde_json::from_str(&json).unwrap_or(HashMap::new());
    if !map.is_empty() {
        set_options(map)
    }
}

pub fn main_test_if_valid_server(server: String) -> String {
    test_if_valid_server(server)
}

pub fn main_set_socks(proxy: String, username: String, password: String) {
    set_socks(proxy, username, password)
}

pub fn main_get_socks() -> Vec<String> {
    get_socks()
}

pub fn main_get_app_name() -> String {
    get_app_name()
}

pub fn main_get_app_name_sync() -> SyncReturn<String> {
    SyncReturn(get_app_name())
}

pub fn main_get_license() -> String {
    get_license()
}

pub fn main_get_version() -> String {
    get_version()
}

pub fn main_get_fav() -> Vec<String> {
    get_fav()
}

pub fn main_store_fav(favs: Vec<String>) {
    store_fav(favs)
}

pub fn main_get_peer(id: String) -> String {
    let conf = get_peer(id);
    serde_json::to_string(&conf).unwrap_or("".to_string())
}

pub fn main_get_lan_peers() -> String {
    serde_json::to_string(&get_lan_peers()).unwrap_or_default()
}

pub fn main_get_connect_status() -> String {
    let status = get_connect_status();
    // (status_num, key_confirmed, mouse_time, id)
    let mut m = serde_json::Map::new();
    m.insert("status_num".to_string(), json!(status.0));
    m.insert("key_confirmed".to_string(), json!(status.1));
    m.insert("mouse_time".to_string(), json!(status.2));
    m.insert("id".to_string(), json!(status.3));
    serde_json::to_string(&m).unwrap_or("".to_string())
}

pub fn main_check_connect_status() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    start_option_status_sync(); // avoid multi calls
}

pub fn main_is_using_public_server() -> bool {
    using_public_server()
}

pub fn main_discover() {
    discover();
}

pub fn main_get_api_server() -> String {
    get_api_server()
}

pub fn main_post_request(url: String, body: String, header: String) {
    post_request(url, body, header)
}

pub fn main_get_local_option(key: String) -> SyncReturn<String> {
    SyncReturn(get_local_option(key))
}

pub fn main_set_local_option(key: String, value: String) {
    set_local_option(key, value)
}

pub fn main_get_my_id() -> String {
    get_id()
}

pub fn main_get_uuid() -> String {
    get_uuid()
}

pub fn main_get_peer_option(id: String, key: String) -> String {
    get_peer_option(id, key)
}

pub fn main_get_peer_option_sync(id: String, key: String) -> SyncReturn<String> {
    SyncReturn(get_peer_option(id, key))
}

pub fn main_set_peer_option(id: String, key: String, value: String) {
    set_peer_option(id, key, value)
}

pub fn main_set_peer_option_sync(id: String, key: String, value: String) -> SyncReturn<bool> {
    set_peer_option(id, key, value);
    SyncReturn(true)
}

pub fn main_set_peer_alias(id: String, alias: String) {
    main_broadcast_message(&HashMap::from([
        ("name", "alias"),
        ("id", &id),
        ("alias", &alias),
    ]));
    set_peer_option(id, "alias".to_owned(), alias)
}

pub fn main_forget_password(id: String) {
    forget_password(id)
}

pub fn main_peer_has_password(id: String) -> bool {
    peer_has_password(id)
}

pub fn main_load_recent_peers() {
    if !config::APP_DIR.read().unwrap().is_empty() {
        let peers: Vec<HashMap<&str, String>> = PeerConfig::peers()
            .drain(..)
            .map(|(id, _, p)| peer_to_map(id, p))
            .collect();
        if let Some(s) = flutter::GLOBAL_EVENT_STREAM
            .read()
            .unwrap()
            .get(flutter::APP_TYPE_MAIN)
        {
            let data = HashMap::from([
                ("name", "load_recent_peers".to_owned()),
                (
                    "peers",
                    serde_json::ser::to_string(&peers).unwrap_or("".to_owned()),
                ),
            ]);
            s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
        };
    }
}

pub fn main_load_fav_peers() {
    if !config::APP_DIR.read().unwrap().is_empty() {
        let favs = get_fav();
        let mut recent = PeerConfig::peers();
        let mut lan = config::LanPeers::load()
            .peers
            .iter()
            .filter(|d| recent.iter().all(|r| r.0 != d.id))
            .map(|d| {
                (
                    d.id.clone(),
                    SystemTime::UNIX_EPOCH,
                    PeerConfig {
                        info: PeerInfoSerde {
                            username: d.username.clone(),
                            hostname: d.hostname.clone(),
                            platform: d.platform.clone(),
                        },
                        ..Default::default()
                    },
                )
            })
            .collect();
        recent.append(&mut lan);
        let peers: Vec<HashMap<&str, String>> = recent
            .into_iter()
            .filter_map(|(id, _, p)| {
                if favs.contains(&id) {
                    Some(peer_to_map(id, p))
                } else {
                    None
                }
            })
            .collect();
        if let Some(s) = flutter::GLOBAL_EVENT_STREAM
            .read()
            .unwrap()
            .get(flutter::APP_TYPE_MAIN)
        {
            let data = HashMap::from([
                ("name", "load_fav_peers".to_owned()),
                (
                    "peers",
                    serde_json::ser::to_string(&peers).unwrap_or("".to_owned()),
                ),
            ]);
            s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
        };
    }
}

pub fn main_load_lan_peers() {
    if let Some(s) = flutter::GLOBAL_EVENT_STREAM
        .read()
        .unwrap()
        .get(flutter::APP_TYPE_MAIN)
    {
        let data = HashMap::from([
            ("name", "load_lan_peers".to_owned()),
            (
                "peers",
                serde_json::to_string(&get_lan_peers()).unwrap_or_default(),
            ),
        ]);
        s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
    };
}

pub fn main_remove_discovered(id: String) {
    remove_discovered(id);
}

fn main_broadcast_message(data: &HashMap<&str, &str>) {
    let apps = vec![
        flutter::APP_TYPE_DESKTOP_REMOTE,
        flutter::APP_TYPE_DESKTOP_FILE_TRANSFER,
        flutter::APP_TYPE_DESKTOP_PORT_FORWARD,
    ];

    for app in apps {
        if let Some(s) = flutter::GLOBAL_EVENT_STREAM.read().unwrap().get(app) {
            s.add(serde_json::ser::to_string(data).unwrap_or("".to_owned()));
        };
    }
}

pub fn main_change_theme(dark: String) {
    main_broadcast_message(&HashMap::from([("name", "theme"), ("dark", &dark)]));
    send_to_cm(&crate::ipc::Data::Theme(dark));
}

pub fn main_change_language(lang: String) {
    main_broadcast_message(&HashMap::from([("name", "language"), ("lang", &lang)]));
    send_to_cm(&crate::ipc::Data::Language(lang));
}

pub fn main_default_video_save_directory() -> String {
    default_video_save_directory()
}

pub fn main_set_user_default_option(key: String, value: String) {
    set_user_default_option(key, value);
}

pub fn main_get_user_default_option(key: String) -> SyncReturn<String> {
    SyncReturn(get_user_default_option(key))
}

pub fn main_handle_relay_id(id: String) -> String {
    handle_relay_id(id)
}

pub fn session_add_port_forward(
    id: String,
    local_port: i32,
    remote_host: String,
    remote_port: i32,
) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.add_port_forward(local_port, remote_host, remote_port);
    }
}

pub fn session_remove_port_forward(id: String, local_port: i32) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.remove_port_forward(local_port);
    }
}

pub fn session_new_rdp(id: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.new_rdp();
    }
}

pub fn session_request_voice_call(id: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.request_voice_call();
    }
}

pub fn session_close_voice_call(id: String) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.close_voice_call();
    }
}

pub fn cm_handle_incoming_voice_call(id: i32, accept: bool) {
    crate::ui_cm_interface::handle_incoming_voice_call(id, accept);
}

pub fn cm_close_voice_call(id: i32) {
    crate::ui_cm_interface::close_voice_call(id);
}

pub fn main_get_last_remote_id() -> String {
    LocalConfig::get_remote_id()
}

pub fn main_get_software_update_url() -> String {
    crate::common::SOFTWARE_UPDATE_URL.lock().unwrap().clone()
}

pub fn main_get_home_dir() -> String {
    fs::get_home_as_string()
}

pub fn main_get_langs() -> String {
    get_langs()
}

pub fn main_get_temporary_password() -> String {
    ui_interface::temporary_password()
}

pub fn main_get_permanent_password() -> String {
    ui_interface::permanent_password()
}

pub fn main_get_online_statue() -> i64 {
    ONLINE.lock().unwrap().values().max().unwrap_or(&0).clone()
}

pub fn cm_get_clients_state() -> String {
    crate::ui_cm_interface::get_clients_state()
}

pub fn cm_check_clients_length(length: usize) -> Option<String> {
    if length != crate::ui_cm_interface::get_clients_length() {
        Some(crate::ui_cm_interface::get_clients_state())
    } else {
        None
    }
}

pub fn cm_get_clients_length() -> usize {
    crate::ui_cm_interface::get_clients_length()
}

pub fn main_init(app_dir: String) {
    initialize(&app_dir);
}

pub fn main_device_id(id: String) {
    *crate::common::DEVICE_ID.lock().unwrap() = id;
}

pub fn main_device_name(name: String) {
    *crate::common::DEVICE_NAME.lock().unwrap() = name;
}

pub fn main_remove_peer(id: String) {
    PeerConfig::remove(&id);
}

pub fn main_has_hwcodec() -> SyncReturn<bool> {
    SyncReturn(has_hwcodec())
}

pub fn main_supported_hwdecodings() -> SyncReturn<String> {
    let decoding = supported_hwdecodings();
    let msg = HashMap::from([("h264", decoding.0), ("h265", decoding.1)]);

    SyncReturn(serde_json::ser::to_string(&msg).unwrap_or("".to_owned()))
}

pub fn main_is_root() -> bool {
    is_root()
}

pub fn get_double_click_time() -> SyncReturn<i32> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        return SyncReturn(crate::platform::get_double_click_time() as _);
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    SyncReturn(500i32)
}

pub fn main_start_dbus_server() {
    #[cfg(target_os = "linux")]
    {
        use crate::dbus::start_dbus_server;
        // spawn new thread to start dbus server
        std::thread::spawn(|| {
            let _ = start_dbus_server();
        });
    }
}

pub fn session_send_mouse(id: String, msg: String) {
    if let Ok(m) = serde_json::from_str::<HashMap<String, String>>(&msg) {
        let alt = m.get("alt").is_some();
        let ctrl = m.get("ctrl").is_some();
        let shift = m.get("shift").is_some();
        let command = m.get("command").is_some();
        let x = m
            .get("x")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);
        let y = m
            .get("y")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);
        let mut mask = 0;
        if let Some(_type) = m.get("type") {
            mask = match _type.as_str() {
                "down" => 1,
                "up" => 2,
                "wheel" => 3,
                "trackpad" => 4,
                _ => 0,
            };
        }
        if let Some(buttons) = m.get("buttons") {
            mask |= match buttons.as_str() {
                "left" => 0x01,
                "right" => 0x02,
                "wheel" => 0x04,
                "back" => 0x08,
                "forward" => 0x10,
                _ => 0,
            } << 3;
        }
        if let Some(session) = SESSIONS.read().unwrap().get(&id) {
            session.send_mouse(mask, x, y, alt, ctrl, shift, command);
        }
    }
}

pub fn session_restart_remote_device(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.restart_remote_device();
    }
}

pub fn session_get_audit_server_sync(id: String, typ: String) -> SyncReturn<String> {
    let res = if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.get_audit_server(typ)
    } else {
        "".to_owned()
    };
    SyncReturn(res)
}

pub fn session_send_note(id: String, note: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.send_note(note)
    }
}

pub fn session_alternative_codecs(id: String) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        let (vp8, h264, h265) = session.alternative_codecs();
        let msg = HashMap::from([("vp8", vp8), ("h264", h264), ("h265", h265)]);
        serde_json::ser::to_string(&msg).unwrap_or("".to_owned())
    } else {
        String::new()
    }
}

pub fn session_change_prefer_codec(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.change_prefer_codec();
    }
}

pub fn main_set_home_dir(_home: String) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        *config::APP_HOME_DIR.write().unwrap() = _home;
    }
}

// This is a temporary method to get data dir for ios
pub fn main_get_data_dir_ios() -> SyncReturn<String> {
    let data_dir = config::Config::path("data");
    if !data_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            log::warn!("Failed to create data dir {}", e);
        }
    }
    SyncReturn(data_dir.to_string_lossy().to_string())
}

pub fn main_stop_service() {
    #[cfg(target_os = "android")]
    {
        config::Config::set_option("stop-service".into(), "Y".into());
        crate::rendezvous_mediator::RendezvousMediator::restart();
    }
}

pub fn main_start_service() {
    #[cfg(target_os = "android")]
    {
        config::Config::set_option("stop-service".into(), "".into());
        crate::rendezvous_mediator::RendezvousMediator::restart();
    }
}

pub fn main_update_temporary_password() {
    update_temporary_password();
}

pub fn main_set_permanent_password(password: String) {
    set_permanent_password(password);
}

pub fn main_check_super_user_permission() -> bool {
    check_super_user_permission()
}

pub fn main_check_mouse_time() {
    check_mouse_time();
}

pub fn main_get_mouse_time() -> f64 {
    get_mouse_time()
}

pub fn main_wol(id: String) {
    crate::lan::send_wol(id)
}

pub fn main_create_shortcut(_id: String) {
    #[cfg(windows)]
    create_shortcut(_id);
}

pub fn cm_send_chat(conn_id: i32, msg: String) {
    crate::ui_cm_interface::send_chat(conn_id, msg);
}

pub fn cm_login_res(conn_id: i32, res: bool) {
    if res {
        crate::ui_cm_interface::authorize(conn_id);
    } else {
        crate::ui_cm_interface::close(conn_id);
    }
}

pub fn cm_close_connection(conn_id: i32) {
    crate::ui_cm_interface::close(conn_id);
}

pub fn cm_remove_disconnected_connection(conn_id: i32) {
    crate::ui_cm_interface::remove(conn_id);
}

pub fn cm_check_click_time(conn_id: i32) {
    crate::ui_cm_interface::check_click_time(conn_id)
}

pub fn cm_get_click_time() -> f64 {
    crate::ui_cm_interface::get_click_time() as _
}

pub fn cm_switch_permission(conn_id: i32, name: String, enabled: bool) {
    crate::ui_cm_interface::switch_permission(conn_id, name, enabled)
}

pub fn cm_can_elevate() -> SyncReturn<bool> {
    SyncReturn(crate::ui_cm_interface::can_elevate())
}

pub fn cm_elevate_portable(conn_id: i32) {
    crate::ui_cm_interface::elevate_portable(conn_id);
}

pub fn cm_switch_back(conn_id: i32) {
    crate::ui_cm_interface::switch_back(conn_id);
}

pub fn main_get_build_date() -> String {
    crate::BUILD_DATE.to_string()
}

#[no_mangle]
unsafe extern "C" fn translate(name: *const c_char, locale: *const c_char) -> *const c_char {
    let name = CStr::from_ptr(name);
    let locale = CStr::from_ptr(locale);
    let res = if let (Ok(name), Ok(locale)) = (name.to_str(), locale.to_str()) {
        crate::client::translate_locale(name.to_owned(), locale)
    } else {
        String::new()
    };
    CString::from_vec_unchecked(res.into_bytes()).into_raw()
}

fn handle_query_onlines(onlines: Vec<String>, offlines: Vec<String>) {
    if let Some(s) = flutter::GLOBAL_EVENT_STREAM
        .read()
        .unwrap()
        .get(flutter::APP_TYPE_MAIN)
    {
        let data = HashMap::from([
            ("name", "callback_query_onlines".to_owned()),
            ("onlines", onlines.join(",")),
            ("offlines", offlines.join(",")),
        ]);
        s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
    };
}

pub fn query_onlines(ids: Vec<String>) {
    crate::rendezvous_mediator::query_online_states(ids, handle_query_onlines)
}

pub fn version_to_number(v: String) -> SyncReturn<i64> {
    SyncReturn(hbb_common::get_version_number(&v))
}

pub fn option_synced() -> bool {
    crate::ui_interface::option_synced()
}

pub fn main_is_installed() -> SyncReturn<bool> {
    SyncReturn(is_installed())
}

pub fn main_start_grab_keyboard() -> SyncReturn<bool> {
    #[cfg(target_os = "linux")]
    if !*crate::common::IS_X11 {
        return SyncReturn(false);
    }
    crate::keyboard::client::start_grab_loop();
    if !is_can_input_monitoring(false) {
        return SyncReturn(false);
    }
    SyncReturn(true)
}

pub fn main_is_installed_lower_version() -> SyncReturn<bool> {
    SyncReturn(is_installed_lower_version())
}

pub fn main_is_installed_daemon(prompt: bool) -> SyncReturn<bool> {
    SyncReturn(is_installed_daemon(prompt))
}

pub fn main_is_process_trusted(prompt: bool) -> SyncReturn<bool> {
    SyncReturn(is_process_trusted(prompt))
}

pub fn main_is_can_screen_recording(prompt: bool) -> SyncReturn<bool> {
    SyncReturn(is_can_screen_recording(prompt))
}

pub fn main_is_can_input_monitoring(prompt: bool) -> SyncReturn<bool> {
    SyncReturn(is_can_input_monitoring(prompt))
}

pub fn main_is_share_rdp() -> SyncReturn<bool> {
    SyncReturn(is_share_rdp())
}

pub fn main_is_rdp_service_open() -> SyncReturn<bool> {
    SyncReturn(is_rdp_service_open())
}

pub fn main_set_share_rdp(enable: bool) {
    set_share_rdp(enable)
}

pub fn main_goto_install() -> SyncReturn<bool> {
    goto_install();
    SyncReturn(true)
}

pub fn main_get_new_version() -> SyncReturn<String> {
    SyncReturn(get_new_version())
}

pub fn main_update_me() -> SyncReturn<bool> {
    update_me("".to_owned());
    SyncReturn(true)
}

pub fn set_cur_session_id(id: String) {
    super::flutter::set_cur_session_id(id);
    #[cfg(windows)]
    crate::keyboard::update_grab_get_key_name();
}

pub fn install_show_run_without_install() -> SyncReturn<bool> {
    SyncReturn(show_run_without_install())
}

pub fn install_run_without_install() {
    run_without_install();
}

pub fn install_install_me(options: String, path: String) {
    install_me(options, path, false, false);
}

pub fn install_install_path() -> SyncReturn<String> {
    SyncReturn(install_path())
}

pub fn main_account_auth(op: String) {
    let id = get_id();
    let uuid = get_uuid();
    account_auth(op, id, uuid);
}

pub fn main_account_auth_cancel() {
    account_auth_cancel()
}

pub fn main_account_auth_result() -> String {
    account_auth_result()
}

pub fn main_on_main_window_close() {
    // may called more than one times
    #[cfg(windows)]
    crate::portable_service::client::drop_portable_service_shared_memory();
}

pub fn main_current_is_wayland() -> SyncReturn<bool> {
    SyncReturn(current_is_wayland())
}

pub fn main_is_login_wayland() -> SyncReturn<bool> {
    SyncReturn(is_login_wayland())
}

pub fn main_start_pa() {
    #[cfg(target_os = "linux")]
    std::thread::spawn(crate::ipc::start_pa);
}

pub fn main_hide_docker() -> SyncReturn<bool> {
    #[cfg(target_os = "macos")]
    crate::platform::macos::hide_dock();
    SyncReturn(true)
}

pub fn main_use_texture_render() -> SyncReturn<bool> {
    #[cfg(not(feature = "flutter_texture_render"))]
    {
        SyncReturn(false)
    }
    #[cfg(feature = "flutter_texture_render")]
    {
        SyncReturn(true)
    }
}

pub fn cm_start_listen_ipc_thread() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    crate::flutter::connection_manager::start_listen_ipc_thread();
}

/// Start an ipc server for receiving the url scheme.
///
/// * Should only be called in the main flutter window.
/// * macOS only
pub fn main_start_ipc_url_server() {
    #[cfg(target_os = "macos")]
    std::thread::spawn(move || crate::server::start_ipc_url_server());
}

/// Send a url scheme throught the ipc.
///
/// * macOS only
#[allow(unused_variables)]
pub fn send_url_scheme(_url: String) {
    #[cfg(target_os = "macos")]
    std::thread::spawn(move || crate::handle_url_scheme(_url));
}

#[cfg(target_os = "android")]
pub mod server_side {
    use hbb_common::{config, log};
    use jni::{
        objects::{JClass, JString},
        sys::jstring,
        JNIEnv,
    };

    use crate::start_server;

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_startServer(
        env: JNIEnv,
        _class: JClass,
        app_dir: JString,
    ) {
        log::debug!("startServer from jvm");
        if let Ok(app_dir) = env.get_string(app_dir) {
            *config::APP_DIR.write().unwrap() = app_dir.into();
        }
        std::thread::spawn(move || start_server(true));
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_startService(
        _env: JNIEnv,
        _class: JClass,
    ) {
        log::debug!("startService from jvm");
        config::Config::set_option("stop-service".into(), "".into());
        crate::rendezvous_mediator::RendezvousMediator::restart();
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_translateLocale(
        env: JNIEnv,
        _class: JClass,
        locale: JString,
        input: JString,
    ) -> jstring {
        let res = if let (Ok(input), Ok(locale)) = (env.get_string(input), env.get_string(locale)) {
            let input: String = input.into();
            let locale: String = locale.into();
            crate::client::translate_locale(input, &locale)
        } else {
            "".into()
        };
        return env.new_string(res).unwrap_or(input).into_inner();
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_refreshScreen(
        _env: JNIEnv,
        _class: JClass,
    ) {
        crate::server::video_service::refresh()
    }
}
