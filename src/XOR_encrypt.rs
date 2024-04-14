use std::fs::File;
use std::io::{self, Read};



pub unsafe fn XOR_encrypt(path: &str)  -> std::io::Result<()>{

  //This part of code, can read a file with a path 
  let mut file = File::open(path)?;
 
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;


    //Test to check if my function XOR_operation is done
    let test_crypt = XOR_operation(&binary);

    if XOR_operation(&test_crypt) == binary{ println!("YES YES YES")}
    

    Ok(())
}
pub unsafe fn XOR_operation(message: &[u8]) -> Vec<u8>{

//indeed, leaving the key in a code isn't optimal for "security", but at the moment, it stays here. 
  let key = b"IloveAlain";
  let mut encrypted_message = Vec::new();

  for (i, byte) in message.iter().enumerate() {
    let key_byte = key[i % key.len()]; 
    let encrypted_byte = byte ^ key_byte;
    encrypted_message.push(encrypted_byte);
  }
  encrypted_message
}
