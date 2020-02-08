extern crate clap;
extern crate env_logger;
extern crate log;
extern crate serde;
extern crate serde_json;

use env_logger::Builder;
use clap::{App, Arg, SubCommand};
use log::{LevelFilter};
use std::io::Write;

fn main() {
    let matches = App::new("Playlist Tools")
        .version("0.1")
        .about("A tool to work with audio playlists")
        .arg(Arg::with_name("v").short("v").multiple(true)
            .help("Sets level of verbosity"))
        .subcommand(SubCommand::with_name("nolist")
            .about("Finds songs that are not in any playlist ")
            .arg(Arg::with_name("playlist location").short("l").long("location")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("playlist type").short("t").long("type")
                .required(true)
                .takes_value(true)))
        .get_matches();

    let verbosity = matches.occurrences_of("v");
    let mut log_builder = Builder::from_default_env();
    log_builder.format(|buf, record| {
        writeln!(buf,
            "[{}] - {}",
            record.level(),
            record.args()
        )
    });
    
    let _ = match verbosity {
        0 => log_builder.filter_level(LevelFilter::Info),
        1 => log_builder.filter_level(LevelFilter::Debug),
        _ => log_builder.filter_level(LevelFilter::Trace)
    };
    log_builder.init();

    if let Some(nolist_matches) = matches.subcommand_matches("nolist") {
        println!("Nolist operation: loc {}\ntype {}", 
            nolist_matches.value_of("playlist location").unwrap(),
            nolist_matches.value_of("playlist type").unwrap());
    }
}
