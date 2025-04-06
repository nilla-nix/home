use clap::Args;
#[derive(Debug, Args)]
#[command(about = "Build a Home Manager Home")]
pub struct BuildArgs {
    #[arg(help = "Home name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
