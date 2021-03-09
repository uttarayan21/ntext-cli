extern crate clap;
extern crate ntext;
use clap::{App, Arg};
use ntext::{to_text, Formatting};
fn main() {
    let matches = App::new("ntext-cli")
        .version("1.0")
        .author("Uttarayan Mondal")
        .about("Converts numbers to words")
        .arg(
            Arg::with_name("number")
                .help("The number to convert to words")
                .required(true), // .index(1),
        )
        .arg(
            Arg::with_name("capitalize")
                .short("c")
                .long("capitalize")
                .possible_values(&["true", "false"])
                .value_name("capitalize")
                .help("Capitalize all the words")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seperator")
                .short("s")
                .long("seperator")
                .value_name("seperator")
                .help("Seperator seperating all the words")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tens_seperator")
                .short("t")
                .long("tens")
                .value_name("tens_seperator")
                .help("Seperator seperating tens place words")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("digits_seperator")
                .short("d")
                .long("digits")
                .help("Seperator seperating all the digits")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("place_seperator")
                .short("p")
                .long("place")
                .help("Seperator seperating numbers from its place value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("noformat")
                .short("n")
                .long("noformat")
                .help("No formatting at all (can be overriden by flags)"),
        )
        .get_matches();

    let number = matches.value_of("number").unwrap().parse::<usize>();

    let capitalize = matches
        .value_of("capitalize")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();
    let noformat = matches.is_present("noformat");
    let seperator = matches.value_of("seperator");
    let tens_seperator = matches.value_of("tens_seperator");
    let digit_seperator = matches.value_of("digits_seperator");
    let place_seperator = matches.value_of("place_seperator");

    let mut fmt = Formatting::default();
    if noformat {
        fmt = Formatting::none();
    }
    fmt = Formatting {
        capitalize,
        digit_seperator: match (digit_seperator, seperator) {
            (None, None) => fmt.digit_seperator,
            (Some(dsep), _) => Some(dsep),
            (None, Some(sep)) => Some(sep),
        },
        tens_seperator: match (tens_seperator, seperator) {
            (None, None) => fmt.tens_seperator,
            (Some(tsep), _) => Some(tsep),
            (None, Some(sep)) => Some(sep),
        },
        place_seperator: match (place_seperator, seperator) {
            (None, None) => fmt.place_seperator,
            (Some(psep), _) => Some(psep),
            (None, Some(sep)) => Some(sep),
        },
    };
    match number {
        Ok(num) => println!("{}", to_text!(num, &fmt)),
        Err(e) => eprintln!("Error: {} \n<number> field should be a number", e),
    }
}
