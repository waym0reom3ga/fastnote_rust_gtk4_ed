/* FastNote Rust/GTK4 Edition — Export module */

use std::fs;
use std::process::Command;

pub fn export_html(path: &str, html: &str) -> Result<(), String> {
    fs::write(path, html).map_err(|e| format!("Write failed: {}", e))
}

pub fn export_pdf(path: &str, html: &str) -> Result<(), String> {
    let output = Command::new("wkhtmltopdf")
        .args(["--quiet", "-", path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    
    match output {
        Ok(mut child) => {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(html.as_bytes());
            }
            let status = child.wait().map_err(|e| format!("wkhtmltopdf error: {}", e))?;
            if status.success() {
                Ok(())
            } else {
                Err("wkhtmltopdf failed".into())
            }
        }
        Err(_) => Err("wkhtmltopdf not found".into()),
    }
}
