use std::{ffi::OsString, fs::File, io::BufReader};

#[derive(Debug)]
pub struct Defaults {
    pub i_max: usize,
}

pub struct Args {
    pub filename: OsString,
    pub i_max: usize,
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

        while let Some(arg) = parser.next()? {
            match arg {
                Value(fname) if filename.is_none() => {
                    filename = Some(fname);
                }
                Value(inv) => {
                    return Err(format!("Não esperava o argumento {:?}", inv))?;
                }
                Short('h') | Long("help") => {
                    eprintln!(
                        include_str!("ksp_ts_help.txt"),
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
            i_max,
        })
    }
}
