# minigrep

Smaller version of `grep` that uses a regular expression for the pattern. Takes two arguments: `PATTERN` and `PATH_TO_FILE` respectively. As of v2.0, it now has support for searching through directories for a file name and searching through a file for the specified pattern.

Has support for `-i`(ignorecase), `-v`(invert-match), `-o`(only-matching), `-b`(byte-offset), and `-m`(max-count) options.

Binary file is located in mini_grep/target/debug/mini_grep. 