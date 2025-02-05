use rand::{seq::SliceRandom, Rng};

use crate::{Graph, Node, Weight};

pub type NodeList = Box<[Node]>;
#[derive(Clone)]
pub struct Solution<'g> {
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
impl std::cmp::Eq for Solution<'_> {}

impl std::cmp::Ord for Solution<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
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
