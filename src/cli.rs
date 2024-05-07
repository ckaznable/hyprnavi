use argh::FromArgs;

///
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
pub struct CommandNext {}

/// focus into window in left side
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "l")]
pub struct CommandPrev {}

