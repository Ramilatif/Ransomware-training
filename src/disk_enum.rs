extern crate winapi;
extern crate walkdir;

use std::path::Path;
use walkdir::WalkDir;
use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::fileapi::{GetLogicalDrives};


pub unsafe fn disk_enum(){   
    let drives = GetLogicalDrives();
    let mut mask = 1;

    for _ in 0..26 {
        if drives & mask != 0 {
            let drive_letter = ('A' as u8 + (mask.trailing_zeros() as u8)) as char;
            let root = drive_letter.to_string() + &":\\";
            println!("{root}");
            
            for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    println!("{}", entry.path().display());


        }
        mask <<= 1;
       
    }

}}}