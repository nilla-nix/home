/// nix build flag for specifying remote builders
pub const BUILDERS_FLAG: &str = "--builders";

/// Extract a value from command-line arguments for a given flag.
///
/// Supports both `--flag value` and `--flag=value` formats.
pub fn extract_value_from_args(args: &[String], flag: &str) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag {
            // Check if next argument is the value
            if let Some(next) = args.get(i + 1) {
                if !next.starts_with('-') {
                    return Some(next.clone());
                }
            }
        } else if arg.starts_with(&format!("{}=", flag)) {
            // Handle --flag=value format
            return arg.split('=').nth(1).map(|s| s.to_string());
        }
    }
    None
}

/// Extract builders information from arguments.
pub fn extract_builders_from_args(args: &[String]) -> Option<String> {
    extract_value_from_args(args, BUILDERS_FLAG)
}
