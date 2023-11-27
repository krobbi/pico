/// Configuration data for Pico.
pub struct Config {
    /// The path to the source PNG file to read.
    pub source_path: String,
}

impl Config {
    /// Create a new config instance from command line arguments.
    pub fn from_args() -> Self {
        use clap::{arg, command};
        
        let mut matches = command!()
            .arg(arg!(<source> "The source PNG file to read"))
            .get_matches();
        
        let source_path = matches
            .remove_one::<String>("source")
            .unwrap();
        
        Config {
            source_path,
        }
    }
}
