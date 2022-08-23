use std::ffi::OsString;
use std::path::PathBuf;
use bio_anno_rs::BEDGraphData;

use clap::{Arg, ArgAction, Command};

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
                        .short('w')
                        .long("winsize")
                        .help("The size of the window to convolve over the chosen column")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("infile")
                        .short("i")
                        .long("infile")
                        .help("The input file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("circular")
                        .short("c")
                        .long("circular")
                        .required(false)
                        .takes_value(false)
                        .help("Include if the chromosomes are all circular"),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("roll_mean", rm_matches)) => {

            let winsize: usize = *rm_matches
                .get_one::<usize>("winsize")
                .expect("Window size");

            let infile = rm_matches
                .get_one::<PathBuf>("infile")
                .expect("input file");

            let circ = rm_matches.is_present("circular");

            let bgd = BEDGraphData::from_file( infile )?;
            let result = bgd.roll_mean( winsize, circ )?;
            result.print();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }


}
