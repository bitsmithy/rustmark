mod rustmark;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .arg(
            clap::Arg::with_name("FILE")
                .help("Path of the Markdown file to parse")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filepath = matches.value_of("FILE").unwrap();
    rustmark::parse_file(filepath);
}
