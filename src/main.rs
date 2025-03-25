use clap::{Arg, ArgAction};
use regex::Regex;
use std::{fmt::Display, process};

#[derive(Debug)]
struct Event {
    name: String,
    weight: u32,
}

impl TryFrom<&str> for Event {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?<name>\S+):(?<weight>\d+)$").unwrap();

        let Some(caps) = re.captures(value) else {
            return Ok(Self {
                name: value.to_string(),
                weight: 1,
            });
        };

        let name = caps.name("name").unwrap().as_str();

        let weight = caps
            .name("weight")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        Ok(Self {
            name: name.to_string(),
            weight,
        })
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

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

    let events: Vec<Event> = matches
        .get_many("events")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>()
        .iter()
        .map(|s| Event::try_from(s.as_str()).unwrap())
        .collect::<Vec<Event>>();

    let total_weight = events.iter().fold(0, |acc, e| acc + e.weight);

    let increment = d / total_weight;

    if increment == 0 {
        eprintln!("ERROR: dice sides must be greater than number of events.");
        process::exit(1);
    }

    println!("Dice      d{d}");
    println!("Events    {}", events.len());
    println!("--------------");

    let mut i = 0;
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
