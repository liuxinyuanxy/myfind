pub struct Options {
    pub verbose: bool,
    pub help: bool,
    pub version: bool,
    pub recursive: bool,
}

impl Options {
    pub fn new(args: Vec<String>) -> Self {
        let mut verbose = false;
        let mut help = false;
        let mut version = false;
        let mut recursive = false;
        for arg in args {
            match arg.as_str() {
                "-v" | "--verbose" => verbose = true,
                "-V" | "--version" => version = true,
                "-r" | "--recursive" => recursive = true,
                _ => help = true,
            }
        }
        Self {
            verbose,
            help,
            version,
            recursive,
        }
    }
}
