use std::f64::consts::E;
use std::time::{Duration, Instant};

use exs::debug_to_kw;
use exs::knapsack::{Item, Weight};

use exs::{
    knapsack::{read_knapsack, Solution, WithPenalty},
    open_file,
};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct PParams {
    pub i_max: usize,
    pub epsilon: f64,
    pub alpha: f64,
    pub temp0: f64,
    pub exponential_cooling: bool,
    pub penalty: Weight,
}

fn run(knapsack: &[Item], params: WithPenalty, pparams: PParams) -> (Duration, Weight) {
    let PParams {
        i_max,
        epsilon,
        alpha,
        temp0: mut temp,
        exponential_cooling,
        ..
    } = pparams;

    // Solução inicial é gulosa.
    let mut s = Solution::greedy(knapsack, params);
    let mut s_best = s.clone();

    let mut rand = rand::thread_rng();

    let now = Instant::now();
    while temp > epsilon {
        for _ in 0..i_max {
            let random_index = rand.gen_range(0..knapsack.len());
            let s_prime = s.flip(random_index);

            if s_prime > s {
                s = s_prime;
                if s > s_best {
                    s_best = s.clone();
                }
            } else if rand.gen::<f64>() < E.powf(*(s.value - s_prime.value) / temp) {
                s = s_prime;
            }
        }

        if exponential_cooling {
            temp *= alpha;
        } else {
            temp -= alpha;
        }
    }

    let runtime = now.elapsed();

    (runtime, s_best.total_value())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maxw, items) = read_knapsack(&mut open_file())?;
    let items = &*items;

    let pparams = PParams {
        epsilon: 0.005,
        i_max: 10,
        temp0: 1000.0,
        alpha: 0.9995,
        exponential_cooling: false,
        penalty: 2.into(),
    };

    let params = WithPenalty {
        max_weight: maxw,
        penalty: pparams.penalty,
    };

    println!("{}", debug_to_kw(&pparams));
    println!("runtime;value");
    for _ in 0..10 {
        let (runtime, objective_func) = run(items, params, pparams);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
