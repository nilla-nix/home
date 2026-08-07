use log::{error, info};
use tokio::process::Command;

use crate::{get_home_specifier_and_system, util::nix};
use crate::commands::common;

pub async fn switch_cmd(
    cli: &home_cli_def::Cli,
    args: &home_cli_def::commands::switch::SwitchArgs,
) {
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

    match out {
        Ok(o) => {
            if o.is_empty() {
                return error!("Failed to build configuration, skipping switching to it");
            }

            info!("Switching to new configuration");
            let out_path = &o[0];

            let activate_output = Command::new(format!("{out_path}/activate"))
                .output()
                .await
                .unwrap();

            if !activate_output.status.success() {
                error!(
                    "Failed to switch to new configuration:\n{}",
                    String::from_utf8_lossy(&activate_output.stdout)
                ) // home-manager writes its "error" text to stdout
            }
        }
        Err(e) => return error!("{:?}", e),
    };
}
