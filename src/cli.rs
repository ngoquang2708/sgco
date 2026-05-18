use clap::Parser;

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short = 'o')]
    pub r#override: Option<String>,
    #[arg(trailing_var_arg = true)]
    pub cmd: Vec<String>,
}
