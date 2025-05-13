//! # Discrete law random experiment simulation
//! 
//! `discrete_law` contains utilities to simulate a random experiment with finite discrete sample space
//! 
//!
//! # Example:
//! ```
//! fn main() {
//!    let omega = ["A", "B", "C"];
//!    let ratios = [ 1.0, 1.0, 2.0];
//!    let exp = DiscreteFiniteRandomExperiment::new(omega.to_vec(), &ratios);
//!
//!    let rep: usize = 100_000;
//!    println!("{rep} repetitions.\n");
//!    println!("Fréquencies of A,B,C with probabilities 1/4,1/4,1/2 respectively, .");
//!    exp.print_simulation(rep);
//!
//!    let omega: Vec<usize> = (1..7).collect();
//!    let ratios =[ 1.0, 5.0, 5.0, 5.0, 5.0, 9.0];
//!    let exp = DiscreteFiniteRandomExperiment::new(omega, &ratios);
//!
//!    println!("Fréquencies of 1 to 6  with probabilities 1/30,1/6,1/6,1/6,1/6,3/10 respectively.");
//!    exp.print_simulation(100_000);
//!}
//! ```
//! 
//! `exp` implements `Distribution` trait so you can use `exp.sample(rng)` to get a sample.
//! 
//!  

use iter_accumulate::IterAccumulate;
use ordered_float::OrderedFloat;
use rand::distr::{Distribution, Uniform};
use std::collections::HashMap;
use std::hash::Hash;
use rand::Rng;


fn position(list: &[OrderedFloat<f64>], value: OrderedFloat<f64>) -> usize {
    match list.binary_search(&value) {
        Ok(i) | Err(i) => i
    }
}

fn cdf_from (ratios: &[f64]) -> Vec<OrderedFloat<f64>> {
    // let mut cdf: Vec<f64> = Vec::new();

    // accumulation pattern
    // see iter_accumulate crate
    // let mut last = 0.0;
    // for r in ratios {
    //     let new = r + last;
    //     cdf.push(new);
    //     last = new;
    // }
   let mut cdf: Vec<OrderedFloat<f64>> = ratios.iter()
        .accumulate(OrderedFloat(0.0), |acc, item| acc + *item)
        .collect();

    // normalization to get probability
    let total = cdf[cdf.len()-1];
    //for v in &mut cdf {
    //    *v = *v / total;
    //}
    cdf.iter_mut()
        .for_each(|x| *x = *x/total);

    cdf
}


/// Discrete distribution struct
/// Contains the probability law and it's cumulative distribution.
/// The cumulative distribution contains OrderedFloat because of use of binary_search to find the index from the value.
#[derive(Debug)]
pub struct DiscreteFiniteDistribution {
    _law: Vec<f64>,
    cdf:  Vec<OrderedFloat<f64>>
}

/// Distribution for the probability law.
impl DiscreteFiniteDistribution {
    pub fn new( law: &[f64] ) -> Self {
        DiscreteFiniteDistribution { 
            _law: law.to_vec(), 
            cdf: cdf_from( law)
        }
    }

//    pub fn sample(&self) -> usize {
//        let u: OrderedFloat<f64> = OrderedFloat(random());
//        position(&self.cdf, u)
//    }

}

impl Distribution<usize> for DiscreteFiniteDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let u: OrderedFloat<f64> = OrderedFloat(rng.sample(Uniform::new(0.0, 1.0).unwrap()));
        position(&self.cdf, u)
    }
}

/// Simulate the experiment from sample space `omega` and law.
#[derive(Debug)]
pub struct DiscreteFiniteRandomExperiment<T> {
    pub omega: Vec<T>,
    pub distribution: DiscreteFiniteDistribution
}

/// Create the experiment from space sample `omega` and `law`
impl<T> DiscreteFiniteRandomExperiment<T> {
    pub fn new( omega: Vec<T>, law: &[f64]) -> Self {
        DiscreteFiniteRandomExperiment {
            omega,
            distribution: DiscreteFiniteDistribution::new(law)
        }
    }

//    pub fn sample(&self) -> &T {
//        &self.omega[self.distribution.sample()]
//    }
}

impl<T: Clone> Distribution<T> for DiscreteFiniteRandomExperiment<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        self.omega[Distribution::sample(&self.distribution, rng)].clone()
    }
}

/// utility to print frequencies of values in experiment repetition.
impl<T: std::fmt::Debug + Eq + Hash + Clone> DiscreteFiniteRandomExperiment<T> {
    pub fn print_simulation (&self, n: usize) {
        let mut table: HashMap<T, i32> = HashMap::new();
        let mut rng = rand::rng();

        for _ in 0..n {
            let o = self.sample(&mut rng);
            *table.entry(o).or_insert(0) += 1;
        }

        for o in &self.omega {
            println!("{:?}: {}", o, *table.get(o).unwrap_or(&0) as f64 / n as f64 );
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribution_check() {
        let piped_dice = 
                DiscreteFiniteRandomExperiment::new(vec![1,2,3,4,5,6], &vec![1.0,4.0,4.0,4.0,4.0,7.0]);
        assert!(piped_dice.distribution.cdf[0] - OrderedFloat(1.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.distribution.cdf[1] - OrderedFloat(5.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.distribution.cdf[2] - OrderedFloat(9.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.distribution.cdf[3] - OrderedFloat(13.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.distribution.cdf[4] - OrderedFloat(17.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.distribution.cdf[5] - OrderedFloat(1.0) <= OrderedFloat(f64::EPSILON));
        let r = piped_dice.sample(&mut rand::rng());
        assert!( piped_dice.omega.contains(&r) );     
     }
}
