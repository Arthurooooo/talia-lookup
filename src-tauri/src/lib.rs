mod bubble;

use bubble::*;
use serde::{Deserialize, Serialize};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

const KEYRING_SERVICE: &str = "fr.talia.lookup";
const KEYRING_USER: &str = "bubble-api-token";

// ---------- Token in OS credential manager ----------

#[tauri::command]
fn save_token(token: String) -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    entry.set_password(&token).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_token() -> Result<Option<String>, String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(t) => Ok(Some(t)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn clear_token() -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    let _ = entry.delete_credential();
    Ok(())
}

// ---------- Bubble API ----------

#[tauri::command]
async fn search_users(token: String, query: String) -> Result<Vec<BubbleUser>, String> {
    bubble::search_users(&token, &query).await
}

#[tauri::command]
async fn contracts_for(token: String, user_id: String) -> Result<Vec<BubbleContract>, String> {
    bubble::contracts_for(&token, &user_id).await
}

#[tauri::command]
async fn sessions_for(token: String, user_id: String) -> Result<Vec<BubbleSession>, String> {
    bubble::sessions_for(&token, &user_id).await
}

#[tauri::command]
async fn meetings_for(token: String, user_id: String) -> Result<Vec<BubbleMeeting>, String> {
    bubble::meetings_for(&token, &user_id).await
}

#[tauri::command]
async fn get_user(token: String, id: String) -> Result<Option<BubbleUser>, String> {
    bubble::get_user(&token, &id).await
}

#[tauri::command]
async fn get_class(token: String, id: String) -> Result<Option<BubbleClass>, String> {
    bubble::get_class(&token, &id).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentBundle {
    contracts: Vec<BubbleContract>,
    sessions: Vec<BubbleSession>,
    meetings: Vec<BubbleMeeting>,
    account_manager: Option<BubbleUser>,
    active_teacher: Option<BubbleUser>,
    active_class: Option<BubbleClass>,
}

#[derive(Deserialize)]
pub struct BundleArgs {
    pub token: String,
    pub user: BubbleUser,
}

#[tauri::command]
async fn fetch_student_bundle(args: BundleArgs) -> Result<StudentBundle, String> {
    let BundleArgs { token, user } = args;
    let (c, s, m, am) = tokio::join!(
        bubble::contracts_for(&token, &user.id),
        bubble::sessions_for(&token, &user.id),
        bubble::meetings_for(&token, &user.id),
        async {
            match &user.account_manager_id {
                Some(id) if !id.is_empty() => bubble::get_user(&token, id).await,
                _ => Ok(None),
            }
        }
    );
    let contracts = c.unwrap_or_default();
    let sessions = s.unwrap_or_default();
    let meetings = m.unwrap_or_default();
    let account_manager = am.unwrap_or(None);

    let active_session = sessions.iter().find(|s| is_session_active(s))
        .or_else(|| sessions.first());
    let active_contract = contracts.iter().find(|c| is_contract_active(c))
        .or_else(|| contracts.first());

    let teacher_fut = async {
        match active_session.and_then(|s| s.main_teacher_id.as_ref()) {
            Some(id) if !id.is_empty() => bubble::get_user(&token, id).await.unwrap_or(None),
            _ => None,
        }
    };
    let class_fut = async {
        match active_contract.and_then(|c| c.class_id.as_ref()) {
            Some(id) if !id.is_empty() => bubble::get_class(&token, id).await.unwrap_or(None),
            _ => None,
        }
    };
    let (active_teacher, active_class) = tokio::join!(teacher_fut, class_fut);

    Ok(StudentBundle {
        contracts, sessions, meetings, account_manager, active_teacher, active_class,
    })
}

fn is_contract_active(c: &BubbleContract) -> bool {
    if c.is_cancelled == Some(true) { return false; }
    if c.is_terminated == Some(true) || c.is_termination_occurred == Some(true) { return false; }
    let now = now_iso8601();
    if let Some(s) = &c.start_date { if s.as_str() > now.as_str() { return false; } }
    if let Some(e) = &c.end_date   { if e.as_str() < now.as_str() { return false; } }
    true
}

fn is_session_active(s: &BubbleSession) -> bool {
    if s.disabled_date.is_some() { return false; }
    let now = now_iso8601();
    if let Some(e) = &s.end_date { if e.as_str() < now.as_str() { return false; } }
    true
}

fn now_iso8601() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
    let days = secs.div_euclid(86400);
    let sod  = secs.rem_euclid(86400);
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = (z - era*146097) as u64;
    let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
    let y = (yoe as i64) + era*400;
    let doy = doe - (365*yoe + yoe/4 - yoe/100);
    let mp = (5*doy + 2)/153;
    let d = (doy - (153*mp+2)/5 + 1) as u64;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
        year, m, d, sod/3600, (sod%3600)/60, sod%60
    )
}

// ---------- App entry ----------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Menu de la barre des tâches.
            let item_open = MenuItem::with_id(app, "open",      "Ouvrir Talia Lookup", true, None::<&str>)?;
            let item_sep  = PredefinedMenuItem::separator(app)?;
            let item_quit = MenuItem::with_id(app, "quit",      "Quitter",             true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&item_open, &item_sep, &item_quit])?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("Talia Lookup")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true) // monochrome sur Mac
                .menu(&menu)
                .show_menu_on_left_click(false) // gauche = toggle fenêtre, droite = menu
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => { let _ = show_window(app); }
                    "quit" => { app.exit(0); }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) && w.is_focused().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Sur Windows on cache à la fermeture de la fenêtre plutôt que quitter — le
            // tray reste actif comme dans un vrai outil système.
            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let _ = w.hide();
                        api.prevent_close();
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_token, load_token, clear_token,
            search_users, contracts_for, sessions_for, meetings_for,
            get_user, get_class, fetch_student_bundle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    if let Some(w) = app.get_webview_window("main") {
        w.show()?;
        w.set_focus()?;
        w.unminimize()?;
    }
    Ok(())
}
