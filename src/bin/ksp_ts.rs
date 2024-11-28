use std::time::Instant;

use bitvec::prelude::*;
use exs::Weight;
use exs::{
    cli::ksp_ts::{Args, Defaults},
    knapsack::{read_knapsack, Item},
};

#[derive(Clone, Default)]
struct Solution<'ks> {
    items: BitVec,
    value: Weight,
    knapsack: &'ks [Item],
}

fn evaluate_solution(items: &BitVec, knapsack: &[Item]) -> Weight {
    todo!()
}

impl<'ks> Solution<'ks> {
    fn new(knapsack: &'ks [Item], items: BitVec) -> Self {
        Self {
            value: evaluate_solution(&items, knapsack),
            items,
            knapsack,
        }
    }
    fn greedy(knapsack: &'ks [Item]) -> Self {
        let mut sorted = knapsack.to_vec();
        todo!()
    }
    fn total_value(&self) -> Weight {
        self.items
            .iter_ones()
            .map(|index| self.knapsack[index].value)
            .sum()
    }
    fn total_weight(&self) -> Weight {
        self.items
            .iter_ones()
            .map(|index| self.knapsack[index].weight)
            .sum()
    }
    fn flip(&self, index: usize) -> Self {
        let mut flipped = self.items.clone();
        let val = !flipped[index];
        flipped.set(index, val);
        Self::new(self.knapsack, flipped)
    }
}

impl std::fmt::Debug for Solution<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.items)
    }
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_argv(Defaults { i_max: 100 })?;
    let (maxw, items) = read_knapsack(&mut args.open_file())?;
    let items = &*items;
    let k = items.len();
    assert_ne!(k, 0);

    let mut best_solution: Solution = Solution::greedy(items);

    println!(
        "Solução inicial: {:?}\nValor: {}",
        best_solution.items, best_solution.value
    );
    let now = Instant::now();
    for i in 0..args.i_max {}
    println!(
        "Solução final: {}\nValor: {}",
        best_solution.items, best_solution.value
    );
    println!("Tempo de execução: {:?}", now.elapsed());
    Ok(())
}
