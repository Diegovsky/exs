use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{Graph, Weight};

pub fn euclidean_distance(a: [Weight; 2], b: [Weight; 2]) -> Weight {
    let xd = a[0].abs_diff(b[0]);
    let yd = a[1].abs_diff(b[1]);
    let d = (xd.pow(2) + yd.pow(2)) as f64;
    let d = d.sqrt() + 0.5;
    d as Weight
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    First,
    Best,
}

pub struct Args {
    pub filename: String,
    pub mode: Mode,
    pub iter_steps: usize,
}

impl Args {
    pub fn open_file(&self) -> BufReader<File> {
        let file = File::open(&self.filename).expect("Falha ao abrir arquivo de entrada");
        BufReader::new(file)
    }
    pub fn from_argv() -> Self {
        let mut args: Vec<String> = std::env::args().skip(1).collect();
        if args.is_empty() {
            panic!("Esperava nome do aquivo de entrada");
        }
        let filename = args.remove(0);

        let mode = args
            .get(0)
            .map(|arg| match arg.as_str() {
                "first" => Mode::First,
                "best" => Mode::Best,
                _ => panic!("Modo inesperado: {arg}"),
            })
            .unwrap_or_default();

        // Condição de parada: qntd de iterações
        let iter_steps: usize = args
            .get(1)
            .map(|arg| arg.parse().expect("Esperava um número"))
            .unwrap_or(10);

        Self {
            mode,
            filename,
            iter_steps,
        }
    }
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
