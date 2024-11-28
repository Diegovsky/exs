use std::io::BufRead;

use crate::Weight;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: Weight,
    pub value: Weight,
}

pub fn read_knapsack(file: &mut dyn BufRead) -> std::io::Result<(Weight, Vec<Item>)> {
    let mut nums = vec![];
    for line in file.lines() {
        // adiciona todos os números no vetor `nums`
        nums.extend(
            line?
                .split_whitespace()
                .map(|num| num.parse::<Weight>().expect("Número inválido")),
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
