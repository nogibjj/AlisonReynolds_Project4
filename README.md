# Project 4 - Big Data
The goal of this project was to predict heat warnings in Durham County based on hourly weather conditions at Raleigh-Durham International Airport from May 2005 to September 2022. The heat alerts data was obtained from the National Weather Service Raleigh Weather Forecast Office and the weather conditions dataset was obtained from ERA5-Land. The weather variables are: air temperature (in degrees Farenheit), atmospheric pressure, solar radiation, dewpoint temperature (in degrees Farenheit), humidity (expressed as a percentage), two-meter and ten-meter wind speed, wet bulb globe temperature, and heat index. The heat alert data is a list of each day with any heat-related advisory issued for Durham County.

For this project, I used AWS S3 and Athena to store and summarize the data. Since the hourly weather conditions dataset was very large, I hosted it in an S3 bucket and used Athena to summarize it into daily metrics. Using SQL queries in Athena, I caculated the daily minimum, maximum, mean, and variance for each of the weather variables. These summaries statistics can then be used to predict whether there is a heat advisory for a given day.

![Diagram](Project4_Diagram.png)
   
   

The first iteration of the project has a function to read in the data as polars data frames. It also has a command line tool framework using clap to interact with the functions. The second iteration of the project adds functions to print the data to make it easy to visualize. There is also a function to join the two dataframes based on the date so that each row has a boolean attribute based on whether there is an alert that day. There is some code written for the eventual modeling process, but next more data processing needs to be done. The third iteration adds functions to remove the target value from the data (whether or not there is a heat warning), to convert the features into a matrix, and to train a logistic regression model and output the test accuracy.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [smartcore-example] (https://levelup.gitconnected.com/machine-learning-and-rust-part-3-smartcore-dataframe-and-linear-regression-10451fdc2e60)
