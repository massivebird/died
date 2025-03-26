use clap::{Arg, ArgAction};
use event::Event;
use std::process;

mod event;

fn main() {
    let matches = clap::command!()
        .about("A dice roll event distributor.")
        .args([
            Arg::new("d")
                .value_name("NUM")
                .help("Number of sides on the dice.")
                .short('d')
                .default_value("20")
                .value_parser(clap::value_parser!(u32).range(1..)),
            Arg::new("events")
                .help("Strings representing events.")
                .action(ArgAction::Append)
                .required(true)
                .trailing_var_arg(true),
        ])
        .get_matches();

    let d: u32 = matches.get_one("d").copied().unwrap();

    let events: Vec<Event> = matches
        .get_many("events")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>()
        .iter()
        .map(|s| Event::try_from(s.as_str()).unwrap())
        .collect::<Vec<Event>>();

    let total_weight: u32 = events.iter().fold(0, |acc, e| acc + e.weight);

    let increment: u32 = d / total_weight;

    if increment == 0 {
        eprintln!("ERROR: dice sides must be greater than or equal to sum of weights. (sides: {d}, sum of weights: {total_weight})");
        process::exit(1);
    }

    println!("Dice      d{d}");
    println!("Events    {}", events.len());
    println!("--------------");

    let mut i: u32 = 0;
    for event in events {
        // This event's lower bound.
        i += 1;
        print!("{i:2} - ");

        // Calculate this event's upper bound.
        i -= 1;
        for _ in 0..event.weight {
            i += increment;
        }

        println!("{i:<2}  {event}");
    }

    if i != d {
        // Designate remaining values as rerolls.
        i += 1;
        print!("{i:2} - ");
        println!("{d:<2}  [Reroll]");
    }
}
