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

/// Focus on the next window. If the current window is already at the edge, focus on the next workspace.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "r")]
pub struct CommandNext {
    #[argh(switch, description = "swap window")]
    pub swap: bool,
}

/// Focus on the previous window. If the current window is already at the edge, focus on the previous workspace.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "l")]
pub struct CommandPrev {
    #[argh(switch, description = "swap window")]
    pub swap: bool,
}

