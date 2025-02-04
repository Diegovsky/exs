use exs::tsp::Solution;
use exs::{debug_to_kw, open_file, Graph, GraphMat, Node, Weight};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;
use std::f64::consts::E;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Params {
    pub i_max: usize,
    pub idle_max: usize,
    pub a: f64,
}

fn eval_candidate(g: &dyn Graph, last_node: Node, candidate: Node) -> Weight {
    g.get_edge_weight(last_node, candidate).unwrap()
}

fn random_greedy_solution<'g>(g: &'g dyn Graph, a: f64, rand: &mut impl rand::Rng) -> Solution<'g> {
    let mut s = Vec::<Node>::new();
    let mut lc = (0..g.node_count())
        .into_iter()
        .map(|n| n as Node)
        .collect_vec();
    s.push(lc.remove(0));
    while !lc.is_empty() {
        let last_node = *s.last().unwrap();

        // Cria a lrc com candidatos e seus g's
        let mut lrc = lc
            .iter()
            .copied()
            .map(|n| (n, eval_candidate(g, last_node, n)))
            .collect_vec();
        // ordena por valor de g
        lrc.sort_unstable_by_key(|k| k.1);
        let worst = lrc[0].1;
        let best = lrc[lrc.len() - 1].1;
        let cutoff = Weight::from(a) * (best - worst) + worst;

        // Remove candidatos cujo g não atende os parametros
        // nesse caso é <= pois é um problema de min
        lrc.retain(|(_, weight)| *weight <= cutoff);

        // escolhe candidato aleatorialemente
        let &(node, _) = lrc.choose(rand).unwrap();
        // remove candidato da LC
        lc.retain_mut(|el| *el != node);

        s.push(node);
    }
    Solution::new(s, g)
}

fn greedy_search(s: Solution) -> Solution {
    let mut s_best = s.clone();

    let all_nodes = 0..(s.nodes.len() as Node);
    loop {
        let mut improved = false;
        for (a, b) in all_nodes
            .clone()
            // itera sobre todos as combinações de N x N nós.
            .cartesian_product(all_nodes.clone())
        {
            // remove não-troca
            if a == b {
                continue;
            }

            let s_prime = s.swap(a as _, b as _);
            // pela elisão de um break, essa função realiza best improvement
            if s_prime < s_best {
                improved = true;
                s_best = s_prime;
            }
        }
        // não há vizinho melhor, estamos no pico local
        if !improved {
            break;
        }
    }
    s_best
}

fn run(g: &dyn Graph, params: &Params) -> (Duration, Weight) {
    let Params { i_max, a, idle_max } = *params;

    let mut s_best = None;

    let mut rand = rand::thread_rng();

    let mut idle = 0;

    let now = Instant::now();
    for i in 0.. {
        let s = random_greedy_solution(g, a, &mut rand);
        let s = greedy_search(s);

        // Na primeira iteração não há uma solução melhor ainda
        let Some(ref mut s_best) = s_best else {
            // Então, se estivermos na primeira iteração, seu s será o best.
            s_best = Some(s);

            continue;
        };

        if s < *s_best {
            *s_best = s;
            idle = 0;
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
    }
    let runtime = now.elapsed();

    (runtime, s_best.unwrap().value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    let alt = false;

    let params = if alt {
        Params {
            i_max: 0,
            idle_max: 300,
            a: 0.15,
        }
    } else {
        Params {
            i_max: 300,
            idle_max: 0,
            a: 0.15,
        }
    };
    println!("{}", debug_to_kw(&params));

    println!("runtime;cost");
    for _ in 0..10 {
        let (runtime, objective_func) = run(&graph, &params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
