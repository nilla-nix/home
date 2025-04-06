use log::{debug, error, info};

use crate::util::nix;

pub async fn build_cmd(cli: &cli_def::Cli, args: &cli_def::commands::build::BuildArgs) {
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

    let attribute = &format!("homes.\"{user}@{tag}\".result.config.activationPackage");

    match nix::exists_in_project(
        "nilla.nix",
        entry.clone(),
        &format!("homes.\"{user}@{tag}\""),
    )
    .await
    {
        Ok(false) => {
            return error!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }

    info!("Building home {user}@{tag}");
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

    if let Err(e) = out {
        return error!("{:?}", e);
    };
}
