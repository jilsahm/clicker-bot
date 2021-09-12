use clap::Clap;

#[derive(Clap)]
#[clap(
    about = "CLI tool for running a simple clicker bot",
    author = clap::crate_authors!(),
    name = clap::crate_name!(),
    version = clap::crate_version!(),
)]
pub struct Configuration {

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {

    #[clap(about = "running a click storm at the cursor position")]
    Click,

    #[clap(about = "record mouse positions by clicking")]
    Record,

    #[clap(about = "replays a given script")]
    Replay,
}
