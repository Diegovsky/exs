use exs::tsp::Solution;
use exs::{debug_to_kw, open_file, Graph, GraphMat, Weight};
use rand::Rng;
use std::f64::consts::E;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Params {
    pub i_max: usize,
    pub epsilon: f64,
    pub alpha: f64,
    pub temp0: f64,
    pub exponential_cooling: bool,
}

fn run(g: &dyn Graph, params: &Params) -> (Duration, Weight) {
    let Params {
        i_max,
        epsilon,
        alpha,
        temp0: mut temp,
        exponential_cooling,
    } = *params;

    // Solução inicial consiste em nós em órdem aleatória.
    let mut s = Solution::random(g);

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
    let mut file = open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    let params = Params {
        epsilon: 0.005,
        i_max: 10,
        temp0: 10.0,
        alpha: 0.9,
        exponential_cooling: false,
    };
    println!("{}", debug_to_kw(&params));

    println!("runtime;cost");
    for _ in 0..10 {
        let (runtime, objective_func) = run(&graph, &params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
