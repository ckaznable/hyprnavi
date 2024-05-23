use argh::FromArgs;

/// simple horizontal navigation in hyprland
#[derive(FromArgs)]
pub struct Flags {
    #[argh(subcommand)]
    pub cmd: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Command {
    Next(CommandNext),
    Prev(CommandPrev),
}

/// focus into window in right side
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "r")]
pub struct CommandNext {
    #[argh(switch, description = "swap window")]
    pub swap: bool,
}

/// focus into window in left side
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "l")]
pub struct CommandPrev {
    #[argh(switch, description = "swap window")]
    pub swap: bool,
}

