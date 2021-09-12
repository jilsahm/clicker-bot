use clap::Clap;
use clicker_bot::Configuration;

fn main() {
    env_logger::builder().parse_filters("info").init();
    let config = Configuration::parse();
    clicker_bot::run(config);
}
