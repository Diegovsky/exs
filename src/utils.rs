use std::{
    io::{self, BufRead},
    ops::Sub,
};

use crate::{Graph, Weight};

pub fn euclidean_distance(a: [Weight; 2], b: [Weight; 2]) -> Weight {
    let xd = a[0].sub(b[0]).abs();
    let yd = a[1].sub(b[1]).abs();
    let d = (xd.powi(2) + yd.powi(2)) as f64;
    let d = d.sqrt() + 0.5;
    d.into()
}

pub fn fill_tsp_graph(file: &mut dyn BufRead, graph: &mut dyn Graph) -> io::Result<()> {
    let mut buf = String::new();
    loop {
        buf.clear();
        file.read_line(&mut buf)?;
        let buf = buf.trim();
        if buf == "NODE_COORD_SECTION" {
            break;
        }
    }
    let mut locations = vec![];
    loop {
        buf.clear();
        file.read_line(&mut buf)?;
        let buf = buf.trim();
        if buf == "EOF" {
            break;
        }
        let nums = buf
            .split(" ")
            .map(|i| i.trim().parse::<Weight>().expect("Número invalido"))
            .collect::<Vec<Weight>>();

        let [_id, x, y] = nums[..3] else {
            panic!("Expected at least 3 elements per line")
        };
        locations.push((graph.add_node(), [x, y]));
    }
    for (a, a_loc) in &locations {
        for (b, b_loc) in &locations {
            graph.add_edge(*a, *b, euclidean_distance(*a_loc, *b_loc))
        }
    }
    Ok(())
}
