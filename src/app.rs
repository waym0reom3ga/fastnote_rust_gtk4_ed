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
    pub event_file: Option<String>,
}

impl FastNoteApp {
    pub fn new() -> Self {
        Self {
            notes_dir: std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()),
            current_path: None,
            document_content: None,
            dirty: false,
            theme: 0,
            event_file: None,
        }
    }

    pub fn run(&self) -> i32 {
        let app = self.clone();
        let state = Rc::new(RefCell::new(app));
        let application = Application::builder()
            .application_id("com.fastnote.rust.gtk4")
            .build();

        let state_clone = Rc::clone(&state);
        application.connect_activate(move |gtk_app| {
            let s = state_clone.borrow().clone();
            crate::ui::build_ui(gtk_app, Rc::new(RefCell::new(s)));
        });

        application.run_with_args(&[] as &[String]).into()
    }

    pub fn fn_event(&self, marker: &str) {
        if let Some(ref path) = self.event_file {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let _ = writeln!(f, "{}", marker);
            }
        }
    }
}
