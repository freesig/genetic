use {sort_by_fitness, Problem};

struct TestProblem;

impl Problem for TestProblem {
    type Individual = Vec<f64>;
    fn initial_pop(&mut self) -> Vec<Self::Individual> {
        unimplemented!()
    }

    fn fitness(&mut self, population: &Self::Individual) -> f64 {
        population.iter().sum()
    }

    fn crossover(&mut self, a: &Self::Individual, b: &Self::Individual) -> (Self::Individual, Self::Individual) {
        let cut_at = 1;
        let mut c = a[..cut_at].to_vec();
        let mut x = b[cut_at..].to_vec();
        let mut y = a[cut_at..].to_vec();
        c.append(&mut x);
        let mut d = b[..cut_at].to_vec();
        d.append(&mut y);
        (c, d)
    }
    
    fn mutate(&mut self, _individual: &mut Self::Individual){
        unimplemented!()
    }
}

fn setup() -> TestProblem {
    TestProblem {}
}

#[test]
fn evolve_sort() {
    let goal = vec![vec![0.1, 0.4], vec![0.2, 0.2], vec![0.1, 0.2]];
    let mut population = Vec::<Vec<f64>>::new();
    population.push(vec![0.1, 0.4]);
    population.push(vec![0.1, 0.2]);
    population.push(vec![0.2, 0.2]);
    let mut problem = setup();
    sort_by_fitness(&mut population, &mut problem);
    assert_eq!(goal, population);
}
