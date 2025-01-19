use std::{ffi::OsString, fs::File, io::BufReader};

#[derive(Debug)]
pub struct PParams {
    pub i_max: usize,
    pub penalty: usize,
}

pub struct Args {
    pub filename: OsString,
    pub params: PParams,
}

impl Args {
    pub fn open_file(&self) -> BufReader<File> {
        let file = File::open(&self.filename).expect("Falha ao abrir arquivo de entrada");
        BufReader::new(file)
    }
    pub fn from_argv(mut defaults: PParams) -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_env();
        let mut filename = None;
        let i_max = &mut defaults.i_max;
        let penalty = &mut defaults.penalty;

        while let Some(arg) = parser.next()? {
            match arg {
                Value(fname) if filename.is_none() => {
                    filename = Some(fname);
                }
                Value(inv) => {
                    return Err(format!("Não esperava o argumento {:?}", inv))?;
                }
                Short('i') | Long("i-max") => *i_max = parser.value()?.parse()?,
                Short('p') | Long("penalty") => *penalty = parser.value()?.parse()?,
                Short('h') | Long("help") => {
                    eprintln!(
                        include_str!("ksp_sa_help.txt"),
                        exe = parser.bin_name().unwrap(),
                        i_max = i_max,
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
