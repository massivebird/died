use clap::Arg;

fn main() {
    let matches = clap::command!()
        .args([Arg::new("d")
            .short('d')
            .default_value("20")
            .value_parser(clap::value_parser!(u32).range(1..))])
        .get_matches();

    dbg!(matches);
}
