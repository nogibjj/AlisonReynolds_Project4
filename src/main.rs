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
    Predict {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Predict {}) => {
            let (warn, weath) = big_data::read_data();
            //big_data::print_rows(warn.clone());
            //big_data::print_rows(weath.clone());
            let join = big_data::joining_data(warn, weath);
            //big_data::print_rows(join);
            let (y, x) = big_data::feature_target(join);
            let xmatrix = big_data::convert_features_to_matrix(&x);
            let _result = big_data::train_mod(xmatrix, y);
            println!("Done!");
        }
        None => println!("Missing function parameter"),
    }
}
