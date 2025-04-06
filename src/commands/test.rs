use log::{debug, error, info};
use tokio::process::Command;

use crate::util::nix;

pub async fn test_cmd(cli: &cli_def::Cli, args: &cli_def::commands::test::TestArgs) {
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

    let system = match args.system.clone() {
        Some(s) => Some(s),
        _ => None,
    };

    let (user, tag) = if let Some(name) = args.name.clone() {
        if name.contains('@') {
            let sp = name.split('@').map(str::to_string).collect::<Vec<String>>();
            (sp[0].clone(), sp[1].clone())
        } else {
            let system = nix::get_system().await.unwrap();
            (name, system)
        }
    } else {
        let user = whoami::username();
        let system = nix::get_system().await.unwrap();
        (user, system)
    };

    let attribute = &format!("systems.nixos.\"{hostname}\".result.config.system.build.toplevel");

    match nix::exists_in_project(
        "nilla.nix",
        entry.clone(),
        &format!("systems.nixos.\"{hostname}\""),
    )
    .await
    {
        Ok(false) => {
            return error!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }

    info!("Building system {hostname}");
    let out = nix::build(
        &path,
        &attribute,
        nix::BuildOpts {
            link: true,
            report: true,
            system: system.as_deref(),
        },
    )
    .await;

    match out {
        Ok(o) => {
            info!("Enabling new configuration");
            let out_path = &o[0];

            let sudo = match which::which("sudo") {
                Ok(s) => s,
                Err(_e) => match which::which("doas") {
                    Ok(d) => d,
                    Err(_e) => return error!("Could not find sudo or doas"),
                },
            };

            Command::new(sudo)
                .arg(format!("{out_path}/bin/switch-to-configuration"))
                .arg("test")
                .output()
                .await
                .unwrap();
        }
        Err(e) => return error!("{:?}", e),
    };
}
