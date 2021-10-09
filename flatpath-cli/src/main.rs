use clap::{App, AppSettings, SubCommand};

fn main() {
    let projects = projects::projects();

    let matches = App::new("flatpath-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg_from_usage("-p, --project=[KEY], 'Generates project by key'")
        .subcommand(SubCommand::with_name("list").about("List projects"))
        .get_matches();

    if matches.subcommand_matches("list").is_some() {
        for p in projects.keys() {
            println!("{}", p);
        }
        return;
    }

    match matches.value_of("project") {
        Some(project_key) => {
            let p = projects
                .get(&project_key)
                .unwrap_or_else(|| panic!("no project with key: {}", project_key));
            projects::save(&**p).expect("error saving");
        }
        None => {
            println!("{}", matches.usage());
        }
    }
}
