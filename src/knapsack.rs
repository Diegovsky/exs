use std::io::BufRead;

pub use crate::Weight as UWeight;
pub type Weight = i32;

use bitvec::vec::BitVec;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: UWeight,
    pub value: UWeight,
}

pub fn read_knapsack(file: &mut dyn BufRead) -> std::io::Result<(UWeight, Vec<Item>)> {
    let mut nums = vec![];
    for line in file.lines() {
        // adiciona todos os números no vetor `nums`
        nums.extend(
            line?
                .split_whitespace()
                .map(|num| num.parse::<UWeight>().expect("Número inválido")),
        )
    }

    // Remove primeira linha
    let info = nums.drain(0..=1).collect::<Vec<_>>();
    let weight = info[1];
    Ok((
        weight,
        nums.chunks_exact(2)
            .map(|pair| Item {
                value: pair[0],
                weight: pair[1],
            })
            .collect(),
    ))
}

#[derive(Clone)]
pub struct Solution<'ks> {
    pub items: BitVec,
    pub value: Weight,
    pub knapsack: &'ks [Item],
    pub params: Params,
}

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub max_weight: UWeight,
    pub penalty: UWeight,
}

fn evaluate_solution(items: &BitVec, knapsack: &[Item], params: Params) -> Weight {
    let total_val = items
        .iter_ones()
        .map(|index| knapsack[index].value)
        .sum::<UWeight>();
    let excess = items
        .iter_ones()
        .map(|index| knapsack[index].weight)
        .sum::<UWeight>()
        .saturating_sub(params.max_weight);
    total_val as Weight - (params.penalty * excess) as Weight
}

impl<'ks> Solution<'ks> {
    pub fn new(knapsack: &'ks [Item], items: BitVec, params: Params) -> Self {
        Self {
            value: evaluate_solution(&items, knapsack, params),
            params,
            items,
            knapsack,
        }
    }
    pub fn greedy(knapsack: &'ks [Item], params: Params) -> Self {
        let mut sorted = knapsack.iter().enumerate().collect::<Vec<_>>();
        // ordena por valor do ítem
        sorted.sort_by_cached_key(|(i, item)| item.value);
        let mut solution = BitVec::new();
        while let Some((i, _)) = sorted.pop() {
            solution.set(i, true);
            if evaluate_solution(&solution, knapsack, params) < 0 {
                solution.set(i, false);
            }
        }
        Self::new(knapsack, solution, params)
    }
    pub fn total_value(&self) -> UWeight {
        self.items
            .iter_ones()
            .map(|index| self.knapsack[index].value)
            .sum()
    }
    pub fn total_weight(&self) -> UWeight {
        self.items
            .iter_ones()
            .map(|index| self.knapsack[index].weight)
            .sum()
    }
    pub fn flip(&self, index: usize) -> Self {
        let mut flipped = self.items.clone();
        let val = !flipped[index];
        flipped.set(index, val);
        Self::new(self.knapsack, flipped, self.params)
    }
    pub fn best_neighbour(&self, taboos: &mut BitVec, best_value: Weight) -> Option<Self> {
        let mut current_best = None;
        for i in 0..self.knapsack.len() {
            let s_prime = self.flip(i);
            // critério de aspiração
            if s_prime.value > best_value {
                return Some(s_prime);
            }
            // Se o movimento for taboo, olha o próximo
            if taboos[i] {
                continue;
            }
            match current_best {
                None => current_best = Some(s_prime),
                Some(cur_best) if s_prime > cur_best => current_best = Some(s_prime),
                _ => (),
            }
        }
        current_best
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
