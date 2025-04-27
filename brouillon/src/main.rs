use std::collections::HashMap;
use discrete_law::DiscreteFiniteRandomExperiment;
use rand::rng;
use rand::prelude::Distribution;

fn main() {
    let rd_exp = DiscreteFiniteRandomExperiment::new(
        vec!["A","B","C"],
        &vec![0.60, 0.30, 0.1 ]
    );

    let n = 1000_000;
    let simu: Vec<&str> = (&rd_exp).sample_iter(rng()).take(n).collect();

    //println!("Hello, world!{:?}", simu);

    let mut table: HashMap<&str, i32> = HashMap::new();

    for s in simu {
        *table.entry(s).or_insert(0) += 1;
    }

    println!("{:?}", table);

    for (issue, ns) in &table {
        println!("{}: {}", *issue, *ns as f64 / n as f64 );
    }
}
