#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Settings;
pub struct Progress;

#[derive(Debug)]
pub struct GA<T, I>
where
    T: GeneticOps<I>,
{
    input: T,
    population: Vec<I>
}

pub trait GeneticOps <I> {
    /// Initialize your population
    fn initialize(&self) -> Vec<I>;
    /// Crossover two individuals
    fn crossover();
    /// Evaluate this individual
    fn fitness(&self) -> f64;
}

impl<T, I> GA<T, I>
where
    T: GeneticOps<I>,
{
    pub fn new(input: T, settings: Settings) -> Self {
        let population = input.initialize();
        GA { input, population}
    }

    fn top(&self) -> Option<f64> {
        Some(self.input.fitness())
    }
}

impl<T, I> Iterator for GA<T, I> 
where
    T: GeneticOps<I>,
{
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.top()
    }
}
