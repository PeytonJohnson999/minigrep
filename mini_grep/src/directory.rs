pub mod Directory{
    use std::io;
    use std::fs::{self};
    use std::path::Path;
    use crate::matches::Matches;
    use crate::options;

    pub fn visit_dirs(dir: &Path, re: &regex::Regex, o: &options::Options::Options,  matches : &mut Vec<(String, String)>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, re, o, matches)?;
                } else {
                    // println!("{:?}", entry);
                    Matches::find_dirs(&entry, re, o, matches);
                }
            }
        }
        Ok(())
    }
}