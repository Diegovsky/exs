use exs::tsp::Solution;
use exs::{debug_to_kw, open_file, Edge, Graph, GraphMat, Node, Weight};
use itertools::Itertools;
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use std::f64::consts::E;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Params {
    pub i_max: usize,
    pub alpha: f64,
    pub beta: f64,
    pub evap: f64,
    pub reinforcement: f64,
    pub ant_count: usize,
}

fn remove_random(
    i: Node,
    s: &mut Vec<Node>,
    rand: &mut impl Rng,
    g: &dyn Graph,
    pheromones: &dyn Graph,
    params: &Params,
) -> Node {
    let distributions = s
        .iter()
        .map(|k| (pheromones[(i, *k)]).powf(params.alpha) * (g[(i, *k)].recip()).powf(params.beta));
    let k_index = rand::distributions::WeightedIndex::new(distributions)
        .unwrap()
        .sample(rand);
    s.remove(k_index)
}

fn ant_path<'g>(g: &'g dyn Graph, pheromones: &dyn Graph, params: &Params) -> Solution<'g> {
    let mut rand = rand::thread_rng();
    let mut s = (0..g.node_count() as Node).into_iter().collect::<Vec<_>>();
    let mut path = Vec::<Node>::new();

    // Escolhe cidade inicial aleatoriamente
    let mut i = s.remove(rand.gen_range(0..s.len()));
    path.push(i);
    while !s.is_empty() {
        let j = remove_random(i, &mut s, &mut rand, g, pheromones, params);
        path.push(j);
        i = j;
    }
    todo!()
}

fn run(g: &dyn Graph, params: &Params) -> (Duration, Weight) {
    let mut pheromones = GraphMat::default();
    pheromones.add_nodes(g.node_count());
    for Edge(u, v, _) in g.edges() {
        // feromonio inicial
        pheromones.add_edge(u, v, 1.into());
    }

    let mut pheromones: &mut dyn Graph = &mut pheromones;

    for _ in 0..params.i_max {
        // Escolhe melhor formiga
        let best = (0..params.ant_count)
            .into_iter()
            .map(|_| ant_path(g, pheromones, params))
            .max_by(|s1, s2| s1.value.total_cmp(&s2.value))
            .unwrap();

        // Evaporação
        for Edge(u, v, w) in pheromones.edges() {
            let new_weight = (Weight::from(1.0 - params.evap) * w).into();
            pheromones[(u, v)] = new_weight;
        }

        // Atualização das trilhas
        let n = best.nodes.len() as Node;
        let components = (0..n - 1)
            .into_iter()
            .map(|i| (i, i + 1))
            .chain(Some((n - 1, 0)));
        for (i, j) in components {
            pheromones[(i, j)] += Weight::from(params.reinforcement) / (best.value);
        }
    }
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    let params = Params {
        i_max: 10,
        alpha: 1.0,
        beta: 1.0,
        evap: 0.05,
        reinforcement: 1.0,
        ant_count: graph.node_count(),
    };

    let (runtime, objective_func) = run(&graph, &params);
    println!("{:?}\n{}", runtime.as_secs_f64(), objective_func);
    Ok(())
}
