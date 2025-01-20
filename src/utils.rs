use std::io::{self, BufRead};

use crate::{Graph, Weight};

pub fn euclidean_distance(a: [Weight; 2], b: [Weight; 2]) -> Weight {
    let xd = a[0].abs_diff(b[0]);
    let yd = a[1].abs_diff(b[1]);
    let d = (xd.pow(2) + yd.pow(2)) as f64;
    let d = d.sqrt() + 0.5;
    d as Weight
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
