use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Build, install, and switch into a home",
    long_about = "Build, install, and switch into a home. Additional nix build options can be passed after --, e.g.: nilla home switch -- --builders \"ssh://remote x86_64-linux\""
)]
pub struct SwitchArgs {
    #[arg(help = "Home specifier, in the format {username}[@hostname][:system]")]
    pub specifier: Option<String>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, help = "Additional arguments to pass to nix build")]
    pub extra_nix_build_args: Vec<String>,
}
