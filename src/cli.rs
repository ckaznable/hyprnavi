use argh::FromArgs;

/// simple horizontal navigation in hyprland
#[derive(FromArgs)]
pub struct Flags {
    #[argh(subcommand)]
    pub cmd: Command,

    #[argh(switch, description = "swap window")]
    pub swap: bool,
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
pub struct CommandNext {}

/// focus into window in left side
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "l")]
pub struct CommandPrev {}

