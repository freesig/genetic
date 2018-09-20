extern crate genetic;
extern crate rand;

use genetic::{Genetic, Problem, Settings};
use rand::prelude::*;

struct DNA {
    rng: ThreadRng,
}

impl Problem for DNA {
    type Individual = Vec<f64>;
    fn initial_pop(&mut self) -> Vec<Self::Individual> {
        let dna = vec![0.5, 0.6, 0.2, 0.4, 0.4, 0.7];
        vec![dna.clone(); 14]
    }
    fn fitness(&mut self, individual: &Self::Individual) -> f64 {
        // Distance from 0.5
        individual
            .iter()
            .fold(0.0, |dist, x| dist + (0.5 - x).abs())
    }
    fn crossover(
        &mut self,
        a: &Self::Individual,
        b: &Self::Individual,
    ) -> (Self::Individual, Self::Individual) {
        let cut_at = self.rng.gen_range(0, a.len());
        let mut c = a[..cut_at].to_vec();
        let mut x = b[cut_at..].to_vec();
        let mut y = a[cut_at..].to_vec();
        c.append(&mut x);
        let mut d = b[..cut_at].to_vec();
        d.append(&mut y);
        (c, d)
    }
    fn mutate(&mut self, individual: &mut Self::Individual) {
        let i = self.rng.gen_range(0, individual.len());
        individual[i] += 0.01;
    }
}

fn main() {
    let problem = DNA { rng: thread_rng() };
    let settings = Settings {
        mutation_rate: 0.01,
        pop_size: 14,
        num_best: 0,
        tournament_size: 10,
        chunk_range: 2..4,
    };
    let generations = 100;
    let mut ga = Genetic::new(problem, settings);
    for _i in 0..generations {
        ga.evolve();
        ga.stats(5);
        println!("best: {:?}", ga.get().first().unwrap());
    }
    println!("Result: {:?}", ga.take());
}
