# Project 4 - Big Data
This project will predict weather alerts in Durham County based on weather conditions at Raleigh-Durham International Airport. The first iteration of the project has a function to read in the data as polars data frames. It also has a command line tool framework using clap to interact with the functions. The second iteration of the project adds functions to print the data to make it easy to visualize. There is also a function to join the two dataframes based on the date so that each row has a boolean attribute based on whether there is an alert that day. There is some code written for the eventual modeling process, but next more data processing needs to be done. The third iteration adds functions to remove the target value from the data (whether or not there is a heat warning), to convert the features into a matrix, and to train a logistic regression model and output the test accuracy.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
