use clap::{crate_authors, crate_description, crate_name, crate_version, ArgMatches};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true)
                .global(true)
                .help("Increases the level of verbosity"),
        )
        .subcommand(SubCommand::with_name("greet"))
}

pub fn configure(matches: &ArgMatches) {
    loggerv::Logger::new()
        .verbosity(matches.occurrences_of("verbosity"))
        .level(true)
        .no_module_path()
        .module_path(false)
        .base_level(log::Level::Info)
        .init()
        .unwrap();
}
