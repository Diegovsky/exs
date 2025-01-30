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

pub trait EvaluationMethod: Sized + Clone + Copy {
    fn evaluate_solution(&self, solution: &Solution<'_, Self>) -> Weight;
    fn max_weight(&self) -> UWeight;
}

#[derive(Debug, Clone, Copy)]
pub struct WithPenalty {
    pub max_weight: UWeight,
    pub penalty: UWeight,
}

impl EvaluationMethod for WithPenalty {
    fn evaluate_solution(&self, solution: &Solution<'_, Self>) -> Weight {
        let total_val = solution.total_value();
        let excess = solution.total_weight().saturating_sub(self.max_weight);
        total_val as Weight - (self.penalty * excess) as Weight
    }
    fn max_weight(&self) -> UWeight {
        self.max_weight
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ByTotalValue {
    pub max_weight: UWeight,
}

impl EvaluationMethod for ByTotalValue {
    fn evaluate_solution(&self, solution: &Solution<'_, Self>) -> Weight {
        solution.total_value() as Weight
    }
    fn max_weight(&self) -> UWeight {
        self.max_weight
    }
}

#[derive(Clone)]
pub struct Solution<'ks, E: EvaluationMethod = WithPenalty> {
    pub items: BitVec,
    pub value: Weight,
    pub knapsack: &'ks [Item],
    pub eval_method: E,
}

impl<'ks, E> Solution<'ks, E>
where
    E: EvaluationMethod,
{
    pub fn new(knapsack: &'ks [Item], items: BitVec, eval_method: E) -> Self {
        let mut this = Self {
            value: 0,
            eval_method,
            items,
            knapsack,
        };
        this.value = this.eval_method.evaluate_solution(&this);
        this
    }
    pub fn empty(knapsack: &'ks [Item], eval_method: E) -> Self {
        Self::new(knapsack, bitvec::bitvec![0; knapsack.len()], eval_method)
    }
    pub fn greedy(knapsack: &'ks [Item], eval_method: E) -> Self {
        let mut this = Self::empty(knapsack, eval_method);

        let mut sorted = knapsack.iter().enumerate().collect::<Vec<_>>();
        // ordena por valor do ítem
        sorted.sort_by_cached_key(|(_, item)| item.value);
        while let Some((i, _)) = sorted.pop() {
            let new = this.flip(i);
            if this < new {
                this = new
            }
        }
        this
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
        Self::new(self.knapsack, flipped, self.eval_method)
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

impl<E: EvaluationMethod> std::fmt::Debug for Solution<'_, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.items)
    }
}

impl<E: EvaluationMethod> std::cmp::PartialEq for Solution<'_, E> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<E: EvaluationMethod> std::cmp::PartialOrd for Solution<'_, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
