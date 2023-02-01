use clap::{builder::PossibleValue, value_parser, Arg, Command, ValueEnum};
use std::{env, io};
use std::{path::PathBuf, str::FromStr};
use thiserror::Error;

// cli struct
pub struct Opt {
    // optional as can read from stdin
    pub input: Box<dyn io::Read>,
    // can be multiple keys
    pub keys: Vec<String>,
    // but only one field
    pub field: String,
    // delimiter
    pub delimiter: char,
    // the summary function
    pub function: Summary,
}

const INPUT: &str = "input";
const KEYS: &str = "keys";
const FIELD: &str = "field";
const DELIMITER: &str = "delimiter";
const SUMMARY: &str = "summary";

// probably implement, max, min, and maybe a few others
#[derive(Debug, Clone, Copy)]
pub enum Summary {
    Mean,
    N,
    StdDev,
    Var,
}

impl Summary {
    pub fn inner_fn(&self, list: &[f64]) -> Option<f64> {
        match self {
            Summary::Mean => Self::mean(list),
            Summary::N => Some(list.len() as f64),
            Summary::StdDev => Self::std_deviation(list),
            Summary::Var => Self::std_deviation(list).map(|e| e.powf(2.0)),
        }
    }

    fn std_deviation(list: &[f64]) -> Option<f64> {
        match (Self::mean(list), list.len()) {
            (Some(list_mean), count) if count > 0 => {
                let variance = list
                    .iter()
                    .map(|value| {
                        let diff = list_mean - *value;

                        diff * diff
                    })
                    .sum::<f64>()
                    / count as f64;

                Some(variance.sqrt())
            }
            _ => None,
        }
    }

    fn mean(list: &[f64]) -> Option<f64> {
        let sum = list.iter().sum::<f64>();
        let count = list.len();

        match count {
            positive if positive > 0 => Some(sum / count as f64),
            _ => None,
        }
    }
}

impl ValueEnum for Summary {
    fn value_variants<'a>() -> &'a [Self] {
        &[Summary::Mean, Summary::N, Summary::StdDev, Summary::Var]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Summary::Mean => PossibleValue::new("mean").help("Calculate mean on groups"),
            Summary::N => PossibleValue::new("N").help("Calculate number in each group"),
            Summary::StdDev => {
                PossibleValue::new("sd").help("Calculate standard deviation on each group")
            }
            Summary::Var => PossibleValue::new("var").help("Calculate variance on each group"),
        })
    }
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Summary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {}", s))
    }
}

// the usage
const USAGE: &str = "\
    gb [OPTIONS] <FILE/STDIN>
    gb -f field1 -k key1,key2 -s N -i input.tsv
    cat input.csv | gb -f field1 -k key1 -s mean -d,";

const TEMPLATE: &str = "\
{bin} {version}
Max Brown <euphrasiamax@gmail.com>
https://github.com/euphrasiologist/gb

{usage-heading}
    {usage}
{all-args}\
";

pub(crate) fn parse() -> Opt {
    let args: Vec<_> = env::args_os().collect();
    let args = app().get_matches_from(args);

    let path = args.get_one::<PathBuf>(INPUT).expect("defaulted by clap");

    let reader: Box<dyn io::Read> = match path.as_os_str().to_str().unwrap() {
        "-" => Box::new(io::stdin()),
        p => Box::new(std::fs::File::open(p).unwrap()),
    };

    let keys = args
        .get_one::<Vec<String>>(KEYS)
        .expect("defaulted by clap")
        .to_owned();

    let field = args
        .get_one::<String>(FIELD)
        .expect("defaulted by clap")
        .clone();

    let delimiter = *args.get_one::<char>(DELIMITER).expect("defaulted by clap");

    let function = *args.get_one::<Summary>(SUMMARY).expect("defaulted by clap");

    Opt {
        input: reader,
        keys,
        field,
        delimiter,
        function,
    }
}

fn app() -> Command {
    let mut app = Command::new("gb")
        .override_usage(USAGE)
        .help_template(TEMPLATE)
        .arg(arg_input())
        .arg(arg_keys())
        .arg(arg_field())
        .arg(arg_delimiter())
        .arg(arg_summary());
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        app = app.version(version);
    }
    app
}

fn arg_input() -> Arg {
    Arg::new(INPUT)
        .short('i')
        .long(INPUT)
        .default_value("-")
        .num_args(1)
        .value_name("INPUT")
        .value_parser(value_parser!(PathBuf))
        .help("Path to the input file. Defaults to STDIN.")
}

fn arg_keys() -> Arg {
    Arg::new(KEYS)
        .short('k')
        .long(KEYS)
        .required(true)
        .num_args(1..)
        .value_name("KEYS")
        .value_parser(parse_keys)
        .help("The grouping keys as column header strings")
}

fn arg_field() -> Arg {
    Arg::new(FIELD)
        .short('f')
        .long(FIELD)
        .required(true)
        .num_args(1)
        .value_name("FIELD")
        .value_parser(value_parser!(String))
        .help("The field on which to calculate grouping stats")
}

fn arg_delimiter() -> Arg {
    Arg::new(DELIMITER)
        .short('d')
        .long(DELIMITER)
        .default_value("\t")
        .num_args(1)
        .value_name("DELIMITER")
        .value_parser(value_parser!(char))
        .help("The delimiter, default is tab")
}

fn arg_summary() -> Arg {
    Arg::new(SUMMARY)
        .short('s')
        .long(SUMMARY)
        .default_value("N")
        .num_args(1)
        .value_name("SUMMARY")
        .value_parser(value_parser!(Summary))
        .help("Summary stat to comupte on groups")
}

#[derive(Error, Debug)]
enum Error {}

fn parse_keys(string: &str) -> Result<Vec<String>, Error> {
    let mut res = Vec::new();

    for predicate in string.split(',') {
        let predicate = predicate.trim();

        res.push(predicate.to_string());
    }

    Ok(res)
}
