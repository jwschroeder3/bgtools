use std::path::PathBuf;
use bio_anno_rs::BEDGraphData;
use std::error::Error;

use clap::{Arg, Command, value_parser};

fn cli() -> Command<'static> {
    Command::new("bgtools")
        .about("A tool for doing simple math on the data in a bedgraph file.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        //.allow_external_subcommands(true)
        .subcommand(
            Command::new("roll_mean")
                .about("Applies a windowed rolling mean to the data")
                .arg(
                    Arg::new("winsize")
                        .value_parser(value_parser!(usize))
                        .short('w')
                        .long("winsize")
                        .help("The size of the window (in base pairs) to roll over the score column")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("infile")
                        .value_parser(value_parser!(PathBuf))
                        .short('i')
                        .long("infile")
                        .help("The input file")
                        .takes_value(true),
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
                        .takes_value(true),
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
                        .takes_value(true),
                ),

        )
}

fn main() -> Result<(),Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("roll_mean", rm_matches)) => {

            let winsize: usize = *rm_matches
                .get_one::<usize>("winsize")
                .expect("--winsize argument is required");

            let infile = rm_matches
                .get_one::<PathBuf>("infile")
                .expect("--infile argument is required");

            let circ = rm_matches.is_present("circular");

            let bgd = BEDGraphData::from_file( infile )?;
            let resolution = bgd.get_resolution();
            let mut winsize_line = winsize / resolution;
            if winsize_line % 2 == 0 {
                winsize_line += 1;
            }
            let result = bgd.roll_mean( winsize_line, circ )?;
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
