use libc;

use clap::{App, Arg};

pub type SDWORD = libc::c_int;
#[doc = "Number of processors"]
pub const _NPROCESSORS_CONF: SDWORD = 83;

static OPTION_ALL: &str = "all";
static OPTION_IGNORE: &str = "ignore";

fn main() {
    let matches = App::new("nproc")
        .arg(
            Arg::with_name(OPTION_ALL)
                .short("")
                .long(OPTION_ALL)
                .help("print the number of installed processors"),
        )
        .arg(
            Arg::with_name(OPTION_IGNORE)
                .short("")
                .long(OPTION_IGNORE)
                .takes_value(true)
                .help("if possible, exclude N processing units"),
        )
        .get_matches_from(std::env::args_os().into_iter());

    let ignore = match matches.value_of(OPTION_IGNORE) {
        Some(numstr) => match numstr.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("\"{}\" is not a valid number: {}", numstr, e);
                std::process::exit(1)
            }
        },
        None => 0,
    };

    let mut cores = unsafe { libc::sysconf(_NPROCESSORS_CONF) };

    if cores <= ignore {
        cores = 1;
    } else {
        cores -= ignore;
    }
    println!("{}", cores);

    std::process::exit(0)
}
