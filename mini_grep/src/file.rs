
pub mod File {
    use std::fs::{self, File};
    use std::io::{prelude::*, self, ErrorKind, Write, BufReader};


    pub fn get_file(file_name: &str, r: bool, w: bool) -> File{
        match fs::OpenOptions::new().read(r).write(w).open(file_name){
            Ok(file) => file,
            Err(e) => match e.kind(){
                ErrorKind::NotFound => panic!("SPECIFIED FILE NOT FOUND IN PATH"),
                _ => panic!("Error opening file: {}", e),
            }, 
        }
    }


}