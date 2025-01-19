use std::{ffi::OsString, fs::File, io::BufReader};

#[derive(Debug)]
pub struct Params {
    pub i_max: usize,
    pub epsilon: f64,
    pub alpha: f64,
    pub temp0: f64,
    pub exponential_cooling: bool,
}

#[derive(Debug)]
pub struct Args {
    pub filename: OsString,
    pub params: Params,
}

impl Args {
    pub fn open_file(&self) -> BufReader<File> {
        let file = File::open(&self.filename).expect("Falha ao abrir arquivo de entrada");
        BufReader::new(file)
    }
    pub fn from_argv(mut defaults: Params) -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_env();
        let mut filename = None;
        let i_max = &mut defaults.i_max;
        let epsilon = &mut defaults.epsilon;
        let alpha = &mut defaults.alpha;
        let temp0 = &mut defaults.temp0;
        let exp_cooling = &mut defaults.exponential_cooling;

        while let Some(arg) = parser.next()? {
            match arg {
                Value(fname) if filename.is_none() => {
                    filename = Some(fname);
                }
                Value(inv) => {
                    return Err(format!("Não esperava o argumento {:?}", inv))?;
                }
                Short('i') | Long("i-max") => *i_max = parser.value()?.parse()?,
                Short('e') | Long("epsilon") => *epsilon = parser.value()?.parse()?,
                Short('a') | Long("alpha") => *alpha = parser.value()?.parse()?,
                Short('E') | Long("exponential-cooling") => {
                    *exp_cooling = parser.value()?.parse()?
                }
                Short('t') | Long("temp0") => *temp0 = parser.value()?.parse()?,
                Short('h') | Long("help") => {
                    eprintln!(
                        include_str!("tsp_sa_help.txt"),
                        exe = parser.bin_name().unwrap(),
                        exponential_cooling = exp_cooling,
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
            params: defaults,
        })
    }
}
