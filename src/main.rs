use clap::{Arg, ArgAction};
use event::Event;
use std::{io::Write, process};
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Rows},
};

mod event;

#[derive(Tabled)]
struct TableRow {
    #[tabled(rename = "Range")]
    range: String,
    #[tabled(rename = "Event")]
    event_name: String,
    #[tabled(rename = "Probability")]
    probability: String,
}

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

    // Number of digits in `d`. Used for styling range strings.
    let d_width = d.to_string().len();

    // Convert input strings into Event instances.
    let events: Vec<Event> = matches
        .get_many("events")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>()
        .iter()
        .map(|s| Event::try_from(s.as_str()).unwrap())
        .collect::<Vec<Event>>();

    let total_weight: u32 = events.iter().fold(0, |acc, e| acc + e.weight);

    // Represents the range size corresponding to a weight of 1.
    let increment: u32 = d / total_weight;

    if increment == 0 {
        eprintln!(
            "ERROR: dice sides must be greater than or equal to sum of weights. (sides: {d}, sum of weights: {total_weight})"
        );
        process::exit(1);
    }

    let mut table_rows: Vec<TableRow> = Vec::new();

    // Tracks current position along range `0..=d` as we iterate through
    // all the events.
    let mut i: u32 = 0;

    for event in events {
        let mut range_buf: Vec<u8> = Vec::new();

        // This event's lower bound.
        i += 1;
        write!(range_buf, "{i:d_width$} - ").unwrap();

        // Compute this event's upper bound.
        i -= 1;
        for _ in 0..event.weight {
            i += increment;
        }

        write!(range_buf, "{i}").unwrap();

        table_rows.push(TableRow {
            range: String::from_utf8(range_buf).unwrap(),
            event_name: event.name,
            probability: format!("{:.02}%", f64::from(event.weight) / f64::from(total_weight)),
        });
    }

    if i != d {
        // Designate remaining values as rerolls.
        i += 1;

        table_rows.push(TableRow {
            range: format!("{i:d_width$} - {d}"),
            event_name: String::from("[Reroll]"),
            probability: String::new(),
        });
    }

    let mut table = Table::new(table_rows);

    table
        .with(Style::psql())
        .modify(Rows::first(), Alignment::center());

    println!("{table}");
}
