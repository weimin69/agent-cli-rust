use clap::{Parser,Subcommand};

#[derive(Parser)] //Trait

pub struct Cli {
    #[command(subcommand)] // att
     pub command: Commands,

}
#[derive(Subcommand)] // 告知下面的类型的能力是子命令
pub enum Commands {
    Hello,
}
