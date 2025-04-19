use iter_accumulate::IterAccumulate;
use ordered_float::OrderedFloat;
use rand::random;
use std::collections::HashMap;
use std::hash::Hash;


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
pub struct DiscreteFiniteRandomExperiment<T: std::fmt::Debug + Eq + Hash> {
    omega: Vec<T>,
    //law: Vec<f64>,
    cdf: Vec<OrderedFloat<f64>>
}

impl<T: std::fmt::Debug + Eq + Hash> DiscreteFiniteRandomExperiment<T> {
    pub fn new( omega: Vec<T>, law: &[f64]) -> Self {
        DiscreteFiniteRandomExperiment {
            omega,
            //law: law.to_vec(),
            cdf: cdf_from(law) }
    }

    pub fn sample(&self) -> &T {
        let u: OrderedFloat<f64> = OrderedFloat(random());
        &self.omega[position(&self.cdf, u)]
    }

    pub fn print_simulation(&self, n: usize) {
        //let simulation: Vec<&T> = Vec::new();
        let mut table: HashMap<&T, i32> = HashMap::new();

        for _ in 0..n {
            let o = self.sample();
            //simuation.push(o);
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
        assert!(piped_dice.cdf[0] - OrderedFloat(1.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.cdf[1] - OrderedFloat(5.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.cdf[2] - OrderedFloat(9.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.cdf[3] - OrderedFloat(13.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.cdf[4] - OrderedFloat(17.0/24.0) <= OrderedFloat(f64::EPSILON));
        assert!(piped_dice.cdf[5] - OrderedFloat(1.0) <= OrderedFloat(f64::EPSILON));
        let r = piped_dice.sample();
        assert!( piped_dice.omega.contains(r) );     
     }
}
