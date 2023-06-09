//A command-line tool to predict weather alerts
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Alison",
    about = "A weather warning predictor"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "0.1.0", author = "Alison")]
    Predict { test: f32, seed: u64 },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Predict { test, seed }) => {
            let (warn, weath) = big_data::read_data();
            let join = big_data::joining_data(warn, weath);
            let (y, x) = big_data::feature_target(join);
            let xmatrix = big_data::convert_features_to_matrix(&x);
            let _result = big_data::train_mod(xmatrix, y, test, seed);
            println!("Done!");
        }
        None => println!("Missing function parameter"),
    }
}
