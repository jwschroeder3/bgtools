use std::path::PathBuf;
use bio_anno_rs::BEDGraphData;
use bio_anno_rs::RollFn;
use std::error::Error;
use std::process::exit;

use clap::{Arg, Command, value_parser};

fn cli() -> Command<'static> {
    Command::new("bgtools")
        .about("A tool for doing simple math on the data in a bedgraph file.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        //.allow_external_subcommands(true)
        .subcommand(
            Command::new("roll")
                .about("Applies a windowed rolling mean to the data")
                .arg(
                    Arg::new("function")
                        //.value_parser(value_parser!(String))
                        .short('f')
                        .long("function")
                        .help("The function to be rolled over the scores in `--infile`. Currently supported functions are `mean` or `median`.")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("winsize")
                        .value_parser(value_parser!(usize))
                        .short('w')
                        .long("winsize")
                        .help("The size of the window (in base pairs) to roll over the score column")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("infile")
                        .value_parser(value_parser!(PathBuf))
                        .short('i')
                        .long("infile")
                        .help("The input file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("circular")
                        .short('c')
                        .long("circular")
                        .required(false)
                        .takes_value(false)
                        .help("Include if the chromosomes are all circular"),
                ),
        )
        .subcommand(
            Command::new("robust_z")
                .about("Calculates robust z-score for each positions")
                .arg(
                    Arg::new("infile")
                        .value_parser(value_parser!(PathBuf))
                        .short('i')
                        .long("infile")
                        .help("The input file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("cpm")
                .about("Returns a bedgraph file with cpm calculated")
                .arg(
                    Arg::new("infile")
                        .value_parser(value_parser!(PathBuf))
                        .short('i')
                        .long("infile")
                        .help("The input file")
                        .takes_value(true)
                        .required(true),
                ),

        )
}

fn main() -> Result<(),Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("roll", rm_matches)) => {
            let allowed_functions = vec!["mean","median"];

            let fun: &str = rm_matches
                .get_one::<String>("function")
                .expect("--function must be included, and can be either `mean` or `median`");
            if !(allowed_functions.contains(&fun)) {
                eprintln!("Your value to `--function` is not allowed. You passed {}, but it must be either \"mean\" or \"median\"", &fun);
                exit(1);
            }

            let winsize: usize = *rm_matches
                .get_one::<usize>("winsize")
                .expect("--winsize argument is required");

            let infile = rm_matches
                .get_one::<PathBuf>("infile")
                .expect("--infile argument is required");

            let circ = rm_matches.is_present("circular");

            let bgd =
                if infile == &PathBuf::from("-") {
                    BEDGraphData::from_stdin()?
                } else {
                    BEDGraphData::from_file( infile )?
                };
            let resolution = bgd.get_resolution();
            let mut winsize_line = winsize / resolution;
            if winsize_line % 2 == 0 {
                winsize_line += 1;
            }
            let result = 
                if fun == "median" {
                    bgd.roll_fn( winsize_line, circ, RollFn::Median )?
                } else if fun == "mean" {
                    bgd.roll_fn( winsize_line, circ, RollFn::Mean )?
                } else {
                    eprintln!("You did not pass an allowable function to `--function`");
                    exit(1);
                };
            result.print()?;
        },
        Some(("robust_z", rm_matches)) => {
            let infile = rm_matches
                .get_one::<PathBuf>("infile")
                .expect("--infile argument is required");

            let bgd = BEDGraphData::from_file( infile )?;
            let result = bgd.robust_z()?;
            result.print()?;
        },
        Some(("cpm", cpm_matches)) => {
             let infile = cpm_matches
                .get_one::<PathBuf>("infile")
                .expect("--infile argument is required");
            let mut bgd = BEDGraphData::from_file( infile )?;
            bgd.to_cpm()?;
            bgd.print()?;
        },
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())

}
