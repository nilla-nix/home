use clap::Args;
#[derive(Debug, Args)]
#[command(
    about = "Build a home",
    long_about = "Build a home. Additional nix build options can be passed after --, e.g.: nilla home build -- --builders \"ssh://remote x86_64-linux\""
)]
pub struct BuildArgs {
    #[arg(help = "Home specifier, in the format {username}[@hostname][:system]")]
    pub specifier: Option<String>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, help = "Additional arguments to pass to nix build")]
    pub extra_nix_build_args: Vec<String>,
}
