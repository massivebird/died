use clap::{Arg, ArgAction};
use std::process;

fn main() {
    let matches = clap::command!()
        .args([
            Arg::new("d")
                .short('d')
                .default_value("20")
                .value_parser(clap::value_parser!(u32).range(1..)),
            Arg::new("events")
                .action(ArgAction::Append)
                .required(true)
                .trailing_var_arg(true),
        ])
        .get_matches();

    let d: u32 = matches.get_one("d").copied().unwrap();

    let events: Vec<String> = matches.get_many("events").unwrap().cloned().collect();
    let num_events = u32::try_from(events.len()).unwrap();

    let increment = d / num_events;

    if increment == 0 {
        eprintln!("ERROR: dice sides must be greater than number of events.");
        process::exit(1);
    }

    let mut i = 0;
    for event in events {
        i += 1;
        print!("{i:2} - ");

        i += increment - 1;
        println!("{i:<2}  {event}");
    }

    if i == d {
        return;
    }

    i += 1;
    print!("{i:2} - ");

    println!("{d:<2}  [Reroll]");
}
