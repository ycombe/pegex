use brouillon::configuration::Config;
use discrete_law::DiscreteFiniteRandomExperiment;
use rand::distr::Distribution;

fn main() {
    let mut conf = Config::new();

    if conf.verbose {
        println!("{:?}", conf);
    }

    let exp = DiscreteFiniteRandomExperiment::new(conf.omega, &conf.law);

    for _ in 0..conf.n {
        println!("{}", exp.sample(&mut conf.rng))
    }
}