use clap::{Arg, ArgAction};
use regex::Regex;
use std::fmt::Display;
use std::process;

#[derive(Debug)]
struct Event {
    name: String,
    weight: u32,
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

    let raw_events: Vec<String> = matches.get_many("events").unwrap().cloned().collect();

    let mut events: Vec<Event> = Vec::new();

    {
        let re = Regex::new(r"(?<name>\S+):(?<weight>\d+)$").unwrap();
        for raw_e in raw_events {
            let Some(caps) = re.captures(&raw_e) else {
                events.push(Event {
                    name: raw_e,
                    weight: 1,
                });

                continue;
            };

            let name = caps.name("name").unwrap().as_str();

            let weight = caps
                .name("weight")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            events.push(Event {
                name: name.to_string(),
                weight,
            });
        }
    }

    let total_weight = events.iter().fold(0, |acc, e| acc + e.weight);

    dbg!(&events);
    dbg!(total_weight);

    let increment = d / total_weight;

    if increment == 0 {
        eprintln!("ERROR: dice sides must be greater than number of events.");
        process::exit(1);
    }

    let mut i = 0;
    for event in events {
        i += 1;
        print!("{i:2} - ");

        i -= 1;
        for _ in 0..event.weight {
            i += increment;
        }

        println!("{i:<2}  {event}");
    }

    if i == d {
        return;
    }

    i += 1;
    print!("{i:2} - ");

    println!("{d:<2}  [Reroll]");
}
