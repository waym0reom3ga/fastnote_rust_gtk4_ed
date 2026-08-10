/* FastNote Rust/GTK4 Edition — Actions */

use crate::app::FastNoteApp;
use crate::export;
use crate::renderer;

pub fn open_file(app: &mut FastNoteApp, path: &str) -> Result<(), String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot open: {}", e))?;
    app.document_content = Some(content);
    app.current_path = Some(path.to_string());
    app.dirty = false;
    Ok(())
}

pub fn save_file(app: &mut FastNoteApp) -> Result<(), String> {
    let path = app.current_path.as_ref().ok_or("No file path")?;
    let content = app.document_content.as_ref().ok_or("No content")?;
    std::fs::write(path, content).map_err(|e| format!("Save failed: {}", e))?;
    app.dirty = false;
    Ok(())
}

pub fn export_html_action(app: &mut FastNoteApp, output: &str) -> Result<(), String> {
    let content = app.document_content.as_ref().ok_or("No content")?;
    let html = renderer::render_markdown(content);
    export::export_html(output, &html)
}

pub fn export_pdf_action(app: &mut FastNoteApp, output: &str) -> Result<(), String> {
    let content = app.document_content.as_ref().ok_or("No content")?;
    let html = renderer::render_markdown(content);
    export::export_pdf(output, &html)
}
