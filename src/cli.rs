use clap::{App, Arg, SubCommand};

pub fn ar_hist_app() -> App<'static, 'static> {
    let app = App::new("ar-hist")
        .version("0.0.1")
        .author("av_elier")
        .subcommand(
            SubCommand::with_name("download")
                .arg(
                    Arg::with_name("typed")
                        .long("typed")
                        .help("Parse initiatives on download. WARNING: this can ignore some data"),
                )
                .arg(
                    Arg::with_name("save")
                        .long("save")
                        .takes_value(true)
                        .possible_values(&["postgres", "redis", "stdout"])
                        .help("Enable saving to db"),
                )
                .arg(
                    Arg::with_name("pg-table-name")
                        .long("pg-table-name")
                        .required_if("save", "postgres")
                        .takes_value(true)
                        .help("A postgres table name to save data to"),
                )
                .arg(
                    Arg::with_name("ar-status")
                        .long("ar-status")
                        .takes_value(true)
                        .possible_values(&[
                            "active",
                            "attention",
                            "completed",
                            "considered",
                            "implemented",
                        ])
                        .help("ar initiative status filter"),
                ),
        )
        .subcommand(SubCommand::with_name("analyze")) // TODO: last day top gainers by votes, views, shares. Option to email (or telegram) results
        .subcommand(
            SubCommand::with_name("migrate")
                .arg(
                    Arg::with_name("action")
                        .long("action")
                        .required(true)
                        .possible_values(&["filter-unchanged"])
                        .help("filter-unchanged: removes exactly same initiatives from latter snapshot")
                )
                .arg(
                    Arg::with_name("pg-table-orig")
                        .long("pg-table-orig")
                        .required(true)
                        .help("a table to get initiatives from"),
                )
                .arg(
                    Arg::with_name("pg-table-dest")
                        .long("pg-table-dest")
                        .help("destination to save to. If not specified, just stdout"),
                ),
        );
    app
}
