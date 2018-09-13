extern crate genetic;

use genetic::{GeneticOps, Settings, GA};

#[derive(Debug)]
struct Fred;
#[derive(Debug, Clone)]
struct DNA(f64, f64, f64, f64, f64);

impl GeneticOps<DNA> for Fred {
    fn initialize(&self) -> Vec<DNA> {
        vec![DNA(1.0, 1.0, 1.0, 1.0, 1.0); 5]
    }
    fn crossover() {}
    fn fitness(&self) -> f64 { 1.0 }
}

fn setup() -> GA<Fred, DNA> {
    let evolve_me = Fred;
    let settings = Settings;
    GA::new(evolve_me, settings)
}

#[test]
fn create() {
    let ga_1 = setup();
    let ga_2 = setup();
    assert_eq!(format!("{:?}", ga_1), format!("{:?}", ga_2));
}

#[test]
fn one_gen_no_progress() {
    let mut ga = setup();
    assert_eq!(1.0, ga.next().unwrap());
}
