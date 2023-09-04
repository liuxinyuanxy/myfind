use regex::Regex;
use std::{path::Path, process::exit};
mod option;
use option::Options;
pub struct Args {
    options: Options,
    paths: Vec<String>,
    expressions: Vec<Regex>,
    name: String,
}

impl Args {
    pub fn new(args: Vec<String>) -> Self {
        let mut options: Vec<String> = Vec::new();
        let mut paths: Vec<String> = Vec::new();
        let mut expressions: Vec<Regex> = Vec::new();
        let name = args[0].clone();
        for arg in args[1..].iter() {
            if arg.starts_with("-") {
                options.push(arg.to_string());
            } else if Path::new(&arg).exists() {
                paths.push(arg.to_string())
            } else {
                let regex = match Regex::new(&arg) {
                    Ok(regex) => regex,
                    Err(_) => {
                        eprintln!("find: invalid argument {}, not a regex nor a path", arg);
                        exit(1);
                    }
                };
                expressions.push(regex);
            }
        }
        Self {
            options: Options::new(options),
            paths,
            expressions,
            name,
        }
    }

    fn check_if_path_satisfy(&self, path: &Path) -> bool {
        let filename = match path.file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => return false,
        };
        for regex in &self.expressions {
            if regex.is_match(&filename) {
                if self.options.verbose {
                    println!("\x1b[32m {} matches {}\x1b[0m", filename, regex);
                }
                return true;
            }
        }
        if self.options.verbose {
            println!("\x1b[31m {} does not match any regex\x1b[0m", filename);
        }
        false
    }

    fn dfs(&self, dir: &Path, matches: &mut Vec<String>) -> Result<(), std::io::Error> {
        if self.options.verbose {
            println!("\x1b[34m searching for {}\x1b[0m", dir.to_string_lossy());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && self.options.recursive {
                self.dfs(&path, matches)?;
            } else {
                if self.check_if_path_satisfy(&path) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
        Ok(())
    }

    fn find(&self) -> Result<(), std::io::Error> {
        let mut matches = Vec::new();
        if self.paths.is_empty() {
            if self.options.verbose {
                println!("\x1b[34m no path specified, using current directory\x1b[0m");
            }
            self.dfs(&Path::new("."), &mut matches)?;
        } else {
            for path in &self.paths {
                self.dfs(&Path::new(path), &mut matches)?;
            }
        }

        if matches.is_empty() {
            println!("no matches found");
        } else {
            println!("matches found:");
            matches.sort();
            for path in matches {
                println!("\t{}", path);
            }
        }
        Ok(())
    }
}

pub fn run(args: Args) -> Result<(), std::io::Error> {
    if args.options.help {
        println!("Usage: {} [options] [path...] [expressions...]", args.name);
        println!("Options:");
        println!("\t-v, --verbose\t\t\tVerbose output");
        println!("\t-V, --version\t\t\tPrint version information and exit");
        println!("\t-r, --recursive\t\t\tSearch recursively");
        println!("\t-h, --help\t\t\tPrint help information and exit");
        println!("Paths:");
        println!("\tPath\t\t\t\tSearch for files in the path, default to current directory");
        println!("Expressions:");
        println!("\tRegex\t\t\t\tSearch for files matching the regex, should be quoted");
        println!("Examples:");
        println!("\t{} -r . ~/ \"\\.(rs|toml)$\"", args.name);
        exit(0);
    }
    if args.options.version {
        println!("find 0.1.0");
        exit(0);
    }
    args.find()?;
    Ok(())
}
