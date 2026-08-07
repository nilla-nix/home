use log::{debug, error, info};
use std::path::PathBuf;

use crate::util::nix::FixedOutputStoreEntry;

/// Get the path to nilla.nix file from the project
pub async fn get_nilla_nix_path(project: &str) -> Result<(PathBuf, FixedOutputStoreEntry), String> {
    debug!("Resolving project {}", project);
    let project_resolved = crate::util::project::resolve(project)
        .await
        .map_err(|_| format!("Could not find project {}", project))?;

    let entry = project_resolved.clone().get_entry();
    let mut path = project_resolved.get_path();
    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => Err("File not found".to_string()),
        _ => Ok((path, entry)),
    }
}

/// Format the home attribute path for a specifier and system
pub fn format_home_attribute(specifier: &str, system: &str) -> String {
    format!("homes.\"{specifier}\".result.\"{system}\".activationPackage")
}

/// Format location string for logging
fn format_location(builders: Option<&String>) -> &str {
    builders.map(|b| b.as_str()).unwrap_or("locally")
}

/// Log build operation with location information
pub fn log_build_operation(specifier: &str, builders: Option<&String>) {
    let build_location = format_location(builders);
    if builders.is_some() {
        info!("Building home {specifier} with builders: {build_location}");
    } else {
        info!("Building home {specifier} locally");
    }
}
