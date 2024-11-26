use std::{
    ffi::OsString,
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

#[derive(Debug)]
pub struct Defaults {
    pub i_max: usize,
    pub epsilon: f64,
    pub alpha: f64,
    pub temp0: f64,
}

pub struct Args {
    pub filename: OsString,
    pub i_max: usize,
    pub epsilon: f64,
    pub alpha: f64,
    pub temp0: f64,
}

impl Args {
    pub fn open_file(&self) -> BufReader<File> {
        let file = File::open(&self.filename).expect("Falha ao abrir arquivo de entrada");
        BufReader::new(file)
    }
    pub fn from_argv(defaults: Defaults) -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_env();
        let mut filename = None;
        let mut i_max = 10;
        let mut epsilon: f64 = defaults.epsilon;
        let mut alpha: f64 = defaults.alpha;
        let mut temp0: f64 = defaults.temp0;

        while let Some(arg) = parser.next()? {
            match arg {
                Value(fname) if filename.is_none() => {
                    filename = Some(fname);
                }
                Value(inv) => {
                    return Err(format!("Não esperava o argumento {:?}", inv))?;
                }
                Short('i') | Long("i-max") => i_max = parser.value()?.parse()?,
                Short('e') | Long("epsilon") => epsilon = parser.value()?.parse()?,
                Short('a') | Long("alpha") => alpha = parser.value()?.parse()?,
                Short('t') | Long("temp0") => temp0 = parser.value()?.parse()?,
                Short('h') | Long("help") => {
                    eprintln!(
                        include_str!("help.txt"),
                        exe = parser.bin_name().unwrap(),
                        i_max = i_max,
                        epsilon = epsilon,
                        temp0 = temp0,
                        alpha = alpha,
                    );
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Self {
            filename: filename.ok_or("Esperava o nome do arquivo")?,
            i_max,
            alpha,
            epsilon,
            temp0,
        })
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
