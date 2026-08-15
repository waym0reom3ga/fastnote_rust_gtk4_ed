/* FastNote Rust/GTK4 Edition — UI */

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Paned, TextView, EventControllerKey};
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::FastNoteApp;

pub fn build_ui(app: &Application, state: Rc<RefCell<FastNoteApp>>) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("FastNote"));
    window.set_default_size(1080, 740);

    let toolbar = Box::new(gtk4::Orientation::Horizontal, 6);
    let open_btn = Button::with_label("Open");
    let save_btn = Button::with_label("Save");
    let save_as_btn = Button::with_label("Save As");
    let export_html_btn = Button::with_label("Export HTML");
    let export_pdf_btn = Button::with_label("Export PDF");
    let theme_btn = Button::with_label("Theme");

    toolbar.append(&open_btn);
    toolbar.append(&save_btn);
    toolbar.append(&save_as_btn);
    toolbar.append(&export_html_btn);
    toolbar.append(&export_pdf_btn);
    toolbar.append(&theme_btn);

    let paned = Paned::new(gtk4::Orientation::Horizontal);
    let editor = TextView::new();
    let preview = Label::new(None);

    paned.set_start_child(Some(&editor));
    paned.set_end_child(Some(&preview));

    let vbox = Box::new(gtk4::Orientation::Vertical, 0);
    vbox.append(&toolbar);
    vbox.append(&paned);
    window.set_child(Some(&vbox));

    // Save button handler
    let state_clone = Rc::clone(&state);
    save_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        if let Err(e) = crate::actions::save_file(&mut s) {
            eprintln!("Save error: {}", e);
        }
    });

    // Export HTML button handler
    let state_clone = Rc::clone(&state);
    export_html_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        if let Err(e) = crate::actions::export_html_action(&mut s, "export.html") {
            eprintln!("Export error: {}", e);
        }
    });

    // Keyboard accelerators (spec 5.2)
    let key_controller = EventControllerKey::new();
    let state_clone = Rc::clone(&state);
    key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        let ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
        
        if ctrl && !shift && keyval == gtk4::gdk::Key::o {
            // Open
            return glib::Propagation::Stop;
        }
        if ctrl && !shift && keyval == gtk4::gdk::Key::s {
            // Save
            let mut s = state_clone.borrow_mut();
            if let Err(e) = crate::actions::save_file(&mut s) {
                eprintln!("Save error: {}", e);
            }
            return glib::Propagation::Stop;
        }
        if ctrl && shift && keyval == gtk4::gdk::Key::S {
            // Save As
            return glib::Propagation::Stop;
        }
        if ctrl && !shift && keyval == gtk4::gdk::Key::e {
            // Export HTML
            let mut s = state_clone.borrow_mut();
            if let Err(e) = crate::actions::export_html_action(&mut s, "export.html") {
                eprintln!("Export error: {}", e);
            }
            return glib::Propagation::Stop;
        }
        if ctrl && shift && keyval == gtk4::gdk::Key::E {
            // Export PDF
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    // Emit painted event
    let state_clone = Rc::clone(&state);
    window.connect_show(move |_| {
        let s = state_clone.borrow();
        s.fn_event("painted");
    });

    window.show();
}
