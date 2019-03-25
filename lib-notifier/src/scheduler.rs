#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

use std::error::Error;

#[cfg(target_os = "linux")]
pub fn schedule(title: &str, body: &str, when: &str) -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("at")
        .arg("-V")
        .output()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("Successfully called the `at` command: {}", s);

        Ok(())
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("Failed to call the `at` command: {}", s);

        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, s)))
    }
}

#[cfg(target_os = "macos")]
pub fn schedule(title: &str, body: &str, when: &str) -> Result<(), Box<dyn Error>> { 
    let date: f64 = when.parse()?;
    mac_notification_sys::schedule_notification(title, &None, body, &Some("Ping"), date)?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn schedule(title: &str, body: &str, when: &str) -> Result<(), Box<dyn Error>> { Ok(()) }

