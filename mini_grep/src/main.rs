use std::{env, io::{self, Seek}, thread::current};
use crate::file::File as file_handler;
use regex::{bytes, RegexBuilder};
use std::io::{BufReader, Read, BufRead};
use std::fs::File;
use std::borrow::Borrow;
use std::iter::Cloned;
// use std::fs::File;

mod file;

fn main() -> io::Result<()>{
    let args = env::args().collect::<Vec<String>>();

    let binding = args.get(1);
    let pattern = match &binding{
        Some(path) => path,
        None => panic!("MISSING PATTERN"),
    };

    let binding = args.get(2);
    let file_path = match &binding{
        Some(path) => path,
        None => panic!("MISSING FILE PATH"),
    };


    let file = file_handler::get_file(file_path, true, false);
    let re = RegexBuilder::new(pattern).build().unwrap();
    let mut reader = BufReader::new(&file);
    let matches = find_matches(&mut reader, &re);
     
    for (l, s) in matches{
        println!("Line: {l}, Match: {s}");
    }
    
    Ok(())
}

fn find_matches<'a>(reader: &mut BufReader<&File>, re: &regex::Regex) ->  Vec<(u32, String)>{
    let mut matches: Vec<(u32, String)> = Vec::new();
    let mut current_line: u32 = 0;
    for line in reader.lines(){

        current_line += 1;
        match re.find(&line.unwrap()){
            Some(m)=> matches.push((current_line, m.as_str().to_owned())),
            None => (),
        }
    }

    matches
}