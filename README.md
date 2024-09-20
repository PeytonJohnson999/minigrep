# minigrep

Smaller version of `grep` that uses a regular expression for the pattern. Takes two arguments: `PATTERN` and `PATH_TO_FILE` respectively. Currently **only** works by searching in a file

Has support for `-i`(ignorecase), `-v`(invert-match), `-o`(only-matching), `-b`(byte-offset), and `-m`(max-count) options.

Binary file is located in mini_grep/target/debug/mini_grep. 