extern crate winapi;

use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::fileapi::{GetLogicalDrives};

fn main() {
   fn disk_enum(){
    unsafe {
        let drives = GetLogicalDrives();
        let mut mask = 1;
        println!("{}", drives);


        for _ in 0..26 {
            if drives & mask != 0 {
                let drive_letter = ('A' as u8 + (mask.trailing_zeros() as u8)) as char;
                println!("Drive {}:\\", drive_letter);
            }
            mask <<= 1;
        }
    }
   } 
   disk_enum()
}