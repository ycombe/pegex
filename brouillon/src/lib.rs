pub mod configuration {
    use std::path::PathBuf;
    use clap::Parser;
    pub use rand::RngCore;
    use std::fmt;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Cli {
        /// Sample space, comma separated list
        #[arg(short, long, allow_hyphen_values=true)]
        omega: String,
    
        /// Law, comma separated list of values 
        #[arg(short, long, allow_hyphen_values=false)]
        law: Option<String>,
    
        /// Repeatitions of simulation
        #[arg(short, default_value_t = 1)]
        n: u32,
    
        /// Random number generator name or file
        #[arg(short, long, default_value_t = String::from("chacha"))]
        rng: String
    }
    
//    #[derive(Debug)]
    pub enum RngSource {
        RNG(Box<dyn RngCore>),
        FILE(PathBuf)
    }
    
    impl fmt::Debug for RngSource {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RngSource::RNG(_) => write!(f, "RngSource is RngCore."),
                RngSource::FILE(p) => write!(f, "RngSource from file {:?}", p)
            }
        }
    }

    #[derive(Debug)]
    pub struct Config {
        pub omega: Vec<String>,
        pub law: Vec<f64>,
        pub n: u32,
        pub rng: RngSource<>
    }
    impl Config {
        pub fn new() -> Self {
            let cli = Cli::parse();

            println!("{:?}", cli);

            Config { 
                omega: Vec::new(), 
                law: Vec::new(),
                n: 0, 
                rng: RngSource::FILE(PathBuf::new())
            }
        }
    }
}

