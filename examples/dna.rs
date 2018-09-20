extern crate genetic;
extern crate rand;

use genetic::{Genetic, Problem, Settings};
use rand::prelude::*;

struct DNA {
    rng: ThreadRng,
}

impl Problem for DNA {
    type Individual = Vec<f64>;
    fn initial_pop(&mut self, pop_size: usize) -> Vec<Self::Individual> {
        let dna = vec![0.5, 0.6, 0.2, 0.4, 0.4, 0.7];
        let mut population = vec![dna.clone(); pop_size - 1];
        let dna = vec![0.1, 0.1, 0.2, 0.2, 0.1, 0.9];
        population.push(dna);
        population

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
        println!("Best: {:?}", ga.get().first().unwrap());
    }
    println!("Top: {}", ga.top());
    println!("Result: {:?}", ga.take());
}
