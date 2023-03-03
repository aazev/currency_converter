#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ServiceMode {
    Socket,
    Address,
}
