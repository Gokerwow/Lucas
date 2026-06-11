#[tauri::command]
fn open_in_browser(url: String) -> Result<(), String> {
  if !url.starts_with("http://") && !url.starts_with("https://") {
    return Err("Only http and https links are allowed".to_string());
  }

  #[cfg(target_os = "windows")]
  {
    std::process::Command::new("cmd")
      .args(["/C", "start", "", &url])
      .spawn()
      .map_err(|e| e.to_string())?;
  }
  #[cfg(target_os = "macos")]
  {
    std::process::Command::new("open")
      .arg(&url)
      .spawn()
      .map_err(|e| e.to_string())?;
  }
  #[cfg(target_os = "linux")]
  {
    std::process::Command::new("xdg-open")
      .arg(&url)
      .spawn()
      .map_err(|e| e.to_string())?;
  }
  Ok(())
}

#[tauri::command]
fn send_notification(title: String, body: String) {
  #[cfg(target_os = "windows")]
  {
    use std::os::windows::process::CommandExt;
    let script = format!(
      "[void] [System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); \
       $notification = New-Object System.Windows.Forms.NotifyIcon; \
       $notification.Icon = [System.Drawing.SystemIcons]::Information; \
       $notification.BalloonTipTitle = '{}'; \
       $notification.BalloonTipText = '{}'; \
       $notification.Visible = $true; \
       $notification.ShowBalloonTip(10000)",
      title.replace("'", "''"),
      body.replace("'", "''")
    );
    let _ = std::process::Command::new("powershell")
      .creation_flags(0x08000000) // CREATE_NO_WINDOW
      .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
      .spawn();
  }
  #[cfg(target_os = "macos")]
  {
    let script = format!(
      "display notification \"{}\" with title \"{}\"",
      body.replace("\"", "\\\""),
      title.replace("\"", "\\\"")
    );
    let _ = std::process::Command::new("osascript")
      .args(["-e", &script])
      .spawn();
  }
  #[cfg(target_os = "linux")]
  {
    let _ = std::process::Command::new("notify-send")
      .args([&title, &body])
      .spawn();
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![open_in_browser, send_notification])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
