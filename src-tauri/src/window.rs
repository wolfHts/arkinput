#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub app_name: String,
    pub window_title: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn get_active_window() -> Option<WindowInfo> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0 == std::ptr::null_mut() {
            return None;
        }

        // Get window title
        let mut title_buf = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);
        let window_title = if title_len > 0 {
            Some(OsString::from_wide(&title_buf[..title_len as usize])
                .to_string_lossy()
                .to_string())
        } else {
            None
        };

        // Get process ID
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        // Get process name
        let app_name = if process_id != 0 {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                let mut name_buf = [0u16; 512];
                let mut name_len = name_buf.len() as u32;
                if QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, &mut name_buf, &mut name_len).is_ok() {
                    let path = OsString::from_wide(&name_buf[..name_len as usize])
                        .to_string_lossy()
                        .to_string();
                    std::path::Path::new(&path)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string())
                } else {
                    "Unknown".to_string()
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        Some(WindowInfo {
            app_name,
            window_title,
        })
    }
}

#[cfg(target_os = "macos")]
pub fn get_active_window() -> Option<WindowInfo> {
    use std::process::Command;

    // Use AppleScript to get the frontmost application name
    // This is more reliable than using objc bindings directly
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first application process whose frontmost is true")
        .output()
        .ok()?;

    if !output.status.success() {
        return Some(WindowInfo {
            app_name: "Unknown".to_string(),
            window_title: None,
        });
    }

    let app_name = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if app_name.is_empty() {
        return Some(WindowInfo {
            app_name: "Unknown".to_string(),
            window_title: None,
        });
    }

    Some(WindowInfo {
        app_name,
        window_title: None,
    })
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn get_active_window() -> Option<WindowInfo> {
    // Linux support could be added using x11 or wayland APIs
    Some(WindowInfo {
        app_name: "Unknown".to_string(),
        window_title: None,
    })
}
