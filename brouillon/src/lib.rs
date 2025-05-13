pub mod configuration {
    use clap::Parser;
    use rand::SeedableRng;
    pub use rand::RngCore;
    //use std::fmt;
    use rand_chacha::{self, ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};
    use rand_pcg::{Pcg32, Pcg64, Pcg64Dxsm, Pcg64Mcg};
    use std::process;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Cli {
        /// Sample space, comma separated list
        #[arg(short, long, allow_hyphen_values=true)]
        omega: Option<String>,
    
        /// Law, comma separated list of values 
        #[arg(short, long, allow_hyphen_values=false)]
        law: Option<String>,
    
        /// Repeatitions of simulation
        #[arg(short, default_value_t = 1)]
        n: usize,
    
        /// RNG (Random number generator)
        #[arg(short, long, default_value_t = String::from("chacha"))]
        rng: String,

        /// Print  informative/debug output
        # [arg(short, long, default_value_t = false)]
        verbose: bool,

        /// RNG Seed (unsigned int) [default value from random default generator]
        # [arg(short, long)]
        seed: Option<u64>,

        /// list of available random numbers generators (RNG).
        # [arg(long="rng-list")]
        rnglist: bool,
    }

// Unfortunately, attribute macro enum_dispatch can't do that on extern trait.
macro_rules! rng_choice{
    (
        $($rngid:ident, $rng:ident, $desc:literal)*
    )=>{
            pub static ALLOWED_RNGS: &[&str] = &[
                $( stringify!($rngid), )*
            ];

            pub static DESC_RNGS: &[&str] = &[
                $( $desc, )*
            ];

            #[derive(Debug)]
            pub enum RngChoice { 
                    $(
                        $rng($rng),
                    )* 
                } 
        
            impl RngCore for RngChoice {
                fn next_u32(&mut self) -> u32 {
                    match self {
                        $( 
                            RngChoice::$rng(r) => r.next_u32(),
                        )*
                    }
                }
                fn next_u64(&mut self) -> u64 {
                    match self {
                        $( 
                            RngChoice::$rng(r) => r.next_u64(),
                        )*
                    }
                }
                fn fill_bytes(&mut self, dst: &mut [u8]) {
                    match self {
                        $( 
                            RngChoice::$rng(r) => r.fill_bytes(dst),
                        )*
                    }
                }

            }

            $(
            impl From<$rng> for RngChoice {
                fn from(inner: $rng) -> RngChoice {
                    RngChoice::$rng(inner)
                    }
                }
            )*

            impl RngChoice {
                pub fn new(id: &str, seed: u64) -> Self {
                    match id {
                        $(
                            stringify!($rngid) => $rng::seed_from_u64(seed).into(),
                        )*
                        _ => { 
                            println!("Unknown RNG <{}> ! Use --rnglist to see choices. ", id);
                            process::exit(1);
                        }
                    }
                }
            }
        
    }
}


rng_choice!(
    chacha, ChaCha20Rng, "ChaCha20 rng (default) (rand_chacha)."
    chacha8, ChaCha8Rng, "ChaCha8  Rng (rand_chacha)."
    chacha12, ChaCha12Rng, "ChaCha12 Rng (rand_chacha)."
    //chacha20, ChaCha20Rng, "ChaCha20 Rng (rand_chacha)."
    pcg32, Pcg32, "PCG Rng (XSH RR 64/32 (LCG) variant) (rand_pcg)."
    pcg64, Pcg64, "PCG Rng (XSL RR 128/64 (LCG) variant) (rand_pcg)."
    pcg64dxm, Pcg64Dxsm, "PCG Rng (CM DXSM 128/64 (LCG) variant) (rand_pcg)."
    pcg64mcg, Pcg64Mcg, "PCG Rng (XSL 128/64 (MCG) variant). (rand_pcg)."
);

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
        pub rng: RngChoice,
        pub rng_id: String,
        pub rng_seed: u64,
        pub verbose: bool
    }
    impl Config {
        pub fn new() -> Self {
            let cli = Cli::parse();
            if cli.rnglist {
                for i in 0..ALLOWED_RNGS.len() {
                println!("{} : {}", ALLOWED_RNGS[i], DESC_RNGS[i]);
                }
                process::exit(0);
            }

            let verbose= cli.verbose;

            if verbose {
                println!("{:?}", cli);
            }

            let omega = match &cli.omega {
                Some(omega) => parse_omega(&omega, verbose),
                None => {
                    println!("--omega <OMEGA> samples space mandatory argument !");
                    process::exit(1);
                }
            };

            let law = parse_law(&cli, &omega, verbose);
            let rng_seed = match cli.seed {
                Some(v) => v,
                None => rand::random::<u64>()
            };

            let rng_id= String::from(cli.rng);
            let rng = RngChoice::new(&rng_id, rng_seed);

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

