use exs::knapsack::{read_knapsack, Item, Solution, UWeight, WithPenalty};
use exs::{debug_to_kw, open_file};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PParams {
    pub i_max: usize,
    pub tabu_memory: usize,
    pub idle_max: usize,
    pub penalty: UWeight,
}

type TabuList = BTreeMap<usize, usize>;

fn next_neighbour<'g>(
    s: &Solution<'g>,
    tabu: &TabuList,
    s_best: &Solution<'g>,
    max_weight: UWeight,
) -> Option<(usize, Solution<'g>)> {
    let all_flips = 0..s.knapsack.len();
    let mut best_neighbour: Option<(usize, Solution<'g>)> = None;
    for flip in all_flips {
        let s_prime = s.flip(flip);
        // aspiração: troca é uma melhora absoluta
        if s_prime > *s_best && s_prime.total_weight() <= max_weight {
            return Some((flip, s_prime));
        }

        // troca está banida, olhar o próximo
        if tabu.get(&flip).is_some() {
            continue;
        }

        match best_neighbour {
            // existe um melhor, descobre se o vizinho atual é melhor
            Some(ref best) => {
                if s_prime > best.1 {
                    best_neighbour = Some((flip, s_prime));
                }
            }
            // melhor não escolhido ainda, aceita o primeiro
            None => best_neighbour = Some((flip, s_prime)),
        }
    }
    best_neighbour
}

fn run(knapsack: &[Item], params: WithPenalty, pparams: PParams) -> (Duration, UWeight) {
    let PParams {
        i_max,
        tabu_memory,
        idle_max,
        ..
    } = pparams;

    let max_weight = params.max_weight;

    // Solução inicial consiste em uma busca gulosa;
    let mut s = Solution::greedy(knapsack, params);

    let mut s_best = s.clone();

    let mut tabu: TabuList = TabuList::new();

    let now = Instant::now();
    let mut idle = 0;
    for i in 0.. {
        let (flip_prime, s_prime) =
            next_neighbour(&s, &tabu, &s_best, max_weight).unwrap_or_else(|| {
                // todos os movimentos estavam banidos, pega o tabu há mais tempo
                let (&flip, _) = tabu
                    .iter()
                    .min_by_key(|(_, tabu_turns)| **tabu_turns)
                    .unwrap();
                (flip, s.flip(flip))
            });

        // houve melhora
        if s_prime > s_best && s_prime.total_weight() <= max_weight {
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
        // println!("{flip_prime} [{}]", s_prime.total_value());
        tabu.insert(flip_prime, tabu_memory);
        // continua a busca a partir da solução encontrada
        s = s_prime;
    }
    let runtime = now.elapsed();

    (runtime, s_best.total_value())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maxw, items) = read_knapsack(&mut open_file())?;
    let items = &*items;

    let pparams = PParams {
        i_max: 5000,
        tabu_memory: 50,
        idle_max: 0,
        penalty: 3,
    };

    // let pparams = PParams {
    //     i_max: 0,
    //     tabu_memory: items.len() / 2,
    //     idle_max: 10,
    //     penalty: 1,
    // };

    let params = WithPenalty {
        max_weight: maxw,
        penalty: pparams.penalty,
    };
    println!("{}", debug_to_kw(&pparams));

    println!("runtime;value");
    let (runtime, objective_func) = run(items, params, pparams);
    println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    Ok(())
}
