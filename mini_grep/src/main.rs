use std::env;
use crate::file::File as file_handler;
use directory::Directory;
use regex::RegexBuilder;
use std::io::{BufReader, self};
use crate::options::Options;
use crate::matches::Matches;

mod file;
mod options;
mod directory;
mod matches;

fn main() -> io::Result<()>{
    let args = env::args().collect::<Vec<String>>();

    let binding = args.get(1);
    let pattern = match &binding{
        Some(path) => path,
        None => panic!("MISSING PATTERN"),
    };

    let options =  Options::options_from_args(args.clone());

    

    


    let mut re: RegexBuilder;
    if options.w {
        re = RegexBuilder::new(format!(r"^({pattern})$").as_str());
    }else{
        re = RegexBuilder::new(pattern);
    } 
    if options.i {
        re.case_insensitive(true);
    }
    let re = re.build().unwrap();

    let binding = args.get(2);
    let file_path = match &binding{
        Some(path) => path,

        //Handle it as a directory
        None => {
            let dir = env::current_dir()?;
            let mut matches : Vec<(String, String)> = Vec::new();
            Directory::visit_dirs(&dir, &re, &options, &mut matches)?;
            if matches.is_empty(){
                println!("No matches found");
                return Ok(())
            }

            for (n, p) in matches{
        
                println!("File: {n}, Path: \"{p}\"");
            }

            return Ok(()); 
        },
    };


    let file = file_handler::get_file(file_path, true, false);
    let mut reader = BufReader::new(&file);
    let matches = Matches::find_matches(&mut reader, &re, &options);

    if matches.is_empty(){
        println!("No matches found");
        return Ok(())
    }
     
    for (l, o, s) in matches{
        if options.b{

            println!("Line: {l}, Start: {o}, Match: \"{s}\"");
        }else{

            println!("Line: {l}, Match: {s}");
        }
    }
    
    Ok(())
}




