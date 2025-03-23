use clap::{Arg, ArgAction};

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
    let num_events = events.len() as u32;

    let increment = d / num_events;

    for (i, event) in events.iter().enumerate().map(|(a, b)| (a as u32 + 1, b)) {
        let higher = i * increment;
        let lower = higher - increment + 1;
        println!("{lower:2}-{higher:<2}  {event}");
    }
}
