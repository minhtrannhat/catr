use clap::builder::{Arg, Command};
use clap::{crate_authors, crate_description, crate_name, crate_version, ArgAction, ArgMatches};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
                .long("number")
                .help("Number lines")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count_digits(mut number: usize) -> usize {
    if number == 0 {
        return 1; // Special case for the number 0
    }

    let mut count = 0;

    while number != 0 {
        number /= 10;
        count += 1;
    }

    count
}

fn cat(
    buf_reader: Box<dyn BufRead>,
    line_num: &mut usize,
    number_lines: bool,
    number_nonblank_lines: bool,
) {
    for line in buf_reader.lines() {
        match line {
            Ok(text) => match (number_lines, number_nonblank_lines) {
                (true, false) => {
                    println!(
                        "{}{}\t{}",
                        " ".to_string().repeat(6 - count_digits(*line_num)),
                        line_num,
                        text
                    );
                    *line_num += 1;
                }
                (false, true) => {
                    if text.trim().is_empty() {
                        println!()
                    } else {
                        println!(
                            "{}{}\t{}",
                            " ".to_string().repeat(6 - count_digits(*line_num)),
                            line_num,
                            text
                        );
                        *line_num += 1;
                    }
                }
                _ => {
                    println!("{}", text);
                    *line_num += 1;
                }
            },

            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num: usize = 1;

    for filepath in config.files {
        match open(&filepath) {
            Err(err) => eprintln!("Failed to open {}: {}", filepath, err),
            Ok(buf_reader) => cat(
                buf_reader,
                &mut line_num,
                config.number_lines,
                config.number_nonblank_lines,
            ),
        }
    }

    Ok(())
}
