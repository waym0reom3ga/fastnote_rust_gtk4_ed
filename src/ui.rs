/* FastNote Rust/GTK4 Edition — UI */

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Paned, TextView};
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::FastNoteApp;

pub fn build_ui(app: &Application, state: Rc<RefCell<FastNoteApp>>) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("FastNote"));
    window.set_default_size(1024, 768);

    let toolbar = Box::new(gtk4::Orientation::Horizontal, 6);
    let open_btn = Button::with_label("Open");
    let save_btn = Button::with_label("Save");
    let export_btn = Button::with_label("Export");

    toolbar.append(&open_btn);
    toolbar.append(&save_btn);
    toolbar.append(&export_btn);

    let paned = Paned::new(gtk4::Orientation::Horizontal);
    let editor = TextView::new();
    let preview = Label::new(None);

    paned.set_start_child(Some(&editor));
    paned.set_end_child(Some(&preview));

    let vbox = Box::new(gtk4::Orientation::Vertical, 0);
    vbox.append(&toolbar);
    vbox.append(&paned);
    window.set_child(Some(&vbox));

    let state_clone = Rc::clone(&state);
    save_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        if let Err(e) = crate::actions::save_file(&mut s) {
            eprintln!("Save error: {}", e);
        }
    });

    let state_clone = Rc::clone(&state);
    export_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        if let Err(e) = crate::actions::export_html_action(&mut s, "export.html") {
            eprintln!("Export error: {}", e);
        }
    });

    window.show();
}
