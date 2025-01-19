use std::f64::consts::E;
use std::time::{Duration, Instant};

use exs::debug_to_kw;
use exs::knapsack::Item;
use exs::knapsack::UWeight;
use exs::knapsack::Weight;

use bitvec::prelude::*;
use exs::{
    knapsack::{read_knapsack, Params, Solution},
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
    pub penalty: usize,
}

fn run(knapsack: &[Item], params: Params, pparams: PParams) -> (Duration, i32) {
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

            if s_prime < s {
                s = s_prime;
                if s < s_best {
                    s_best = s.clone();
                }
            } else if rand.gen::<f64>() < E.powf((s.value as f64 - s_prime.value as f64) / temp) {
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

    (runtime, s_best.value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maxw, items) = read_knapsack(&mut open_file())?;
    let items = &*items;

    let pparams = PParams {
        epsilon: 0.005,
        i_max: 10,
        temp0: 10.0,
        alpha: 0.9,
        exponential_cooling: false,
        penalty: 2,
    };

    let params = Params {
        max_weight: maxw,
        penalty: pparams.penalty as _,
    };

    println!("{}", debug_to_kw(&pparams));
    println!("runtime;cost");
    for _ in 0..10 {
        let (runtime, objective_func) = run(items, params, pparams);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
