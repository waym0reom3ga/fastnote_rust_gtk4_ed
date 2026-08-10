/* FastNote Rust/GTK4 Edition — Main entry point */

mod app;
mod file_browser;
mod renderer;
mod export;
mod actions;
mod ui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|a| a == "--version") {
        println!("fastnote-rust-gtk4 v1.0");
        return;
    }
    
    let mut app = app::FastNoteApp::new();
    let status = app.run(&args);
    std::process::exit(status);
}
