extern crate winapi;
extern crate walkdir;
use walkdir::WalkDir;
use winapi::um::fileapi::{GetLogicalDrives};


pub unsafe fn disk_enum(){   
    let drives = GetLogicalDrives();
    let mut mask = 1;

    // this loop is capable to enumerate all file on the computer
    for _ in 0..26 {
        if drives & mask != 0 {
            let drive_letter = ('A' as u8 + (mask.trailing_zeros() as u8)) as char;
            let root = drive_letter.to_string() + &":\\";
            println!("{root}");
        
            /*for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    println!("{}", entry.path().display());*/


        }
        mask <<= 1;
       
    }
    // For exercice, we are going to enumerate just one local disc
    for entry in WalkDir::new("E:\\").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());

}}}