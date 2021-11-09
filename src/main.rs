use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Tasker")
        .version("0.1.0")
        .author("William Drescher <wilkua2@gmail.com")
        .about("A simple to use task management system.")
        .subcommand(SubCommand::with_name("add")
            .arg(Arg::with_name("params")
                 .multiple(true)))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_matches)) => println!("sub matches: {:?}", sub_matches),
        _ => println!("something else:"),
    };
}
