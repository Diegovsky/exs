use exs::tsp::Solution;
use exs::{debug_to_kw, open_file, Graph, GraphMat};
use rand::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct GAParams {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
}

fn initialize_population<'g>(graph: &'g dyn Graph, size: usize) -> Vec<Solution<'g>> {
    (0..size).map(|_| Solution::random(graph)).collect()
}

fn selection<'pops, 'g>(population: &'pops [Solution<'g>]) -> &'pops Solution<'g> {
    let weights: Vec<_> = population
        .iter()
        .map(|s| (10000.0 * s.value.into_inner()).recip())
        .collect();
    let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    &population[dist.sample(&mut rng)]
}

fn crossover<'g>(parent1: &Solution<'g>, parent2: &Solution<'g>) -> Solution<'g> {
    let mut rng = thread_rng();
    let point = rng.gen_range(1..parent1.nodes.len() - 1);
    let mut child_nodes = parent1.nodes[..point].to_vec();
    child_nodes.extend(
        parent2
            .nodes
            .iter()
            .filter(|&n| !child_nodes.contains(n))
            .collect::<Vec<_>>(),
    );
    Solution::new(child_nodes, parent1.graph)
}

fn mutate<'g>(solution: &mut Solution<'g>, rate: f64) {
    let mut rng = thread_rng();
    if rng.gen::<f64>() < rate {
        let (i, j) = {
            let len = solution.nodes.len();
            let a = rng.gen_range(0..len);
            let mut b = rng.gen_range(0..len);
            while a == b {
                b = rng.gen_range(0..len);
            }
            (a, b)
        };
        solution.nodes.swap(i, j);
        solution.reeval();
    }
}

fn run<'g>(graph: &'g dyn Graph, params: GAParams) -> (Duration, f64) {
    let GAParams {
        population_size,
        generations,
        mutation_rate,
    } = params;

    let mut population = initialize_population(graph, population_size);
    let now = Instant::now();

    for _ in 0..generations {
        let mut new_population = Vec::with_capacity(population_size);
        while new_population.len() < population_size {
            let parent1 = selection(&population);
            let parent2 = selection(&population);
            let mut offspring = crossover(parent1, parent2);
            mutate(&mut offspring, mutation_rate);
            new_population.push(offspring);
        }
        population = new_population;
    }

    let best_solution = population.into_iter().min().unwrap();
    let runtime = now.elapsed();
    (runtime, best_solution.value.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    let params = GAParams {
        population_size: 200,
        generations: 20,
        mutation_rate: 0.05,
    };

    println!("{}", debug_to_kw(&params));
    println!("runtime;value");
    for _ in 0..10 {
        let (runtime, objective_func) = run(&graph, params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
