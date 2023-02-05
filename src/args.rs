use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    /// locales [zh, jp, en]
    #[arg(short, long, default_value=&"en")]
    pub locale: String,
}
