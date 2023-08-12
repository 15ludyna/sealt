use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Sealt",
    about = "Website status checker "
)]
pub struct Cli {
    #[structopt(
        short = "j", 
        long = "json",
        help = "Format output to Json"
    )]
    pub json: bool,
    pub domain: String
}
