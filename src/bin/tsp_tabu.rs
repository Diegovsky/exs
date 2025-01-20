use exs::tsp::Solution;
use exs::{debug_to_kw, open_file, Graph, GraphMat, Node, Weight};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Params {
    pub i_max: usize,
    pub tabu_memory: usize,
    pub idle_max: usize,
}

type TabuList = BTreeMap<(Node, Node), usize>;

fn tabu_get(tabu: &TabuList, mut a: Node, mut b: Node) -> Option<usize> {
    // garante ordenação entre A e B, tal que
    // tabu_get(tabu, a, b) == tabu_get(tabu, b, a)
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    tabu.get(&(a, b)).copied()
}

fn tabu_set(tabu: &mut TabuList, mut a: Node, mut b: Node, val: usize) {
    // garante ordenação entre A e B, tal que
    // tabu_set(tabu, a, b, val) == tabu_set(tabu, b, a, val)
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    tabu.insert((a, b), val);
}

fn next_neighbour<'g>(
    s: &Solution<'g>,
    tabu: &TabuList,
    s_best: &Solution<'g>,
) -> Option<((Node, Node), Solution<'g>)> {
    let all_nodes = 0..(s.nodes.len() as Node);
    let mut best_neighbour: Option<((Node, Node), Solution<'g>)> = None;
    for (a, b) in all_nodes
        .clone()
        // itera sobre todos as combinações de N x N nós.
        .cartesian_product(all_nodes)
    {
        // remove não-troca
        if a == b {
            continue;
        }

        let s_prime = s.swap(a as _, b as _);
        // aspiração: troca é uma melhora absoluta
        if s_prime < *s_best {
            return Some(((a, b), s_prime));
        }

        // troca está banida, olhar o próximo
        if tabu_get(tabu, a, b).is_some() {
            continue;
        }

        match best_neighbour {
            // existe um melhor, descobre se o vizinho atual é melhor
            Some(ref best) => {
                if s_prime < best.1 {
                    best_neighbour = Some(((a, b), s_prime));
                }
            }
            // melhor não escolhido ainda, aceita o primeiro
            None => best_neighbour = Some(((a, b), s_prime)),
        }
    }
    best_neighbour
}

fn run(g: &dyn Graph, params: &Params) -> (Duration, Weight) {
    let Params {
        i_max,
        tabu_memory,
        idle_max,
    } = *params;

    // Solução inicial consiste em nós em órdem sequencial
    let mut s = Solution::sequential(g);

    let mut s_best = s.clone();

    let mut tabu: TabuList = TabuList::new();

    let now = Instant::now();
    let mut idle = 0;
    for i in 0.. {
        let (swap_prime, s_prime) = next_neighbour(&s, &tabu, &s_best).unwrap_or_else(|| {
            // todos os movimentos estavam banidos, pega o tabu há mais tempo
            let ((a, b), _) = tabu
                .iter()
                .min_by_key(|(_move, tabu_turns)| **tabu_turns)
                .unwrap();
            ((*a, *b), s.swap(*a as _, *b as _))
        });

        // houve melhora
        if s_prime < s_best {
            s_best = s_prime.clone();
            idle = 0;
        // não houve
        } else {
            idle += 1;
        }

        // se idle_max != 0, quer dizer que estamos limitando por iterações sem melhoria
        if idle_max != 0 && idle >= idle_max {
            // quantidade de turnos sem melhora excedeu o parâmetro.
            break;
        }

        // se i_max != 0, quer dizer que estamos limitando por quantidade de iterações
        if i_max != 0 && i >= i_max {
            // quantidade de iterações excedeu o máximo.
            break;
        }

        tabu.retain(|_move, tabu_turns| {
            *tabu_turns -= 1;
            // remove trocas cujo tempo de vida se tornou 0
            *tabu_turns != 0
        });
        let (a, b) = swap_prime;
        tabu_set(&mut tabu, a, b, tabu_memory);
        // println!("({a}, {b}) {}", s_prime.value);
        // continua a busca a partir da solução encontrada
        s = s_prime;
    }
    let runtime = now.elapsed();

    (runtime, s_best.value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    // let params = Params {
    //     i_max: 5000,
    //     tabu_memory: 40,
    //     idle_max: 0,
    // };

    let params = Params {
        i_max: 0,
        tabu_memory: 100,
        idle_max: 50,
    };
    println!("{}", debug_to_kw(&params));

    println!("runtime;cost");
    let (runtime, objective_func) = run(&graph, &params);
    println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    Ok(())
}
