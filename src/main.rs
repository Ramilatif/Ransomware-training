extern crate winapi;
mod disk_enum;
use disk_enum::disk_enum;

fn main() {unsafe {

        disk_enum()
    }
   } 
  
