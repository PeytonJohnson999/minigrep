use std::env;
use std::fmt::Error;
use crate::file::File as file_handler;
use directory::Directory;
use regex::RegexBuilder;
use std::io::{BufReader, self, BufRead};
use std::fs::File;
use crate::options::Options;
use std::path::Path;

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

    let options = Options::Options::options_from_args(args.clone());

    

    


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
    let matches = find_matches(&mut reader, &re, &options);

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

fn find_matches(reader: &mut BufReader<&File>, re: &regex::Regex, o: &Options::Options) ->  Vec<(u32, u32, String)>{
    let mut matches: Vec<(u32, u32, String)> = Vec::new();
    let mut current_line: u32 = 0;
    for line in reader.lines(){
        if (matches.len() == o.m.1.try_into().unwrap() && o.m.0){
            break;
        }
        let line = line.unwrap().trim().to_string();

        current_line += 1;
        if line.is_empty(){
            continue;
        }
        
        // If the option -v is enabled and the line is not empty print the entire line
        if o.v{
            match re.find(&line){
                Some(_) => (),
                None => {
                    matches.push((current_line, 0, line))
                },
            }
        // 
        }else {
            match re.find(&line){
                Some(m) => {
                    //print only the match
                    if o.o{
                        matches.push((current_line, (m.start() + 1) as u32, m.as_str().to_owned()))

                    // print the entire line
                    }else {
                        matches.push((current_line, 0, line))
                    }
                },
                None => (),
            }
        }
        
    }

    matches
}



