use log::error;

use crate::{get_home_specifier_and_system, util::nix};
use crate::commands::common;

pub async fn build_cmd(cli: &home_cli_def::Cli, args: &home_cli_def::commands::build::BuildArgs) {
    let (path, entry) = match common::get_nilla_nix_path(&cli.project).await {
        Ok(p) => p,
        Err(e) => return error!("{}", e),
    };

    let (specifier, system) = match get_home_specifier_and_system(
        entry,
        &args.specifier.clone().unwrap_or("".to_owned()),
    )
    .await
    {
        Ok((specifier, system)) => (specifier, system),
        Err(e) => return error!("{:?}", e),
    };

    let attribute = common::format_home_attribute(&specifier, &system);
    let builders = crate::util::args::extract_builders_from_args(&args.extra_nix_build_args);

    common::log_build_operation(&specifier, builders.as_ref());

    let out = nix::build(
        &path,
        &attribute,
        nix::BuildOpts {
            link: true,
            report: true,
            system: Some(system.as_str()),
            extra_args: &args.extra_nix_build_args,
        },
    )
    .await;

    if let Err(e) = out {
        return error!("{:?}", e);
    };
}
