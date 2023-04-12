use polars::prelude::*;
use smartcore::linalg::basic::arrays::MutArray;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;

const WARNINGS: &str = "src/data/heat_alerts.csv";
const WEATHER: &str = "src/data/rdu_daily.csv";

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

pub fn feature_target(mut joined_df: DataFrame) -> (Vec<i32>, DataFrame) {
    let target = joined_df.drop_in_place("is_heat").unwrap();
    let _no_date = joined_df.drop_in_place("date").unwrap();

    let y: Vec<bool> = target.bool().unwrap().into_no_null_iter().collect();
    let y = y.into_iter().map(|x| x as i32).collect();

    (y, joined_df)
}

pub fn convert_features_to_matrix(x: &DataFrame) -> DenseMatrix<f64> {
    // function to convert feature dataframe to a DenseMatrix, readable by smartcore

    let nrows = x.height();
    let ncols = x.width();
    let tot = nrows * ncols;
    // convert to array
    let features_res = x.to_ndarray::<Float64Type>().unwrap();
    // create a zero matrix and populate with features

    let zero_vec = vec![0.0; tot];

    let mut xmatrix: DenseMatrix<f64> = DenseMatrix::new(nrows, ncols, zero_vec, false);
    // initialize the row and column counters
    let mut col: u32 = 0;
    let mut row: u32 = 0;

    for val in features_res.iter() {
        // converting into usize
        let m_row = usize::try_from(row).unwrap();
        let m_col = usize::try_from(col).unwrap();

        xmatrix.set((m_row, m_col), *val);
        // incrementing row and columns
        if m_col == ncols - 1 {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    xmatrix
}

// training function
pub fn train_mod(x: DenseMatrix<f64>, y: Vec<i32>, test: f32, seed: u64) -> f64 {
    // train split
    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, test, true, Some(seed));

    // model
    let log_regression = LogisticRegression::fit(&x_train, &y_train, Default::default()).unwrap();
    // predictions
    let preds = log_regression.predict(&x_test).unwrap();
    // metrics
    let acc = accuracy(&y_test, &preds);
    println!("Accuracy: {:?}", acc);
    acc
}
