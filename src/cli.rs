use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn cli<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("get")
                .about("gets string value from table given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("adds a key/value string pair to the table")
                .arg(
                    Arg::with_name("KEY")
                        .help("the key string for the insert")
                        .required(true),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("the string value for the insert")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("removes a string value from table provided a string key")
                .arg(
                    Arg::with_name("KEY")
                        .help("the string key for the string value to be removed")
                        .required(true),
                ),
        )
        .get_matches()
}
