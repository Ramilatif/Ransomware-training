use std::fs::File;
use std::io::{self, Read};



pub unsafe fn XOR_encrypt(path: &str)  -> std::io::Result<()>{

  //This part of code, can read a file with a path 
  let mut file = File::open(path)?;
 
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    println!("{:?}", content);

    Ok(())

}