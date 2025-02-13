use exs::knapsack::{read_knapsack, ByTotalValue, Item, UWeight};
use exs::{debug_to_kw, open_file};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::*;
use std::time::{Duration, Instant};

type Solution<'ks> = exs::knapsack::Solution<'ks, ByTotalValue>;

#[derive(Debug, Clone, Copy)]
pub struct GAParams {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_chance: f64,
}

fn initialize_population<'ks>(
    knapsack: &'ks [Item],
    params: ByTotalValue,
    size: usize,
) -> Vec<Solution<'ks>> {
    (0..size)
        .map(|_| Solution::random(knapsack, params))
        .collect()
}

// Seleciona uma população aleatóriamente pelo método da roleta
fn selection<'pops, 'ks>(population: &'pops [Solution<'ks>]) -> &'pops Solution<'ks> {
    let weights: Vec<_> = population.iter().map(|s| s.value.into_inner()).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    &population[dist.sample(&mut rng)]
}

// Realiza um crossover de corte único
fn crossover<'ks>(parent1: &Solution<'ks>, parent2: &Solution<'ks>) -> Solution<'ks> {
    let mut rng = thread_rng();
    let crossover_point = rng.gen_range(0..parent1.knapsack.len());
    let mut new_items = parent1.items.clone();
    new_items[..crossover_point].copy_from_bitslice(&parent2.items[..crossover_point]);
    Solution::new(parent1.knapsack, new_items, parent1.eval_method)
}

// Realiza mutações na solução de acordo com a chance de mutação
fn mutate<'ks>(solution: &mut Solution<'ks>, mutation_chance: f64) {
    let mut rng = thread_rng();
    for i in 0..solution.knapsack.len() {
        if rng.gen::<f64>() < mutation_chance {
            *solution = solution.flip(i);
        }
    }
    // é necessário re-avaliar a expressão
    solution.reeval();
}

fn run<'ks>(
    knapsack: &'ks [Item],
    params: ByTotalValue,
    ga_params: GAParams,
) -> (Duration, UWeight) {
    let GAParams {
        population_size,
        generations,
        mutation_chance,
    } = ga_params;

    // cria várias populações aleatórias
    let mut population = initialize_population(knapsack, params, population_size);
    let now = Instant::now();

    for _ in 0..generations {
        let mut new_population = Vec::with_capacity(population_size);
        while new_population.len() < population_size {
            let parent1 = selection(&population);
            let parent2 = selection(&population);
            let mut offspring = crossover(parent1, parent2);
            mutate(&mut offspring, mutation_chance);
            new_population.push(offspring);
        }
        population = new_population;
    }

    let best_solution = population.into_iter().max_by_key(|s| s.value).unwrap();
    let runtime = now.elapsed();
    (runtime, best_solution.value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maxw, items) = read_knapsack(&mut open_file())?;
    let items = &*items;

    let ga_params = GAParams {
        population_size: 10,
        generations: 20,
        mutation_chance: 0.05,
    };

    let params = ByTotalValue { max_weight: maxw };

    println!("{}", debug_to_kw(&ga_params));
    println!("runtime;value");
    for _ in 0..10 {
        let (runtime, objective_func) = run(items, params, ga_params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
