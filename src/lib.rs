use polars::prelude::*;
//use linfa::prelude::*;
//use linfa_elasticnet::{ElasticNet, Result};

const WARNINGS: &str = "src/data/heat_alerts.csv";
const WEATHER: &str = "src/data/weather_conditions.csv";

pub fn read_data() -> (DataFrame, DataFrame) {
    let warn_df = CsvReader::from_path(WARNINGS).unwrap().finish().unwrap();
    let weather_df = CsvReader::from_path(WEATHER).unwrap().finish().unwrap();
    (warn_df, weather_df)
}

// printing the first 5 rows of dataframe
pub fn print_rows(df: DataFrame) {
    println!("Data:");
    println!("{}", &df.head(Some(5)));
}

// joining the two dataframes on the date column
pub fn joining_data(warn_df: DataFrame, weather_df: DataFrame) -> DataFrame {
    let joined_df = weather_df.left_join(&warn_df, ["date"], ["date"]).unwrap();
    println!("Joined!");
    joined_df
}


//accepts ratio of training data
//pub fn predicting(joined_df: DataFrame) -> Result<()> {
    // randomly select 80% of the data for training
//    let (train, valid) = joined_df
//        .with_column(
            // create a random boolean column
//            col("is_train").apply(|_| {
//                let mut rng = rand::thread_rng();
//                rng.gen_bool(0.8)
//            }),
//        )
//        .split_by_rand_bool("is_train", 0.8);

    // train pure LASSO model with 0.3 penalty
 //   let model = ElasticNet::params()
//        .penalty(0.3)
//        .l1_ratio(1.0)
 //       .fit(&train)?;

//    println!("intercept:  {}", model.intercept());
//    println!("params: {}", model.hyperplane());

 //   println!("z score: {:?}", model.z_score());

    // validate
//    let y_est = model.predict(&valid);
 //   println!("predicted variance: {}", valid.r2(&y_est)?);

 //   Ok(())
//}
