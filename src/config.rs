extern crate app;
use app::{App, Opt};

use std::path::PathBuf;

impl Config {
    pub fn parse_args() -> Self {
        let mut config = Config::default();
        let helper = {
            App::new("bitcoin-asm")
                .version("0.1.0")
                .author("Gerard D. Sexton", "wildcat.gerry@gmail.com")
                .addr("Github", "https://github.com/wildcatgerry")
                .desc("Assembler for Bitcoin Script")
                .opt(Opt::new("input", &mut config.input)
                        .short("i")
                        .long("input")
                        .help("set the input Bitcoin Script file."))
                .opt(Opt::new("output", &mut config.output)
            			.optional()
	                    .short("o")
	                    .long("output")
	                    .help("sets the output file. Default: out.bcb"))                
                .parse_args()
        };
        config
        	.check()
            .map_err(|e| helper.help_err_exit(e, 1))
            .unwrap()
    }
    fn check(self) -> Result<Self, String> {
        if !PathBuf::from(&self.input).as_path().exists() {
            return Err(format!("Input file {:?} does not exist", &self.input));
        }   
        Ok(self)
    }
}


#[derive(Debug,Clone)]
pub struct Config {
    pub input: String,
    pub output: String    
}

impl Default for Config {
    fn default() -> Self {
        Config {
            input: String::new(),
            output: String::from("out.bcb"),            
        }
    }
}