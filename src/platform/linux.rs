use super::{CursorData, ResultType};
use desktop::Desktop;
pub use hbb_common::platform::linux::*;
use hbb_common::{
    allow_err, bail,
    libc::{c_char, c_int, c_long, c_void},
    log,
    message_proto::Resolution,
    regex::{Captures, Regex},
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

type Xdo = *const c_void;

pub const PA_SAMPLE_RATE: u32 = 48000;
static mut UNMODIFIED: bool = true;

thread_local! {
    static XDO: RefCell<Xdo> = RefCell::new(unsafe { xdo_new(std::ptr::null()) });
    static DISPLAY: RefCell<*mut c_void> = RefCell::new(unsafe { XOpenDisplay(std::ptr::null())});
}

extern "C" {
    fn xdo_get_mouse_location(
        xdo: Xdo,
        x: *mut c_int,
        y: *mut c_int,
        screen_num: *mut c_int,
    ) -> c_int;
    fn xdo_new(display: *const c_char) -> Xdo;
}

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> *mut c_void;
    // fn XCloseDisplay(d: *mut c_void) -> c_int;
}

#[link(name = "Xfixes")]
extern "C" {
    // fn XFixesQueryExtension(dpy: *mut c_void, event: *mut c_int, error: *mut c_int) -> c_int;
    fn XFixesGetCursorImage(dpy: *mut c_void) -> *const xcb_xfixes_get_cursor_image;
    fn XFree(data: *mut c_void);
}

// /usr/include/X11/extensions/Xfixes.h
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: c_long,
    pub pixels: *const c_long,
}

#[inline]
fn sleep_millis(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

pub fn get_cursor_pos() -> Option<(i32, i32)> {
    let mut res = None;
    XDO.with(|xdo| {
        if let Ok(xdo) = xdo.try_borrow_mut() {
            if xdo.is_null() {
                return;
            }
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            unsafe {
                xdo_get_mouse_location(*xdo, &mut x as _, &mut y as _, std::ptr::null_mut());
            }
            res = Some((x, y));
        }
    });
    res
}

pub fn reset_input_cache() {}

pub fn get_cursor() -> ResultType<Option<u64>> {
    let mut res = None;
    DISPLAY.with(|conn| {
        if let Ok(d) = conn.try_borrow_mut() {
            if !d.is_null() {
                unsafe {
                    let img = XFixesGetCursorImage(*d);
                    if !img.is_null() {
                        res = Some((*img).cursor_serial as u64);
                        XFree(img as _);
                    }
                }
            }
        }
    });
    Ok(res)
}

pub fn get_cursor_data(hcursor: u64) -> ResultType<CursorData> {
    let mut res = None;
    DISPLAY.with(|conn| {
        if let Ok(ref mut d) = conn.try_borrow_mut() {
            if !d.is_null() {
                unsafe {
                    let img = XFixesGetCursorImage(**d);
                    if !img.is_null() && hcursor == (*img).cursor_serial as u64 {
                        let mut cd: CursorData = Default::default();
                        cd.hotx = (*img).xhot as _;
                        cd.hoty = (*img).yhot as _;
                        cd.width = (*img).width as _;
                        cd.height = (*img).height as _;
                        // to-do: how about if it is 0
                        cd.id = (*img).cursor_serial as _;
                        let pixels =
                            std::slice::from_raw_parts((*img).pixels, (cd.width * cd.height) as _);
                        // cd.colors.resize(pixels.len() * 4, 0);
                        let mut cd_colors = vec![0_u8; pixels.len() * 4];
                        for y in 0..cd.height {
                            for x in 0..cd.width {
                                let pos = (y * cd.width + x) as usize;
                                let p = pixels[pos];
                                let a = (p >> 24) & 0xff;
                                let r = (p >> 16) & 0xff;
                                let g = (p >> 8) & 0xff;
                                let b = (p >> 0) & 0xff;
                                if a == 0 {
                                    continue;
                                }
                                let pos = pos * 4;
                                cd_colors[pos] = r as _;
                                cd_colors[pos + 1] = g as _;
                                cd_colors[pos + 2] = b as _;
                                cd_colors[pos + 3] = a as _;
                            }
                        }
                        cd.colors = cd_colors.into();
                        res = Some(cd);
                    }
                    if !img.is_null() {
                        XFree(img as _);
                    }
                }
            }
        }
    });
    match res {
        Some(x) => Ok(x),
        _ => bail!("Failed to get cursor image of {}", hcursor),
    }
}

fn start_uinput_service() {
    use crate::server::uinput::service;
    std::thread::spawn(|| {
        service::start_service_control();
    });
    std::thread::spawn(|| {
        service::start_service_keyboard();
    });
    std::thread::spawn(|| {
        service::start_service_mouse();
    });
}

#[inline]
fn try_start_server_(user: Option<(String, String)>) -> ResultType<Option<Child>> {
    if user.is_some() {
        run_as_user(vec!["--server"], user)
    } else {
        Ok(Some(crate::run_me(vec!["--server"])?))
    }
}

#[inline]
fn start_server(user: Option<(String, String)>, server: &mut Option<Child>) {
    match try_start_server_(user) {
        Ok(ps) => *server = ps,
        Err(err) => {
            log::error!("Failed to start server: {}", err);
        }
    }
}

fn stop_server(server: &mut Option<Child>) {
    if let Some(mut ps) = server.take() {
        allow_err!(ps.kill());
        sleep_millis(30);
        match ps.try_wait() {
            Ok(Some(_status)) => {}
            Ok(None) => {
                let _res = ps.wait();
            }
            Err(e) => log::error!("error attempting to wait: {e}"),
        }
    }
}

fn set_x11_env(desktop: &Desktop) {
    log::info!("DISPLAY: {}", desktop.display);
    log::info!("XAUTHORITY: {}", desktop.xauth);
    if !desktop.display.is_empty() {
        std::env::set_var("DISPLAY", &desktop.display);
    }
    if !desktop.xauth.is_empty() {
        std::env::set_var("XAUTHORITY", &desktop.xauth);
    }
}

#[inline]
fn stop_rustdesk_servers() {
    let _ = run_cmds(&format!(
        r##"ps -ef | grep -E 'rustdesk +--server' | awk '{{printf("kill -9 %d\n", $2)}}' | bash"##,
    ));
}

#[inline]
fn stop_subprocess() {
    let _ = run_cmds(&format!(
        r##"ps -ef | grep '/etc/rustdesk/xorg.conf' | grep -v grep | awk '{{printf("kill -9 %d\n", $2)}}' | bash"##,
    ));
    let _ = run_cmds(&format!(
        r##"ps -ef | grep -E 'rustdesk +--cm-no-ui' | grep -v grep | awk '{{printf("kill -9 %d\n", $2)}}' | bash"##,
    ));
}

fn should_start_server(
    try_x11: bool,
    uid: &mut String,
    desktop: &Desktop,
    cm0: &mut bool,
    last_restart: &mut Instant,
    server: &mut Option<Child>,
) -> bool {
    let cm = get_cm();
    let mut start_new = false;
    let mut should_kill = false;

    if desktop.is_headless() {
        if !uid.is_empty() {
            // From having a monitor to not having a monitor.
            *uid = "".to_owned();
            should_kill = true;
        }
    } else if desktop.uid != *uid && !desktop.uid.is_empty() {
        *uid = desktop.uid.clone();
        if try_x11 {
            set_x11_env(&desktop);
        }
        should_kill = true;
    }

    if !should_kill
        && !cm
        && ((*cm0 && last_restart.elapsed().as_secs() > 60)
            || last_restart.elapsed().as_secs() > 3600)
    {
        // restart server if new connections all closed, or every one hour,
        // as a workaround to resolve "SpotUdp" (dns resolve)
        // and x server get displays failure issue
        should_kill = true;
        log::info!("restart server");
    }

    if should_kill {
        if let Some(ps) = server.as_mut() {
            allow_err!(ps.kill());
            sleep_millis(30);
            *last_restart = Instant::now();
        }
    }

    if let Some(ps) = server.as_mut() {
        match ps.try_wait() {
            Ok(Some(_)) => {
                *server = None;
                start_new = true;
            }
            _ => {}
        }
    } else {
        start_new = true;
    }
    *cm0 = cm;
    start_new
}

// to-do: stop_server(&mut user_server); may not stop child correctly
// stop_rustdesk_servers() is just a temp solution here.
fn force_stop_server() {
    stop_rustdesk_servers();
    sleep_millis(super::SERVICE_INTERVAL);
}

pub fn start_os_service() {
    stop_rustdesk_servers();
    stop_subprocess();
    start_uinput_service();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut desktop = Desktop::default();
    let mut sid = "".to_owned();
    let mut uid = "".to_owned();
    let mut server: Option<Child> = None;
    let mut user_server: Option<Child> = None;
    if let Err(err) = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }) {
        println!("Failed to set Ctrl-C handler: {}", err);
    }

    let mut cm0 = false;
    let mut last_restart = Instant::now();
    while running.load(Ordering::SeqCst) {
        desktop.refresh();

        // Duplicate logic here with should_start_server
        // Login wayland will try to start a headless --server.
        if desktop.username == "root" || !desktop.is_wayland() || desktop.is_login_wayland() {
            // try kill subprocess "--server"
            stop_server(&mut user_server);
            // try start subprocess "--server"
            if should_start_server(
                true,
                &mut uid,
                &desktop,
                &mut cm0,
                &mut last_restart,
                &mut server,
            ) {
                stop_subprocess();
                force_stop_server();
                start_server(None, &mut server);
            }
        } else if desktop.username != "" {
            // try kill subprocess "--server"
            stop_server(&mut server);

            // try start subprocess "--server"
            if should_start_server(
                false,
                &mut uid,
                &desktop,
                &mut cm0,
                &mut last_restart,
                &mut user_server,
            ) {
                stop_subprocess();
                force_stop_server();
                start_server(
                    Some((desktop.uid.clone(), desktop.username.clone())),
                    &mut user_server,
                );
            }
        } else {
            force_stop_server();
            stop_server(&mut user_server);
            stop_server(&mut server);
        }

        let keeps_headless = sid.is_empty() && desktop.is_headless();
        let keeps_session = sid == desktop.sid;
        if keeps_headless || keeps_session {
            // for fixing https://github.com/rustdesk/rustdesk/issues/3129 to avoid too much dbus calling,
            sleep_millis(500);
        } else {
            sleep_millis(super::SERVICE_INTERVAL);
        }
        if !desktop.is_headless() {
            sid = desktop.sid.clone();
        }
    }

    if let Some(ps) = user_server.take().as_mut() {
        allow_err!(ps.kill());
    }
    if let Some(ps) = server.take().as_mut() {
        allow_err!(ps.kill());
    }
    log::info!("Exit");
}

#[inline]
pub fn get_active_user_id_name() -> (String, String) {
    let vec_id_name = get_values_of_seat0(&[1, 2]);
    (vec_id_name[0].clone(), vec_id_name[1].clone())
}

#[inline]
pub fn get_active_userid() -> String {
    get_values_of_seat0(&[1])[0].clone()
}

fn get_cm() -> bool {
    if let Ok(output) = Command::new("ps").args(vec!["aux"]).output() {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.contains(&format!(
                "{} --cm",
                std::env::current_exe()
                    .unwrap_or("".into())
                    .to_string_lossy()
            )) {
                return true;
            }
        }
    }
    false
}

pub fn is_login_wayland() -> bool {
    if let Ok(contents) = std::fs::read_to_string("/etc/gdm3/custom.conf") {
        contents.contains("#WaylandEnable=false") || contents.contains("WaylandEnable=true")
    } else if let Ok(contents) = std::fs::read_to_string("/etc/gdm/custom.conf") {
        contents.contains("#WaylandEnable=false") || contents.contains("WaylandEnable=true")
    } else {
        false
    }
}

#[inline]
pub fn current_is_wayland() -> bool {
    return is_desktop_wayland() && unsafe { UNMODIFIED };
}

// to-do: test the other display manager
fn _get_display_manager() -> String {
    if let Ok(x) = std::fs::read_to_string("/etc/X11/default-display-manager") {
        if let Some(x) = x.split("/").last() {
            return x.to_owned();
        }
    }
    "gdm3".to_owned()
}

#[inline]
pub fn get_active_username() -> String {
    get_values_of_seat0(&[2])[0].clone()
}

pub fn get_active_user_home() -> Option<PathBuf> {
    let username = get_active_username();
    if !username.is_empty() {
        let home = PathBuf::from(format!("/home/{}", username));
        if home.exists() {
            return Some(home);
        }
    }
    None
}

pub fn get_env_var(k: &str) -> String {
    match std::env::var(k) {
        Ok(v) => v,
        Err(_e) => "".to_owned(),
    }
}

// Headless is enabled, always return true.
pub fn is_prelogin() -> bool {
    let n = get_active_userid().len();
    n < 4 && n > 1
}

pub fn is_root() -> bool {
    crate::username() == "root"
}

fn is_opensuse() -> bool {
    if let Ok(res) = run_cmds("cat /etc/os-release | grep opensuse") {
        if !res.is_empty() {
            return true;
        }
    }
    false
}

pub fn run_as_user(arg: Vec<&str>, user: Option<(String, String)>) -> ResultType<Option<Child>> {
    let (uid, username) = match user {
        Some(id_name) => id_name,
        None => get_active_user_id_name(),
    };
    let cmd = std::env::current_exe()?;
    if uid.is_empty() {
        bail!("No valid uid");
    }
    let xdg = &format!("XDG_RUNTIME_DIR=/run/user/{}", uid) as &str;
    let mut args = vec![xdg, "-u", &username, cmd.to_str().unwrap_or("")];
    args.append(&mut arg.clone());
    // -E required for opensuse
    if is_opensuse() {
        args.insert(0, "-E");
    }

    let task = Command::new("sudo").args(args).spawn()?;
    Ok(Some(task))
}

pub fn get_pa_monitor() -> String {
    get_pa_sources()
        .drain(..)
        .map(|x| x.0)
        .filter(|x| x.contains("monitor"))
        .next()
        .unwrap_or("".to_owned())
}

pub fn get_pa_source_name(desc: &str) -> String {
    get_pa_sources()
        .drain(..)
        .filter(|x| x.1 == desc)
        .map(|x| x.0)
        .next()
        .unwrap_or("".to_owned())
}

pub fn get_pa_sources() -> Vec<(String, String)> {
    use pulsectl::controllers::*;
    let mut out = Vec::new();
    match SourceController::create() {
        Ok(mut handler) => {
            if let Ok(devices) = handler.list_devices() {
                for dev in devices.clone() {
                    out.push((
                        dev.name.unwrap_or("".to_owned()),
                        dev.description.unwrap_or("".to_owned()),
                    ));
                }
            }
        }
        Err(err) => {
            log::error!("Failed to get_pa_sources: {:?}", err);
        }
    }
    out
}

pub fn get_default_pa_source() -> Option<(String, String)> {
    use pulsectl::controllers::*;
    match SourceController::create() {
        Ok(mut handler) => {
            if let Ok(dev) = handler.get_default_device() {
                return Some((
                    dev.name.unwrap_or("".to_owned()),
                    dev.description.unwrap_or("".to_owned()),
                ));
            }
        }
        Err(err) => {
            log::error!("Failed to get_pa_source: {:?}", err);
        }
    }
    None
}

pub fn lock_screen() {
    Command::new("xdg-screensaver").arg("lock").spawn().ok();
}

pub fn toggle_blank_screen(_v: bool) {
    // https://unix.stackexchange.com/questions/17170/disable-keyboard-mouse-input-on-unix-under-x
}

pub fn block_input(_v: bool) -> bool {
    true
}

pub fn is_installed() -> bool {
    true
}

pub(super) fn get_env_tries(name: &str, uid: &str, process: &str, n: usize) -> String {
    for _ in 0..n {
        let x = get_env(name, uid, process);
        if !x.is_empty() {
            return x;
        }
        sleep_millis(300);
    }
    "".to_owned()
}

#[inline]
fn get_env(name: &str, uid: &str, process: &str) -> String {
    let cmd = format!("ps -u {} -f | grep '{}' | grep -v 'grep' | tail -1 | awk '{{print $2}}' | xargs -I__ cat /proc/__/environ 2>/dev/null | tr '\\0' '\\n' | grep '^{}=' | tail -1 | sed 's/{}=//g'", uid, process, name, name);
    if let Ok(x) = run_cmds(&cmd) {
        x.trim_end().to_string()
    } else {
        "".to_owned()
    }
}

#[inline]
fn get_env_from_pid(name: &str, pid: &str) -> String {
    let cmd = format!("cat /proc/{}/environ 2>/dev/null | tr '\\0' '\\n' | grep '^{}=' | tail -1 | sed 's/{}=//g'", pid, name, name);
    if let Ok(x) = run_cmds(&cmd) {
        x.trim_end().to_string()
    } else {
        "".to_owned()
    }
}

#[link(name = "gtk-3")]
extern "C" {
    fn gtk_main_quit();
}

pub fn quit_gui() {
    unsafe { gtk_main_quit() };
}

pub fn check_super_user_permission() -> ResultType<bool> {
    let file = "/usr/share/rustdesk/files/polkit";
    let arg;
    if Path::new(file).is_file() {
        arg = file;
    } else {
        arg = "echo";
    }
    let status = Command::new("pkexec").arg(arg).status()?;
    Ok(status.success() && status.code() == Some(0))
}

type GtkSettingsPtr = *mut c_void;
type GObjectPtr = *mut c_void;
#[link(name = "gtk-3")]
extern "C" {
    // fn gtk_init(argc: *mut c_int, argv: *mut *mut c_char);
    fn gtk_settings_get_default() -> GtkSettingsPtr;
}

#[link(name = "gobject-2.0")]
extern "C" {
    fn g_object_get(object: GObjectPtr, first_property_name: *const c_char, ...);
}

pub fn get_double_click_time() -> u32 {
    // GtkSettings *settings = gtk_settings_get_default ();
    // g_object_get (settings, "gtk-double-click-time", &double_click_time, NULL);
    unsafe {
        let mut double_click_time = 0u32;
        let property = std::ffi::CString::new("gtk-double-click-time").unwrap();
        let settings = gtk_settings_get_default();
        g_object_get(
            settings,
            property.as_ptr(),
            &mut double_click_time as *mut u32,
            0 as *const c_void,
        );
        double_click_time
    }
}

#[inline]
fn get_width_height_from_captures<'t>(caps: &Captures<'t>) -> Option<(i32, i32)> {
    match (caps.name("width"), caps.name("height")) {
        (Some(width), Some(height)) => {
            match (
                width.as_str().parse::<i32>(),
                height.as_str().parse::<i32>(),
            ) {
                (Ok(width), Ok(height)) => {
                    return Some((width, height));
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}

#[inline]
fn get_xrandr_conn_pat(name: &str) -> String {
    format!(
        r"{}\s+connected.+?(?P<width>\d+)x(?P<height>\d+)\+(?P<x>\d+)\+(?P<y>\d+).*?\n",
        name
    )
}

pub fn resolutions(name: &str) -> Vec<Resolution> {
    let resolutions_pat = r"(?P<resolutions>(\s*\d+x\d+\s+\d+.*\n)+)";
    let connected_pat = get_xrandr_conn_pat(name);
    let mut v = vec![];
    if let Ok(re) = Regex::new(&format!("{}{}", connected_pat, resolutions_pat)) {
        match run_cmds("xrandr --query | tr -s ' '") {
            Ok(xrandr_output) => {
                // There'are different kinds of xrandr output.
                /*
                1.
                Screen 0: minimum 320 x 175, current 1920 x 1080, maximum 1920 x 1080
                default connected 1920x1080+0+0 0mm x 0mm
                 1920x1080 10.00*
                 1280x720 25.00
                 1680x1050 60.00
                Virtual2 disconnected (normal left inverted right x axis y axis)
                Virtual3 disconnected (normal left inverted right x axis y axis)

                XWAYLAND0 connected primary 1920x984+0+0 (normal left inverted right x axis y axis) 0mm x 0mm
                Virtual1 connected primary 1920x984+0+0 (normal left inverted right x axis y axis) 0mm x 0mm
                HDMI-0 connected (normal left inverted right x axis y axis)

                rdp0 connected primary 1920x1080+0+0 0mm x 0mm
                    */
                if let Some(caps) = re.captures(&xrandr_output) {
                    if let Some(resolutions) = caps.name("resolutions") {
                        let resolution_pat =
                            r"\s*(?P<width>\d+)x(?P<height>\d+)\s+(?P<rates>(\d+\.\d+[* ]*)+)\s*\n";
                        let resolution_re = Regex::new(&format!(r"{}", resolution_pat)).unwrap();
                        for resolution_caps in resolution_re.captures_iter(resolutions.as_str()) {
                            if let Some((width, height)) =
                                get_width_height_from_captures(&resolution_caps)
                            {
                                let resolution = Resolution {
                                    width,
                                    height,
                                    ..Default::default()
                                };
                                if !v.contains(&resolution) {
                                    v.push(resolution);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => log::error!("Failed to run xrandr query, {}", e),
        }
    }

    v
}

pub fn current_resolution(name: &str) -> ResultType<Resolution> {
    let xrandr_output = run_cmds("xrandr --query | tr -s ' '")?;
    let re = Regex::new(&get_xrandr_conn_pat(name))?;
    if let Some(caps) = re.captures(&xrandr_output) {
        if let Some((width, height)) = get_width_height_from_captures(&caps) {
            return Ok(Resolution {
                width,
                height,
                ..Default::default()
            });
        }
    }
    bail!("Failed to find current resolution for {}", name);
}

pub fn change_resolution(name: &str, width: usize, height: usize) -> ResultType<()> {
    Command::new("xrandr")
        .args(vec![
            "--output",
            name,
            "--mode",
            &format!("{}x{}", width, height),
        ])
        .spawn()?;
    Ok(())
}

mod desktop {
    use super::*;

    pub const XFCE4_PANEL: &str = "xfce4-panel";
    pub const GNOME_SESSION_BINARY: &str = "gnome-session-binary";

    #[derive(Debug, Clone, Default)]
    pub struct Desktop {
        pub sid: String,
        pub username: String,
        pub uid: String,
        pub protocal: String,
        pub display: String,
        pub xauth: String,
        pub is_rustdesk_subprocess: bool,
    }

    impl Desktop {
        #[inline]
        pub fn is_wayland(&self) -> bool {
            self.protocal == DISPLAY_SERVER_WAYLAND
        }

        #[inline]
        pub fn is_login_wayland(&self) -> bool {
            super::is_gdm_user(&self.username) && self.protocal == DISPLAY_SERVER_WAYLAND
        }

        #[inline]
        pub fn is_headless(&self) -> bool {
            self.sid.is_empty() || self.is_rustdesk_subprocess
        }

        fn get_display(&mut self) {
            self.display = get_env_tries("DISPLAY", &self.uid, GNOME_SESSION_BINARY, 10);
            if self.display.is_empty() {
                self.display = get_env_tries("DISPLAY", &self.uid, XFCE4_PANEL, 10);
            }
            if self.display.is_empty() {
                self.display = Self::get_display_by_user(&self.username);
            }
            if self.display.is_empty() {
                self.display = ":0".to_owned();
            }
            self.display = self
                .display
                .replace(&whoami::hostname(), "")
                .replace("localhost", "");
        }

        fn get_xauth_from_xorg(&mut self) {
            if let Ok(output) = run_cmds(&format!(
                "ps -u {} -f | grep 'Xorg' | grep -v 'grep'",
                &self.uid
            )) {
                for line in output.lines() {
                    let mut auth_found = false;
                    for v in line.split_whitespace() {
                        if v == "-auth" {
                            auth_found = true;
                        } else if auth_found {
                            if std::path::Path::new(v).is_absolute() {
                                self.xauth = v.to_string();
                            } else {
                                if let Some(pid) = line.split_whitespace().nth(1) {
                                    let home_dir = get_env_from_pid("HOME", pid);
                                    if home_dir.is_empty() {
                                        self.xauth = format!("/home/{}/{}", self.username, v);
                                    } else {
                                        self.xauth = format!("{}/{}", home_dir, v);
                                    }
                                } else {
                                    // unreachable!
                                }
                            }
                            return;
                        }
                    }
                }
            }
        }

        fn get_xauth(&mut self) {
            self.xauth = get_env_tries("XAUTHORITY", &self.uid, GNOME_SESSION_BINARY, 10);
            if self.xauth.is_empty() {
                get_env_tries("XAUTHORITY", &self.uid, XFCE4_PANEL, 10);
            }
            if self.xauth.is_empty() {
                self.get_xauth_from_xorg();
            }

            let gdm = format!("/run/user/{}/gdm/Xauthority", self.uid);
            if self.xauth.is_empty() {
                self.xauth = if std::path::Path::new(&gdm).exists() {
                    gdm
                } else {
                    let username = &self.username;
                    if username == "root" {
                        format!("/{}/.Xauthority", username)
                    } else {
                        let tmp = format!("/home/{}/.Xauthority", username);
                        if std::path::Path::new(&tmp).exists() {
                            tmp
                        } else {
                            format!("/var/lib/{}/.Xauthority", username)
                        }
                    }
                };
            }
        }

        fn get_display_by_user(user: &str) -> String {
            // log::debug!("w {}", &user);
            if let Ok(output) = std::process::Command::new("w").arg(&user).output() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let mut iter = line.split_whitespace();
                    let b = iter.nth(2);
                    if let Some(b) = b {
                        if b.starts_with(":") {
                            return b.to_owned();
                        }
                    }
                }
            }
            // above not work for gdm user
            //log::debug!("ls -l /tmp/.X11-unix/");
            let mut last = "".to_owned();
            if let Ok(output) = std::process::Command::new("ls")
                .args(vec!["-l", "/tmp/.X11-unix/"])
                .output()
            {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let mut iter = line.split_whitespace();
                    let user_field = iter.nth(2);
                    if let Some(x) = iter.last() {
                        if x.starts_with("X") {
                            last = x.replace("X", ":").to_owned();
                            if user_field == Some(&user) {
                                return last;
                            }
                        }
                    }
                }
            }
            last
        }

        fn set_is_subprocess(&mut self) {
            self.is_rustdesk_subprocess = false;
            let cmd = "ps -ef | grep 'rustdesk/xorg.conf' | grep -v grep | wc -l";
            if let Ok(res) = run_cmds(cmd) {
                if res.trim() != "0" {
                    self.is_rustdesk_subprocess = true;
                }
            }
        }

        pub fn refresh(&mut self) {
            if !self.sid.is_empty() && is_active(&self.sid) {
                return;
            }

            let seat0_values = get_values_of_seat0(&[0, 1, 2]);
            if seat0_values[0].is_empty() {
                *self = Self::default();
                self.is_rustdesk_subprocess = false;
                return;
            }

            self.sid = seat0_values[0].clone();
            self.uid = seat0_values[1].clone();
            self.username = seat0_values[2].clone();
            self.protocal = get_display_server_of_session(&self.sid).into();
            if self.is_login_wayland() {
                self.display = "".to_owned();
                self.xauth = "".to_owned();
                self.is_rustdesk_subprocess = false;
                return;
            }

            self.get_display();
            self.get_xauth();
            self.set_is_subprocess();
        }
    }
}
