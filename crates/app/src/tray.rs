//! Icon next to the clock: the app keeps running there when its window is
//! closed, like other download managers.
//!
//! Windows uses the `tray-icon` crate (Win32 shell notifications); Linux
//! speaks the freedesktop StatusNotifierItem protocol directly through
//! `ksni` — pure D-Bus, so no GTK and no extra system libraries are needed.
//! It works out of the box on KDE Plasma, XFCE, Cinnamon, MATE and LXQt
//! (the desktops of Manjaro and most other distributions).

#[cfg_attr(not(windows), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrayAction {
    Show,
    AddUrl,
    ResumeAll,
    StopAll,
    ToggleLimiter,
    Scheduler,
    Options,
    About,
    Exit,
}

/// What was clicked on the small icon of a hidden download window.
#[cfg_attr(not(windows), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DownloadAction {
    Show,
    Pause,
}

/// 32 x 32 RGBA picture for a hidden download: a blue "E" with a green
/// bar under it showing how far it is (`None`: size unknown, no bar).
#[cfg_attr(not(windows), allow(dead_code))]
pub fn download_icon(progress: Option<f32>) -> Vec<u8> {
    const N: usize = 32;
    let mut px = vec![0u8; N * N * 4];
    let mut put = |x: usize, y: usize, c: [u8; 4]| {
        let i = (y * N + x) * 4;
        px[i..i + 4].copy_from_slice(&c);
    };
    let blue = [0x1f, 0x6f, 0xeb, 0xff];
    let white = [0xff, 0xff, 0xff, 0xff];
    for y in 0..N {
        for x in 0..N {
            // a square with round corners (radius 6)
            let (dx, dy) = (x.min(N - 1 - x), y.min(N - 1 - y));
            let corner = dx < 6 && dy < 6 && {
                let (cx, cy) = (6.0 - dx as f32 - 0.5, 6.0 - dy as f32 - 0.5);
                cx * cx + cy * cy > 36.0
            };
            if !corner {
                put(x, y, blue);
            }
        }
    }
    let rect = |put: &mut dyn FnMut(usize, usize, [u8; 4]), x0: usize, y0: usize, x1: usize, y1: usize, c: [u8; 4]| {
        for y in y0..y1 {
            for x in x0..x1 {
                put(x, y, c);
            }
        }
    };
    // the letter
    rect(&mut put, 9, 4, 13, 22, white);
    rect(&mut put, 9, 4, 23, 8, white);
    rect(&mut put, 9, 11, 21, 15, white);
    rect(&mut put, 9, 18, 23, 22, white);
    if let Some(p) = progress {
        let p = p.clamp(0.0, 1.0);
        rect(&mut put, 3, 24, 29, 30, [0x0b, 0x2a, 0x5c, 0xff]);
        let end = 4 + ((24.0 * p).round() as usize);
        rect(&mut put, 4, 25, end, 29, [0x3d, 0xdc, 0x5b, 0xff]);
    }
    px
}

#[cfg(windows)]
mod imp {
    use super::{DownloadAction, TrayAction};
    use std::collections::HashMap;
    use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
    use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

    const MAIN_ID: &str = "ezoo-main";
    const DL_PREFIX: &str = "ezoo-dl-";

    /// The icon of one hidden download window.
    struct Mini {
        icon: TrayIcon,
        show: MenuId,
        pause: MenuId,
        /// what it shows now, to change it only when needed
        step: Option<u32>,
        tip: String,
    }

    thread_local! {
        static HIDDEN: std::cell::RefCell<HashMap<u64, Mini>> = std::cell::RefCell::new(HashMap::new());
        static PENDING: std::cell::RefCell<Vec<(u64, DownloadAction)>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    fn icon_for(progress: Option<f32>) -> Option<Icon> {
        Icon::from_rgba(super::download_icon(progress), 32, 32).ok()
    }

    /// A small icon next to the clock for a download whose window was hidden.
    pub fn show_download(id: u64, tip: &str, progress: Option<f32>, labels: (&str, &str)) -> bool {
        if HIDDEN.with(|h| h.borrow().contains_key(&id)) {
            return true;
        }
        let menu = Menu::new();
        let show = MenuItem::new(labels.0, true, None);
        let pause = MenuItem::new(labels.1, true, None);
        if menu.append(&show).is_err() || menu.append(&pause).is_err() {
            return false;
        }
        let Some(icon) = icon_for(progress) else { return false };
        let built = TrayIconBuilder::new()
            .with_id(format!("{DL_PREFIX}{id}"))
            .with_tooltip(super::short_tip(tip))
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .build();
        let Ok(icon) = built else { return false };
        let mini = Mini {
            icon,
            show: show.id().clone(),
            pause: pause.id().clone(),
            step: progress.map(super::step_of),
            tip: tip.to_string(),
        };
        HIDDEN.with(|h| h.borrow_mut().insert(id, mini));
        true
    }

    /// New numbers for its icon and its hint.
    pub fn update_download(id: u64, tip: &str, progress: Option<f32>) {
        HIDDEN.with(|h| {
            if let Some(m) = h.borrow_mut().get_mut(&id) {
                let step = progress.map(super::step_of);
                if step != m.step {
                    m.step = step;
                    let _ = m.icon.set_icon(icon_for(progress));
                }
                if m.tip != tip {
                    m.tip = tip.to_string();
                    let _ = m.icon.set_tooltip(Some(super::short_tip(tip)));
                }
            }
        });
    }

    /// Its window is back (or it is done): the icon goes.
    pub fn remove_download(id: u64) {
        HIDDEN.with(|h| h.borrow_mut().remove(&id));
    }

    /// Clicks on the icons of hidden downloads since the last call.
    pub fn take_download_actions() -> Vec<(u64, DownloadAction)> {
        PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()))
    }

    pub struct Tray {
        _icon: TrayIcon,
        limiter: CheckMenuItem,
        ids: Vec<(MenuId, fn() -> TrayAction)>,
    }

    impl Tray {
        pub fn new() -> Option<Tray> {
            let menu = Menu::new();
            let show = MenuItem::new("Restore Ezoo Download", true, None);
            let add = MenuItem::new("Add new download…", true, None);
            let resume = MenuItem::new("Resume all downloads", true, None);
            let stop = MenuItem::new("Stop all downloads", true, None);
            let limiter = CheckMenuItem::new("Speed Limiter", true, false, None);
            let sched = MenuItem::new("Scheduler…", true, None);
            let options = MenuItem::new("Options…", true, None);
            let about = MenuItem::new("About", true, None);
            let exit = MenuItem::new("Exit", true, None);
            menu.append(&show).ok()?;
            menu.append(&PredefinedMenuItem::separator()).ok()?;
            menu.append(&add).ok()?;
            menu.append(&resume).ok()?;
            menu.append(&stop).ok()?;
            menu.append(&PredefinedMenuItem::separator()).ok()?;
            menu.append(&limiter).ok()?;
            menu.append(&sched).ok()?;
            menu.append(&options).ok()?;
            menu.append(&PredefinedMenuItem::separator()).ok()?;
            menu.append(&about).ok()?;
            menu.append(&exit).ok()?;
            let icon = Icon::from_rgba(include_bytes!("../ui/tray32.rgba").to_vec(), 32, 32).ok()?;
            let tray = TrayIconBuilder::new()
                .with_id(MAIN_ID)
                .with_tooltip("Ezoo Download")
                .with_icon(icon)
                .with_menu(Box::new(menu))
                // the menu on a right click only; a double click opens the window
                .with_menu_on_left_click(false)
                .build()
                .ok()?;
            let ids: Vec<(MenuId, fn() -> TrayAction)> = vec![
                (show.id().clone(), || TrayAction::Show),
                (add.id().clone(), || TrayAction::AddUrl),
                (resume.id().clone(), || TrayAction::ResumeAll),
                (stop.id().clone(), || TrayAction::StopAll),
                (limiter.id().clone(), || TrayAction::ToggleLimiter),
                (sched.id().clone(), || TrayAction::Scheduler),
                (options.id().clone(), || TrayAction::Options),
                (about.id().clone(), || TrayAction::About),
                (exit.id().clone(), || TrayAction::Exit),
            ];
            Some(Tray { _icon: tray, limiter, ids })
        }

        /// Clicks since the last call (those on hidden downloads go to
        /// [`take_download_actions`]).
        pub fn poll(&self) -> Vec<TrayAction> {
            let mut out = Vec::new();
            while let Ok(ev) = TrayIconEvent::receiver().try_recv() {
                let id = ev.id().0.clone();
                if let Some(dl) = id.strip_prefix(DL_PREFIX).and_then(|n| n.parse::<u64>().ok()) {
                    // one click on a hidden download's icon: its window
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = ev {
                        PENDING.with(|p| p.borrow_mut().push((dl, DownloadAction::Show)));
                    }
                    continue;
                }
                // left double click: the window (the menu is on the right button)
                if let TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } = ev {
                    out.push(TrayAction::Show);
                }
            }
            while let Ok(ev) = MenuEvent::receiver().try_recv() {
                if let Some((_, make)) = self.ids.iter().find(|(id, _)| *id == ev.id) {
                    out.push(make());
                    continue;
                }
                let hit = HIDDEN.with(|h| {
                    h.borrow().iter().find_map(|(dl, m)| {
                        if ev.id == m.show {
                            Some((*dl, DownloadAction::Show))
                        } else if ev.id == m.pause {
                            Some((*dl, DownloadAction::Pause))
                        } else {
                            None
                        }
                    })
                });
                if let Some(hit) = hit {
                    PENDING.with(|p| p.borrow_mut().push(hit));
                }
            }
            out
        }

        /// Keep the check mark of "Speed Limiter" in step with the app.
        pub fn set_limiter(&self, on: bool) {
            if self.limiter.is_checked() != on {
                self.limiter.set_checked(on);
            }
        }
    }
}

#[cfg(unix)]
mod imp {
    use super::{step_of, short_tip, DownloadAction, TrayAction};
    use ksni::blocking::{Handle, TrayMethods};
    use ksni::menu::{CheckmarkItem, StandardItem};
    use ksni::{Icon, MenuItem, ToolTip};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex, OnceLock};

    /// Our RGBA picture as an ARGB32 `ksni::Icon` (the byte order the
    /// StatusNotifierItem protocol wants).
    fn ksni_icon(progress: Option<f32>) -> Icon {
        let mut data = super::download_icon(progress);
        for px in data.chunks_exact_mut(4) {
            px.rotate_right(1); // RGBA -> ARGB
        }
        Icon { width: 32, height: 32, data }
    }

    /// The tip starts with "67%  file.zip" (or "12 MB  file.zip"): the bold
    /// line of the hint is nicer as the plain file name.
    fn short_name(tip: &str) -> String {
        let first = tip.lines().next().unwrap_or("");
        match first.split_once("  ") {
            Some((_, name)) if !name.trim().is_empty() => name.trim().to_string(),
            _ => "Ezoo Download".to_string(),
        }
    }

    fn pending_main() -> &'static Mutex<Vec<TrayAction>> {
        static P: OnceLock<Mutex<Vec<TrayAction>>> = OnceLock::new();
        P.get_or_init(|| Mutex::new(Vec::new()))
    }

    fn pending_dl() -> &'static Mutex<Vec<(u64, DownloadAction)>> {
        static P: OnceLock<Mutex<Vec<(u64, DownloadAction)>>> = OnceLock::new();
        P.get_or_init(|| Mutex::new(Vec::new()))
    }

    /// The tray object itself. Its fields are read from the D-Bus service
    /// thread (menu, icon, hint), the clicks come back through the queue.
    struct MainTray {
        limiter: Arc<AtomicBool>,
    }

    impl ksni::Tray for MainTray {
        fn id(&self) -> String {
            "ezoo-download".into()
        }
        fn title(&self) -> String {
            "Ezoo Download".into()
        }
        fn icon_name(&self) -> String {
            "ezoo-download".into() // used when the theme has our icon
        }
        fn icon_pixmap(&self) -> Vec<Icon> {
            vec![ksni_icon(None)]
        }
        fn tool_tip(&self) -> ToolTip {
            ToolTip { title: "Ezoo Download".into(), ..Default::default() }
        }
        /// A plain click on the icon: the window comes back.
        fn activate(&mut self, _x: i32, _y: i32) {
            pending_main().lock().unwrap_or_else(|p| p.into_inner()).push(TrayAction::Show);
        }
        fn menu(&self) -> Vec<MenuItem<Self>> {
            let push = |a: TrayAction| {
                Box::new(move |_: &mut Self| {
                    pending_main().lock().unwrap_or_else(|p| p.into_inner()).push(a);
                }) as Box<dyn Fn(&mut Self) + Send>
            };
            vec![
                StandardItem { label: "Restore Ezoo Download".into(), activate: push(TrayAction::Show), ..Default::default() }.into(),
                MenuItem::Separator,
                StandardItem { label: "Add new download…".into(), activate: push(TrayAction::AddUrl), ..Default::default() }.into(),
                StandardItem { label: "Resume all downloads".into(), activate: push(TrayAction::ResumeAll), ..Default::default() }.into(),
                StandardItem { label: "Stop all downloads".into(), activate: push(TrayAction::StopAll), ..Default::default() }.into(),
                MenuItem::Separator,
                CheckmarkItem {
                    label: "Speed Limiter".into(),
                    checked: self.limiter.load(Ordering::Relaxed),
                    activate: push(TrayAction::ToggleLimiter),
                    ..Default::default()
                }
                .into(),
                StandardItem { label: "Scheduler…".into(), activate: push(TrayAction::Scheduler), ..Default::default() }.into(),
                StandardItem { label: "Options…".into(), activate: push(TrayAction::Options), ..Default::default() }.into(),
                MenuItem::Separator,
                StandardItem { label: "About".into(), activate: push(TrayAction::About), ..Default::default() }.into(),
                StandardItem { label: "Exit".into(), activate: push(TrayAction::Exit), ..Default::default() }.into(),
            ]
        }
    }

    /// One small icon for a download whose window was hidden.
    struct DlTray {
        id: u64,
        step: Option<u32>,
        name: String,
        tip: String,
        show_label: String,
        stop_label: String,
    }

    impl ksni::Tray for DlTray {
        fn id(&self) -> String {
            format!("ezoo-download-{}", self.id)
        }
        fn title(&self) -> String {
            self.name.clone()
        }
        fn icon_pixmap(&self) -> Vec<Icon> {
            vec![ksni_icon(self.step.map(|s| s as f32 / 24.0))]
        }
        fn tool_tip(&self) -> ToolTip {
            ToolTip {
                title: self.name.clone(),
                description: self.tip.clone(),
                ..Default::default()
            }
        }
        fn activate(&mut self, _x: i32, _y: i32) {
            let hit = (self.id, DownloadAction::Show);
            pending_dl().lock().unwrap_or_else(|p| p.into_inner()).push(hit);
        }
        fn menu(&self) -> Vec<MenuItem<Self>> {
            let id = self.id;
            let push = |a: DownloadAction| {
                Box::new(move |_: &mut Self| {
                    pending_dl().lock().unwrap_or_else(|p| p.into_inner()).push((id, a));
                }) as Box<dyn Fn(&mut Self) + Send>
            };
            vec![
                StandardItem { label: self.show_label.clone(), activate: push(DownloadAction::Show), ..Default::default() }.into(),
                StandardItem { label: self.stop_label.clone(), activate: push(DownloadAction::Pause), ..Default::default() }.into(),
            ]
        }
    }

    struct Mini {
        handle: Handle<DlTray>,
        step: Option<u32>,
        tip: String,
    }

    fn minis() -> &'static Mutex<HashMap<u64, Mini>> {
        static M: OnceLock<Mutex<HashMap<u64, Mini>>> = OnceLock::new();
        M.get_or_init(|| Mutex::new(HashMap::new()))
    }

    pub struct Tray {
        handle: Handle<MainTray>,
        limiter: Arc<AtomicBool>,
    }

    impl Tray {
        /// `None`: no working tray on this desktop (no D-Bus session, or no
        /// StatusNotifier host — bare GNOME for example). The app then simply
        /// quits when its window is closed, like before.
        pub fn new() -> Option<Tray> {
            let limiter = Arc::new(AtomicBool::new(false));
            let handle = MainTray { limiter: limiter.clone() }.spawn().ok()?;
            Some(Tray { handle, limiter })
        }

        /// Clicks since the last call (those on hidden downloads go to
        /// [`take_download_actions`]).
        pub fn poll(&self) -> Vec<TrayAction> {
            std::mem::take(&mut *pending_main().lock().unwrap_or_else(|p| p.into_inner()))
        }

        /// Keep the check mark of "Speed Limiter" in step with the app.
        pub fn set_limiter(&self, on: bool) {
            if self.limiter.swap(on, Ordering::Relaxed) != on {
                self.handle.update(|_| {}); // re-render the menu
            }
        }
    }

    /// A small icon next to the clock for a download whose window was hidden.
    pub fn show_download(id: u64, tip: &str, progress: Option<f32>, labels: (&str, &str)) -> bool {
        if minis().lock().unwrap_or_else(|p| p.into_inner()).contains_key(&id) {
            return true;
        }
        let tray = DlTray {
            id,
            step: progress.map(step_of),
            name: short_name(tip),
            tip: short_tip(tip),
            show_label: labels.0.to_string(),
            stop_label: labels.1.to_string(),
        };
        let Ok(handle) = tray.spawn() else { return false };
        minis().lock().unwrap_or_else(|p| p.into_inner()).insert(
            id,
            Mini { handle, step: progress.map(step_of), tip: short_tip(tip) },
        );
        true
    }

    /// New numbers for its icon and its hint.
    pub fn update_download(id: u64, tip: &str, progress: Option<f32>) {
        let step = progress.map(step_of);
        let tip = short_tip(tip);
        let mut ms = minis().lock().unwrap_or_else(|p| p.into_inner());
        if let Some(m) = ms.get_mut(&id) {
            if m.step != step || m.tip != tip {
                m.step = step;
                m.tip = tip;
                m.handle.update(|_| {}); // re-render icon and hint
            }
        }
    }

    /// Its window is back (or it is done): the icon goes.
    pub fn remove_download(id: u64) {
        if let Some(m) = minis().lock().unwrap_or_else(|p| p.into_inner()).remove(&id) {
            m.handle.shutdown(); // no wait: the service thread cleans up
        }
    }

    /// Clicks on the icons of hidden downloads since the last call.
    pub fn take_download_actions() -> Vec<(u64, DownloadAction)> {
        std::mem::take(&mut *pending_dl().lock().unwrap_or_else(|p| p.into_inner()))
    }
}

pub use imp::{remove_download, show_download, take_download_actions, update_download, Tray};

/// Windows shows at most 127 characters in an icon's hint.
#[cfg_attr(not(windows), allow(dead_code))]
fn short_tip(tip: &str) -> String {
    if tip.chars().count() <= 120 {
        tip.to_string()
    } else {
        tip.chars().take(119).chain(Some('…')).collect()
    }
}

/// The bar changes in steps of 1/24 (its width in dots).
#[cfg_attr(not(windows), allow(dead_code))]
fn step_of(p: f32) -> u32 {
    (p.clamp(0.0, 1.0) * 24.0).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_icon_shows_the_progress() {
        let empty = download_icon(Some(0.0));
        let half = download_icon(Some(0.5));
        assert_eq!(empty.len(), 32 * 32 * 4);
        let green = |px: &[u8], x: usize| px[(27 * 32 + x) * 4 + 1] == 0xdc;
        assert!(!green(&empty, 10) && green(&half, 10) && !green(&half, 26));
        // corners are see-through, the letter is white
        assert_eq!(half[3], 0);
        assert_eq!(&half[(5 * 32 + 10) * 4..(5 * 32 + 10) * 4 + 4], &[255, 255, 255, 255]);
        assert_eq!(short_tip(&"x".repeat(300)).chars().count(), 120);
        assert_eq!(step_of(0.5), 12);
    }
}
