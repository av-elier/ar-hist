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
        .subcommand(SubCommand::with_name("analyze"));
    app
}
