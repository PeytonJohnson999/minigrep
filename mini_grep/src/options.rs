pub mod Options{
    use regex::Regex;
    #[derive(Debug)]
    pub struct Options{
        pub i : bool,
        pub v : bool,
        pub o : bool,
        pub b : bool,
    }

    impl Options{
        pub fn options_from_args(args : Vec<String>) -> Self{
            let args = args.join(" ");
            Options { 
                i: Regex::new(r"-\w*i").unwrap().is_match(&args),
                v: Regex::new(r"-\w*v").unwrap().is_match(&args),
                o: Regex::new(r"-\w*o").unwrap().is_match(&args),
                b: Regex::new(r"-\w*b").unwrap().is_match(&args),
            }
        }
    }
}