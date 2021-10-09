use clap::{App, Arg, SubCommand};

fn main() {
    let projects = projects::projects();

    let matches = App::new("flatpath-cli")
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("NAME")
                .help("PROject")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").about("list projects"))
        .get_matches();

    if matches.subcommand_matches("list").is_some() {
        for p in projects.keys() {
            println!("{}", p);
        }
        return;
    }

    match matches.value_of("project") {
        Some(project) => {
            let p = projects.get(&project).unwrap();
            projects::save(&**p).expect("error saving");
        }
        None => {
            println!("{}", matches.usage());
        }
    }
}
