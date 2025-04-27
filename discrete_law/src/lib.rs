use iter_accumulate::IterAccumulate;
use ordered_float::OrderedFloat;
use rand::distr::{Distribution, Uniform};
use std::collections::HashMap;
use std::hash::Hash;
use rand::{Rng, random};


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

#[derive(Debug)]
pub struct DiscreteFiniteDistribution {
    pub law: Vec<f64>,
    pub cdf:  Vec<OrderedFloat<f64>>
}

impl DiscreteFiniteDistribution {
    pub fn new( law: &[f64] ) -> Self {
        DiscreteFiniteDistribution { 
            law: law.to_vec(), 
            cdf: cdf_from( law)
        }
    }

    pub fn sample(&self) -> usize {
        let u: OrderedFloat<f64> = OrderedFloat(random());
        position(&self.cdf, u)
    }

}

impl Distribution<usize> for DiscreteFiniteDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let u: OrderedFloat<f64> = OrderedFloat(rng.sample(Uniform::new(0.0, 1.0).unwrap()));
        position(&self.cdf, u)
    }
}

#[derive(Debug)]
pub struct DiscreteFiniteRandomExperiment<T> {
    pub omega: Vec<T>,
    pub distribution: DiscreteFiniteDistribution
}

impl<T> DiscreteFiniteRandomExperiment<T> {
    pub fn new( omega: Vec<T>, law: &[f64]) -> Self {
        DiscreteFiniteRandomExperiment {
            omega,
            distribution: DiscreteFiniteDistribution::new(law)
        }
    }

    pub fn sample(&self) -> &T {
        &self.omega[self.distribution.sample()]
    }
}

impl<T: Clone> Distribution<T> for DiscreteFiniteRandomExperiment<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        self.omega[Distribution::sample(&self.distribution, rng)].clone()
    }
}

pub fn print_simulation<T: std::fmt::Debug + Eq + Hash> (experiment: &DiscreteFiniteRandomExperiment<T>, n: usize) {
    //let simulation: Vec<&T> = Vec::new();
    let mut table: HashMap<&T, i32> = HashMap::new();

    for _ in 0..n {
        let o = experiment.sample();
        //simuation.push(o);
        *table.entry(o).or_insert(0) += 1;
    }

    for o in &experiment.omega {
            println!("{:?}: {}", o, *table.get(o).unwrap_or(&0) as f64 / n as f64 );
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
        let r = piped_dice.sample();
        assert!( piped_dice.omega.contains(r) );     
     }
}
