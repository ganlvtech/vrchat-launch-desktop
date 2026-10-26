#![windows_subsystem = "windows"]

use anyhow::anyhow;
use std::env;
use std::process::Command;
use windows::core::{w, HSTRING};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, IDOK, MB_ICONERROR, MB_OK, MB_OKCANCEL};
use windows_registry::{Value, CLASSES_ROOT, CURRENT_USER};

fn msg_box(s: &str) {
    unsafe {
        let _ = MessageBoxW(None, &HSTRING::from(s), w!("VRChat Launch Desktop"), MB_OK);
    }
}

fn msg_box_error(s: &str) {
    unsafe {
        let _ = MessageBoxW(None, &HSTRING::from(s), w!("VRChat Launch Desktop"), MB_OK | MB_ICONERROR);
    }
}

fn msg_box_cancelable(s: &str) -> bool {
    unsafe {
        MessageBoxW(None, &HSTRING::from(s), w!("VRChat Launch Desktop"), MB_OKCANCEL) == IDOK
    }
}

fn add_uninstaller() -> anyhow::Result<()> {
    let exe_path = env::current_exe().map_err(|e| anyhow!("get current exe path error: {}", e))?;
    let key = CURRENT_USER.create("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\launchnovr").map_err(|e| anyhow!("create uninstall key error: {}", e))?;
    key.set_string("DisplayName", "VRChat Launch Desktop").map_err(|e| anyhow!("set DisplayName error: {}", e))?;
    key.set_string("UninstallString", format!("\"{}\" --uninstall", exe_path.to_string_lossy())).map_err(|e| anyhow!("set UninstallString error: {}", e))?;
    key.set_string("DisplayIcon", exe_path.to_string_lossy().to_string()).map_err(|e| anyhow!("set DisplayIcon error: {}", e))?;
    key.set_value("NoModify", &Value::from(1u32)).map_err(|e| anyhow!("set NoModify error: {}", e))?;
    key.set_value("NoRepair", &Value::from(1u32)).map_err(|e| anyhow!("set NoRepair error: {}", e))?;
    Ok(())
}

fn install() -> anyhow::Result<()> {
    let key = CURRENT_USER.create("SOFTWARE\\Classes\\vrchatnovr").map_err(|e| anyhow!("create HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchatnovr error: {}", e))?;
    key.set_string("URL Protocol", "").map_err(|e| anyhow!("set HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchatnovr URL Protocol error: {}", e))?;
    let cmd_key = CURRENT_USER.create("SOFTWARE\\Classes\\vrchatnovr\\shell\\open\\command").map_err(|e| anyhow!("create HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchatnovr\\shell\\open\\command error: {}", e))?;
    let exe_path = env::current_exe().map_err(|e| anyhow!("get current exe path error: {}", e))?;
    cmd_key.set_string("", format!("\"{}\" \"%1\"", exe_path.to_string_lossy())).map_err(|e| anyhow!("set HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchatnovr\\shell\\open\\command error: {}", e))?;
    add_uninstaller().map_err(|e| anyhow!("add uninstaller error: {}", e))?;
    Ok(())
}

fn uninstall() -> anyhow::Result<()> {
    CURRENT_USER.remove_tree("SOFTWARE\\Classes\\vrchatnovr").map_err(|e| anyhow!("remove HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchatnovr error: {}", e))?;
    CURRENT_USER.remove_tree("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\launchnovr").map_err(|e| anyhow!("remove uninstall key error: {}", e))?;
    Ok(())
}

fn get_vrchat_launch_exe_path() -> anyhow::Result<String> {
    let key = CLASSES_ROOT.open("vrchat\\shell\\open\\command").map_err(|e| anyhow!("open HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchat\\shell\\open\\command error: {}", e))?;
    let s = key.get_string("").map_err(|e| anyhow!("get HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchat\\shell\\open\\command as string error: {}", e))?;
    let cmd = s.split('"').nth(1).ok_or(anyhow!("parse HKEY_CURRENT_USER\\SOFTWARE\\Classes\\vrchat\\shell\\open\\command error"))?.to_string();
    Ok(cmd)
}

fn main2() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        if msg_box_cancelable("Are you sure to install launchnovr?") {
            match install() {
                Ok(_) => msg_box("Install successfully"),
                Err(e) => return Err(anyhow!("Install error: {}", e)),
            }
        }
    } else if args.len() == 1 && args[0] == "--uninstall" {
        if msg_box_cancelable("Are you sure to uninstall launchnovr?") {
            match uninstall() {
                Ok(_) => msg_box("Uninstall successfully"),
                Err(e) => return Err(anyhow!("Uninstall error: {}", e)),
            }
        }
    } else {
        match get_vrchat_launch_exe_path() {
            Ok(exe_path) => {
                let mut cmd = Command::new(&exe_path);
                for arg in &args {
                    cmd.arg(arg.replace("vrchatnovr://", "vrchat://"));
                }
                cmd.arg("--no-vr");
                match cmd.status() {
                    Ok(status) => {
                        if !status.success() {
                            return Err(anyhow!("Run VRChat launch.exe path error: {}", status));
                        }
                    }
                    Err(e) => return Err(anyhow!("Run VRChat launch.exe path error: {}", e)),
                }
            }
            Err(e) => return Err(anyhow!("Get VRChat launch.exe path error error: {}", e)),
        }
    }
    Ok(())
}

fn main() {
    match main2() {
        Ok(_) => {}
        Err(e) => msg_box_error(&format!("{}", e)),
    }
}
