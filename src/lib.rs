extern crate rand;
use rand::prelude::*;
use std::f64::INFINITY;
use std::fmt::Debug;
use std::mem;
use std::ops::Range;

#[cfg(test)]
mod tests;

pub struct Settings {
    /// Number of individuals
    pub pop_size: usize,
    /// Number of top performers to pass through
    pub num_best: usize,
    /// The size of the tournament to
    /// select each pair of individuals
    pub tournament_size: usize,
    /// How likely a mutation is to occur
    pub mutation_rate: f32,
    /// This is_cycle the min and max of
    /// the population that the tournament selects from.
    /// The actual size is chosen at random but
    /// is within these bounds
    pub chunk_range: Range<usize>,
}

pub struct Genetic<T, I>
where
    T: Problem<Individual = I>,
{
    problem: T,
    population: Vec<I>,
    new_population: Vec<I>,
    pop_size: usize,
    num_best: usize,
    mutation_rate: f32,
    tournament_size: usize,
    chunk_range: Range<usize>,
    choices: Vec<usize>,
    rng: ThreadRng,
}

pub trait Problem {
    type Individual;
    fn initial_pop(&mut self, pop_size: usize) -> Vec<Self::Individual>;
    fn fitness(&mut self, individual: &Self::Individual) -> f64;
    fn crossover(
        &mut self,
        a: &Self::Individual,
        b: &Self::Individual,
    ) -> (Self::Individual, Self::Individual);
    fn mutate(&mut self, individual: &mut Self::Individual);
}

impl<T, I> Genetic<T, I>
where
    I: Clone + Default + Debug,
    T: Problem<Individual = I>,
{
    pub fn new(mut problem: T, settings: Settings) -> Self {
        let Settings {
            pop_size,
            mutation_rate,
            chunk_range,
            tournament_size,
            num_best,
        } = settings;
        assert!(pop_size % 2 == 0, "Population must be even");
        assert!(pop_size > 0, "Can't have no population");
        let population = problem.initial_pop(pop_size);
        let new_population = vec![I::default(); pop_size];
        assert!(pop_size == population.len());
        assert!(tournament_size > 0, "Can't have zero tournaments");
        assert!(
            mutation_rate >= 0.0 && mutation_rate <= 1.0,
            "Mutation rate needs to be between 0.0 and 1.0"
        );
        assert!(chunk_range.end <= pop_size, "Chunk range must be <= population");
        assert!(num_best <= pop_size, "Num best must be <= population");
        let rng = thread_rng();
        let choices = vec![0; pop_size];
        Genetic {
            problem,
            population,
            new_population,
            pop_size,
            mutation_rate,
            num_best,
            chunk_range,
            tournament_size,
            rng,
            choices,
        }
    }

    pub fn evolve(&mut self) {
        self.tournaments();
        self.breed();
        sort_by_fitness(&mut self.new_population, &mut self.problem);
        // Swap buffers
        mem::swap(&mut self.population, &mut self.new_population);
    }

    fn breed(&mut self) {
        let Self {
            num_best,
            pop_size,
            mutation_rate,
            ref mut rng,
            ref mut problem,
            ref mut new_population,
            ref population,
            ref choices,
            ..
        } = *self;
        for i in 0..num_best {
            new_population[i] = population.get(i).expect("outside bounds").clone();
        }
        for i in (num_best..(pop_size - 1)).step_by(2) {
            let (mut first_result, mut second_result) =
                problem.crossover(&population[choices[i]], &population[choices[i + 1]]);
            if rng.gen::<f32>() < mutation_rate {
                problem.mutate(&mut first_result)
            }
            if rng.gen::<f32>() < mutation_rate {
                problem.mutate(&mut second_result)
            }
            new_population[i] = first_result;
            new_population[i + 1] = second_result;
        }
    }

    pub fn get(&self) -> &Vec<I> {
        &self.population
    }

    pub fn take(self) -> Vec<I> {
        self.population
    }

    fn tournaments(&mut self) {
        for i in 0..self.pop_size {
            self.choices[i] = self.tournament();
        }
    }

    /// If k=1 then this is just random selection
    /// If k is too high compared to pop_size then
    /// you end up picking everyone and will mostly
    /// pick the best individuals.
    /// Aim for a k that picks more better individuals
    /// but allows for the chance to pick worse individuals
    fn tournament(&mut self) -> usize {
        let Self {
            pop_size,
            ref mut problem,
            ref population,
            ref chunk_range,
            tournament_size,
            ref mut rng,
            ..
        } = *self;
        // Start with worst possible score
        let mut best = -INFINITY;
        let mut best_i: Option<usize> = None;

        let chunk = make_chunk(pop_size, chunk_range, rng);

        for _i in 0..tournament_size {
            // Select random individual from population chunk
            let rand_i = rng.gen_range(chunk.start, chunk.end);
            let ind = problem.fitness(population.get(rand_i).unwrap());
            if ind >= best {
                best = ind;
                best_i = Some(rand_i);
            }
        }
        // Should always result in a best due to >= -INFINITY
        best_i.expect("Tournament failed")
    }

    pub fn stats(&mut self, n: usize) {
        let n = if n <= self.pop_size { n } else { self.pop_size };
        for i in 0..n {
            let fit = self.problem.fitness(&self.population[i]);
            println!("Rank {} Fitness {}", i + 1, fit);
        }
    }

    pub fn top(&mut self) -> f64 {
        self.problem.fitness(&self.population[0])
    }
}

fn sort_by_fitness<I, T: Problem<Individual = I>>(population: &mut Vec<I>, problem: &mut T) {
    population.sort_by(|a, b| problem.fitness(b).partial_cmp(&problem.fitness(a)).unwrap());
}

/// Creates a range of indicies randomly
/// from within the possible size
/// and places it randomly amoung the population
fn make_chunk(pop_size: usize, possible_size: &Range<usize>, rng: &mut ThreadRng) -> Range<usize> {
    assert!(possible_size.start > 0 && possible_size.end <= pop_size);
    // Chunk size needs to be (0..pop_size]
    let chunk_size = rng.gen_range(possible_size.start, possible_size.end);
    let chunk_start = rng.gen_range(0, pop_size - chunk_size);
    let chunk = chunk_start..(chunk_start + chunk_size);
    chunk
}
