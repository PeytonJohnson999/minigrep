pub mod Matches{
    use std::{fs::DirEntry, path::Path};
    use crate::options::Options;

    pub fn find_dirs(dir: &DirEntry, re: &regex::Regex, o: &Options::Options, matches : &mut Vec<(String, String)>) {
        // let mut matches = Vec::new();
        let p = dir.path();
        if re.is_match(p.file_name().unwrap().to_str().unwrap()){
            matches.push((p.file_name().unwrap().to_str().unwrap().to_string(), p.to_str().unwrap().to_string()));
        }
    
    }
}