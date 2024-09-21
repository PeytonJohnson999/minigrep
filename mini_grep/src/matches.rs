pub mod Matches{
    use crate::options::Options;
    use std::fs::DirEntry;
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    pub fn find_dirs(dir: &DirEntry, re: &regex::Regex, o: &Options::Options, matches : &mut Vec<(String, String)>) {
        // let mut matches = Vec::new();
        let p = dir.path();
        if re.is_match(p.file_name().unwrap().to_str().unwrap()){
            matches.push((p.file_name().unwrap().to_str().unwrap().to_string(), p.to_str().unwrap().to_string()));
        }
    
    }

    pub fn find_matches(reader: &mut BufReader<&File>, re: &regex::Regex, o: &Options::Options) ->  Vec<(u32, u32, String)>{
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
    
}