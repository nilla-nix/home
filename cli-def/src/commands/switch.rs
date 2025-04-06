use clap::Args;

#[derive(Debug, Args)]
#[command(about = "Build, install, and activate into a home")]
pub struct SwitchArgs {
    #[arg(help = "Home name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
