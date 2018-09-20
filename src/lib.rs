extern crate rand;
use rand::prelude::*;
use std::f64::INFINITY;
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
    fn initial_pop(&mut self) -> Vec<Self::Individual>;
    fn fitness(&self, individual: &Self::Individual) -> f64;
    fn crossover(
        &mut self,
        a: &Self::Individual,
        b: &Self::Individual,
    ) -> (Self::Individual, Self::Individual);
    fn mutate(&mut self, individual: &mut Self::Individual);
}

impl<T, I> Genetic<T, I>
where
    I: Clone,
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
        let population = problem.initial_pop();
        assert!(pop_size % 2 == 0);
        assert!(pop_size == population.len());
        let rng = thread_rng();
        let choices = vec![0; pop_size];
        Genetic {
            problem,
            population,
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
        self.population = self.breed();
        sort_by_fitness(&mut self.population, &self.problem);
    }

    fn breed(&mut self) -> Vec<I> {
        let Self {
            num_best,
            pop_size,
            mutation_rate,
            ref mut rng,
            ref mut problem,
            ref population,
            ref choices,
            ..
        } = *self;
        //TODO reuse this buffer
        let mut new_population = Vec::<I>::with_capacity(pop_size);
        for i in 0..num_best {
            new_population.push(population.get(i).expect("outside bounds").clone());
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
            new_population.push(first_result);
            new_population.push(second_result);
        }
        new_population
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
            ref problem,
            ref population,
            ref chunk_range,
            tournament_size,
            ref mut rng,
            ..
        } = *self;
        let mut best = -INFINITY;
        let mut best_i: Option<usize> = None;
        let chunk_size: usize = rng.gen_range(chunk_range.start, chunk_range.end);
        let start_position: usize = rng.gen_range(0, pop_size - chunk_size);
        for _i in 0..tournament_size {
            let rand_i = rng.gen_range(start_position, start_position + chunk_size);
            let ind = problem.fitness(population.get(rand_i).unwrap());
            if ind >= best {
                best = ind;
                best_i = Some(rand_i);
            }
        }
        best_i.expect("Tournament failed")
    }

    pub fn stats(&self, n: usize) {
        let n = if n <= self.pop_size { n } else { self.pop_size };
        for i in 0..n {
            let fit = self.problem.fitness(&self.population[i]);
            println!("Rank {} Fitness {}", i + 1, fit);
        }
    }
}

fn sort_by_fitness<I, T: Problem<Individual = I>>(population: &mut Vec<I>, problem: &T) {
    population.sort_by(|a, b| problem.fitness(b).partial_cmp(&problem.fitness(a)).unwrap());
}
