use clap::Parser;

mod commands {
    // TODO: build
    mod get;
    mod list;

    pub use get::*;
    pub use list::*;
}

#[derive(Parser)]
#[clap(
    about = "A stapler making the Napkin totally bounded into a specific snapshot",
    long_about = r#"Stapler helps you get not only the most recent draft of Napkin
but also a Napkin book from a specific git commit hash.

By using stapler, you can easily manage Napkin books"#
)]
enum StaplerCli {
    Get(commands::Get),
    #[clap(visible_alias = "ls")]
    List(commands::List),
}

fn main() -> eyre::Result<()> {
    match StaplerCli::parse() {
        StaplerCli::Get(get) => get.run(),
        StaplerCli::List(list) => list.run(),
    }
}
