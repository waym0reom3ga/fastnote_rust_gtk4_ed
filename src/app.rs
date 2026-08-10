/* FastNote Rust/GTK4 Edition — Application state */

use gtk4::gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct FastNoteApp {
    pub notes_dir: String,
    pub current_path: Option<String>,
    pub document_content: Option<String>,
    pub dirty: bool,
    pub theme: u8,
}

impl FastNoteApp {
    pub fn new() -> Self {
        Self {
            notes_dir: std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()),
            current_path: None,
            document_content: None,
            dirty: false,
            theme: 0,
        }
    }

    pub fn run(&self, args: &[String]) -> i32 {
        let mut app = self.clone();

        for (i, arg) in args.iter().enumerate() {
            if arg == "--headless" {
                let mut j = i + 1;
                while j < args.len() {
                    match args[j].as_str() {
                        "--open" if j + 1 < args.len() => {
                            j += 1;
                            if let Ok(content) = std::fs::read_to_string(&args[j]) {
                                app.document_content = Some(content);
                                app.current_path = Some(args[j].clone());
                            }
                        }
                        "--selftest" => {
                            let html = crate::renderer::render_markdown("# Hello\n**World**");
                            if html.contains("<h1>") && html.contains("<strong>") {
                                println!("selftest: pass");
                                return 0;
                            }
                            println!("selftest: fail");
                            return 1;
                        }
                        "--save" => {
                            if let (Some(ref path), Some(ref content)) =
                                (&app.current_path, &app.document_content)
                            {
                                let _ = std::fs::write(path, content);
                            }
                        }
                        "--export" if j + 1 < args.len() => {
                            j += 1;
                            if let Some(ref content) = app.document_content {
                                let html = crate::renderer::render_markdown(content);
                                let _ = std::fs::write(&args[j], &html);
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                return 0;
            }
            if arg == "--notes-dir" && i + 1 < args.len() {
                app.notes_dir = args[i + 1].clone();
            }
        }

        let state = Rc::new(RefCell::new(app));
        let application = Application::builder()
            .application_id("com.fastnote.rust.gtk4")
            .build();

        let state_clone = Rc::clone(&state);
        application.connect_activate(move |gtk_app| {
            let s = state_clone.borrow().clone();
            crate::ui::build_ui(gtk_app, Rc::new(RefCell::new(s)));
        });

        application.run_with_args(args).into()
    }
}
