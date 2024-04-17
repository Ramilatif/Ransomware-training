extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
mod window_interact;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;
use window_interact::window_interact;

use fltk::{prelude::*, *};

fn main() {
    unsafe {
        

        
            let app = app::App::default();
            let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
            my_window.end();
            my_window.show();
            app.run().unwrap();
        
    }
}
