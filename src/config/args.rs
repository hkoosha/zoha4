use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ZohaArgs {
    /// Override location of config file.
    #[arg(short, long)]
    pub cfg_file: Option<String>,

    /// Disable listening on dbus for keypress.
    #[arg(short, long, default_value_t = false)]
    #[cfg(feature = "hack")]
    pub keypress_grabber: bool,

    /// List keys accepted by keypress grabber.
    #[arg(long, default_value_t = false)]
    #[cfg(feature = "hack")]
    pub list_key_grabber_keys: bool,

    /// Signal Zoha to toggle visibility and exit.
    #[arg(short, long, default_value_t = false)]
    pub signal: bool,

    /// List monitors and exit.
    #[arg(long, default_value_t = false)]
    pub list_monitors: bool,

    /// Do not print hints.
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Sanitize configuration, print any errors and exit.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Print configuration and exit
    #[arg(short, long, default_value_t = false)]
    pub print_config: bool,

    /// Print color pallets
    #[arg(long, default_value_t = false)]
    pub print_pallets: bool,
}