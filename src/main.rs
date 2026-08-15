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
    
    let mut event_file = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--event-file" && i + 1 < args.len() {
            event_file = Some(args[i + 1].clone());
            i += 2;
        } else {
            eprintln!("fastnote-rust-gtk4: unknown option: {}", args[i]);
            std::process::exit(2);
        }
    }
    
    let mut app = app::FastNoteApp::new();
    app.event_file = event_file;
    let status = app.run();
    std::process::exit(status);
}
