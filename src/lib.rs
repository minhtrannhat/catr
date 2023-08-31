use clap::builder::{Arg, Command};
use clap::{crate_authors, crate_description, crate_name, crate_version, ArgAction, ArgMatches};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches: ArgMatches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .help_template(
            "\
{before-help}{name} {version}
    {author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args} {after-help}
",
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
                .num_args(1..)
                .default_values(["-"]),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .help("Number lines")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .help("Number nonblank lines")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .expect("Must contains valid filepaths")
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
