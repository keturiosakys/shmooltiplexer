use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_parser, num_args =1.., value_name = "COMMAND")]
    script: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    for command in cli.script {
        println!("Running command: {}", command);
    }
}
