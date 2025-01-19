use exs::cli::tsp_sa::{Args, Params};
use exs::tsp::Solution;
use exs::{Graph, GraphMat, Weight};
use rand::Rng;
use std::f64::consts::E;
use std::time::{Duration, Instant};

fn run(g: &dyn Graph, params: &Params) -> (Duration, Weight) {
    let mut temp = params.temp0;

    let i_max = params.i_max;
    let alpha = params.alpha;
    let epsilon = params.epsilon;

    // Solução inicial consiste em nós em órdem aleatória.
    let mut s = Solution::sequential(g);

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
        if params.exponential_cooling {
            temp *= alpha;
        } else {
            temp -= alpha;
        }
    }
    let runtime = now.elapsed();

    (runtime, s_best.value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv(Params {
        epsilon: 0.005,
        i_max: 10,
        temp0: 10.0,
        alpha: 0.9,
        exponential_cooling: false,
    })?;
    let params = &args.params;
    let temp0 = params.temp0;

    let i_max = params.i_max;
    let alpha = params.alpha;
    let epsilon = params.epsilon;
    println!("epsilon={epsilon}||i_max={i_max}||temp0={temp0}||alpha={alpha}");
    let mut file = args.open_file();
    let mut graph = GraphMat::default();
    exs::utils::fill_tsp_graph(&mut file, &mut graph)?;

    println!("runtime;cost");
    for _ in 0..10 {
        let (runtime, objective_func) = run(&graph, &params);
        println!("{:?};{}", runtime.as_secs_f64(), objective_func);
    }
    Ok(())
}
