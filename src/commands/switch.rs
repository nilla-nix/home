use log::{debug, error, info};
use tokio::process::Command;

use crate::{
    get_home_specifier_and_system,
    util::nix::{self},
};

pub async fn switch_cmd(
    cli: &nixos_cli_def::Cli,
    args: &nixos_cli_def::commands::switch::SwitchArgs,
) {
    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };

    let entry = project.clone().get_entry();
    let mut path = project.get_path();

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => return error!("File not found"),
        _ => {}
    }

    let (specifier, system) = match get_home_specifier_and_system(
        entry,
        &args.specifier.clone().unwrap_or("".to_owned()),
    )
    .await
    {
        Ok((specifier, system)) => (specifier, system),
        Err(e) => return error!("{:?}", e),
    };

    let attribute = format!("homes.\"{specifier}\".result.activationPackage");

    info!("Building home {specifier}");
    let out = nix::build(
        &path,
        &attribute,
        nix::BuildOpts {
            link: true,
            report: true,
            system: Some(system.as_str()),
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

            Command::new(format!("{out_path}/activate"))
                .output()
                .await
                .unwrap();
        }
        Err(e) => return error!("{:?}", e),
    };
}
