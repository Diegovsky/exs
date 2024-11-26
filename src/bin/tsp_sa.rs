use core::f64;
use exs::utils::{Args, Defaults};
use exs::{Graph, GraphMat, Node, Weight};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use std::f64::consts::E;
use std::time::Instant;

pub type NodeList = Box<[Node]>;

#[derive(Clone)]
struct Solution<'g> {
    pub nodes: NodeList,
    pub value: Weight,
    graph: &'g dyn Graph,
}

// Implementa interfaces de comparação
// Dessa forma, é possível realizar comparações entre soluções
impl std::cmp::PartialEq for Solution<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl std::cmp::PartialOrd for Solution<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

// Função de avaliação
fn solution_value(solution: &[Node], g: &dyn Graph) -> Weight {
    let first = *solution.first().unwrap();
    let last = *solution.last().unwrap();
    solution
        .windows(2)
        .map(|window| g.get_edge_weight(window[0], window[1]).unwrap())
        .sum::<Weight>()
        + g.get_edge_weight(last, first).unwrap()
}

impl<'g> Solution<'g> {
    pub fn new(nodes: impl Into<Box<[Node]>>, graph: &'g dyn Graph) -> Self {
        let nodes = nodes.into();
        Self {
            value: solution_value(&*nodes, graph),
            nodes: nodes.into(),
            graph,
        }
    }

    pub fn sequential(g: &'g dyn Graph) -> Self {
        Self::new((0..g.node_count() as Node).collect::<Box<[_]>>(), g)
    }

    pub fn random(graph: &'g dyn Graph) -> Self {
        let k = graph.node_count() as Node;
        let mut nodes: Vec<Node> = (0..k).collect();
        nodes.shuffle(&mut rand::thread_rng());
        Self::new(nodes, graph)
    }

    pub fn swap(&self, a: usize, b: usize) -> Self {
        let mut nodes = self.nodes.clone();
        nodes.swap(a, b);
        Self::new(nodes, self.graph)
    }

    pub fn random_neighbour(&self, rand: &mut impl Rng) -> Self {
        let span = 0..self.nodes.len();
        let a = rand.gen_range(span.clone());
        let mut b;
        loop {
            b = rand.gen_range(span.clone());
            if b != a {
                break;
            }
        }
        self.swap(a, b)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv(Defaults {
        epsilon: 0.005,
        i_max: 10,
        temp0: 10.0,
        alpha: 0.9,
    })?;
    let mut file = args.open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    let mut temp = args.temp0;

    let i_max = args.i_max;
    let alpha = args.alpha;
    let epsilon = args.epsilon;

    // Solução inicial consiste em nós em órdem aleatória.
    let mut s = Solution::sequential(&graph);

    let mut s_best = s.clone();

    let mut rand = rand::thread_rng();

    let now = Instant::now();
    while temp > epsilon {
        for _ in 0..i_max {
            let s_prime = s.random_neighbour(&mut rand);

            if s_prime < s {
                s = s_prime;
                if s < s_best {
                    s_best = s.clone();
                }
            } else if rand.gen::<f64>() < E.powf((s.value as f64 - s_prime.value as f64) / temp) {
                s = s_prime;
            }
        }
        temp *= alpha;
    }
    println!(
        "Solução final: {:?}\n\
             Valor: {}",
        s_best.nodes, s_best.value
    );
    println!("Tempo de execução: {:?}", now.elapsed());
    Ok(())
}
