pub mod configuration {
    use clap::Parser;
    use rand::SeedableRng;
    pub use rand::RngCore;
    use std::fmt;
    use rand_chacha::{self, ChaCha8Rng, ChaChaRng};

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
        n: usize,
    
        /// Random number generator name or file
        #[arg(short, long, default_value_t = String::from("chacha"))]
        rng: String,

        /// Print  informative/debug output
        # [arg(short, long, default_value_t = false)]
        verbose: bool,

        /// RNG Seed (unsigned int value) [ default value from random default generator ]
        # [arg(short, long)]
        seed: Option<u64>
    }
    
    pub enum ERng {
        Chacha(ChaChaRng),
        Chacha8(ChaCha8Rng),
//        Chacha12(ChaCha12Rng),
//        Chacha20(ChaCha20Rng),
    }
    
    impl fmt::Debug for ERng {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chacha(arg0) => f.debug_tuple("Chacha").field(arg0).finish(),
            Self::Chacha8(arg0) => f.debug_tuple("Chacha8").field(arg0).finish(),
        }
    }
    }

    pub struct CRng (ERng);

    impl CRng {
        pub fn new(id: &str, seed: u64) -> Self {
            match id {
                "chacha" =>  CRng(ERng::Chacha(rand_chacha::ChaChaRng::seed_from_u64(seed))),
                "chacha8" =>  CRng(ERng::Chacha8(rand_chacha::ChaCha8Rng::seed_from_u64(seed))),
                _ => CRng(ERng::Chacha(rand_chacha::ChaChaRng::seed_from_u64(seed))),
            }
        }
    }

    impl RngCore for CRng {
        fn next_u32(&mut self) -> u32 {
            match &mut self.0 {
                ERng::Chacha(rng) => rng.next_u32(),
                ERng::Chacha8(rng) => rng.next_u32(),
            }
        }
        
        fn next_u64(&mut self) -> u64 {
            match &mut self.0 {
                ERng::Chacha(rng) => rng.next_u64(),
                ERng::Chacha8(rng) => rng.next_u64(),
            }            
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            match &mut self.0 {
                ERng::Chacha(rng) => rng.fill_bytes(dst),
                ERng::Chacha8(rng) => rng.fill_bytes(dst),
            }            
        }
    }

    impl fmt::Debug for CRng {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("CRng").field(&self.0).finish()
        }
    }

    fn parse_omega(o_arg: &str, _verbose: bool) -> Vec<String> {
        o_arg.split(',').map(|s| String::from(s)).collect()
    }

    // need omega to set equiprobable law
    fn parse_law(args: &Cli, omega: &Vec<String>, _verbose: bool) -> Vec<f64> {
        let omega_n = omega.len();

        match &args.law {
            None => {
                let p: f64 = 1.0 / omega_n as f64;
                (0..omega_n).map(|_| p).collect()
            },
            Some(l_arg) => {
                let mut res : Vec<f64> = Vec::new();
                for s in l_arg.split(',') {
                    match s.parse::<f64>() {
                        Ok(x) => res.push(x),
                        Err(e) => panic!("{:?} Parsing error for law: {} is not a float !", e, s)
                    }
                }

                // Validation
                if res.len() != omega_n {
                    panic!["Space sample omega and law MUST have the same length !"]
                }

                if res.iter().any(|x| *x < 0.0 ) {
                    panic!["law: values must be positive {:?}! ", res]
                }

                let error_margin = f64::EPSILON;
                let law_sum = res.iter().sum::<f64>(); 
                if (law_sum - 1.0).abs() > error_margin {
                    if _verbose {
                        println!("Law sum is {}. Normalizing to 1.0.", law_sum);
                    }
                    res.iter_mut().for_each(|x| *x = *x / law_sum );
                }
    
                res
            }
        }
    }

    #[derive(Debug)]
    pub struct Config {
        pub omega: Vec<String>,
        pub law: Vec<f64>,
        pub n: usize,
        pub rng: CRng,
        pub rng_id: String,
        pub rng_seed: u64,
        pub verbose: bool
    }
    impl Config {
        pub fn new() -> Self {
            let cli = Cli::parse();
            let verbose= cli.verbose;

            if verbose {
                println!("{:?}", cli);
            }

            let omega = parse_omega(&cli.omega, verbose);
            let law = parse_law(&cli, &omega, verbose);
            let rng_seed = match cli.seed {
                Some(v) => v,
                None => rand::random::<u64>()
            };

            let rng_id= String::from(cli.rng);
            let rng = CRng::new(&rng_id, rng_seed);

            Config { 
                omega, 
                law,
                n: cli.n, 
                rng_id,
                rng_seed,
                rng,
                verbose: cli.verbose
            }
        }
    }
}

