use bitvec::bitvec;
use exs::knapsack::{Solution, *};
use exs::{debug_to_kw, open_file};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct PParams {
    pub i_max: usize,
    pub idle_max: usize,
    pub a: f64,
    penalty: UWeight,
}

// g é a melhora em relação a solução atual
fn eval_candidate(s: &Solution, s_prime: &Solution) -> Weight {
    s_prime.value - s.value
}

fn random_greedy_solution<'g>(
    knapsack: &'g [Item],
    a: f64,
    rand: &mut impl rand::Rng,
    params: Params,
) -> Solution<'g> {
    // solução inicial vazia
    let mut s = Solution::new(knapsack, bitvec![0; knapsack.len()], params);

    let mut lc = knapsack.len();

    while lc != 0 {
        // Cria a lrc com candidatos e seus g's
        let mut lrc = (0..knapsack.len())
            .into_iter()
            .map(|flip| s.flip(flip))
            .map(|flipped| {
                let g = eval_candidate(&s, &flipped);
                (flipped, g)
            })
            .collect_vec();
        // ordena por valor de g
        lrc.sort_unstable_by_key(|k| k.1);
        let worst = lrc[0].1 as f64;
        let best = lrc[lrc.len() - 1].1 as f64;
        let cutoff = a * (best - worst) + worst;

        // Remove candidatos cujo g não atende os parametros
        // nesse caso é >= pois é um problema de max
        lrc.retain(|(_, weight)| *weight as f64 >= cutoff);

        // escolhe candidato aleatorialemente
        let (flipped, _) = lrc.choose_mut(rand).unwrap();
        // remove candidato da LC
        lc -= 1;

        // s = flipped;
        std::mem::swap(&mut s, flipped);
    }
    s
}

fn greedy_search(s: &mut Solution) {
    let mut sorted = s.knapsack.iter().enumerate().collect::<Vec<_>>();
    // ordena por valor do ítem
    sorted.sort_by_cached_key(|(_, item)| item.value);

    while let Some((i, _)) = sorted.pop() {
        let s_prime = s.flip(i);

        if s_prime > *s && s_prime.total_weight() <= s.total_weight() {
            *s = s_prime
        }
    }
}

fn run(knapsack: &[Item], pparams: PParams, params: Params) -> (Duration, UWeight) {
    let PParams {
        i_max, a, idle_max, ..
    } = pparams;

    let mut s_best = None;

    let mut rand = rand::thread_rng();

    let mut idle = 0;

    let now = Instant::now();
    for i in 0.. {
        let mut s = random_greedy_solution(knapsack, a, &mut rand, params);
        greedy_search(&mut s);

        // Na primeira iteração não há uma solução melhor ainda
        let Some(ref mut s_best) = s_best else {
            // Então, se estivermos na primeira iteração, seu s será o best.
            s_best = Some(s);

            continue;
        };

        if s > *s_best && s.total_weight() <= params.max_weight {
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

    (runtime, s_best.unwrap().total_value())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maxw, items) = read_knapsack(&mut open_file())?;
    let items = &*items;

    let alt = false;

    let pparams = if alt {
        PParams {
            i_max: 100,
            idle_max: 0,
            penalty: 1,
            a: 0.60,
        }
    } else {
        PParams {
            i_max: 0,
            idle_max: 80,
            penalty: 2,
            a: 0.20,
        }
    };

    let params = Params {
        max_weight: maxw,
        penalty: pparams.penalty as _,
    };

    println!("{}", debug_to_kw(&pparams));
    println!("runtime;value");
    for _ in 0..10 {
        let (runtime, objective_func) = run(items, pparams, params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
