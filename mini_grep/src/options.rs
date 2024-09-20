pub mod Options{
    use regex::Regex;
    #[derive(Debug)]
    pub struct Options{
        pub i : bool,
        pub v : bool,
        pub o : bool,
        pub b : bool,
        pub m : (bool, u32),
    }

    impl Options{
        pub fn options_from_args(args : Vec<String>) -> Self{
            let args = args.join(" ");
            Options { 
                i: Regex::new(r"-\w*i").unwrap().is_match(&args),
                v: Regex::new(r"-\w*v").unwrap().is_match(&args),
                o: Regex::new(r"-\w*o").unwrap().is_match(&args),
                b: Regex::new(r"-\w*b").unwrap().is_match(&args),
                m: {
                    
                    let m = Regex::new(r"-\w*m").unwrap().find(&args);
                    let mut enabled = false;
                    let mut lines = 0;
                    if m.is_some() {
                        enabled = true;
                        let m = m.unwrap();
                        
                        //Finding the beginning and end of the number
                        let mut num = String::new();
                        for ch in args[m.end()..].to_string().chars(){
                            if (ch.is_digit(10)){
                                num.push(ch);
                            }else if ( num.len() > 0){
                                break;
                            }
                        }
                        lines = num.parse().expect("failed parsing -m num");
                    }

                    (enabled, lines)
                }
            }
        }
    }
}