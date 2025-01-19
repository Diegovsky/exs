use std::time::Instant;

use bitvec::prelude::*;
use exs::{
    cli::ksp_sa::{Args, PParams},
    knapsack::{read_knapsack, Item, Params, Solution},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv(PParams {
        i_max: 100,
        penalty: 2,
    })?;
    let (maxw, items) = read_knapsack(&mut args.open_file())?;
    let items = &*items;
    let k = items.len();
    assert_ne!(k, 0);

    let mut best_solution: Solution = Solution::greedy(
        items,
        Params {
            max_weight: maxw,
            penalty: args.params.penalty as _,
        },
    );

    println!(
        "Solução inicial: {:?}\nValor: {}",
        best_solution.items, best_solution.value
    );
    let mut taboo = bitvec![];
    let now = Instant::now();
    for i in 0..args.params.i_max {
        todo!()
    }
    println!(
        "Solução final: {}\nValor: {}",
        best_solution.items, best_solution.value
    );
    println!("Tempo de execução: {:?}", now.elapsed());
    Ok(())
}
