"""
Test module for descriptive statistics functions.
"""

import pandas as pd
from descriptive_stats import summary_stats

def sample_dataframe():
    """
    Fixture to load the dataset for testing.
    """
    df = pd.read_csv('graduate_program_recommendation_data.csv')
    return df

def test_summary_stats():
    """
    Test to verify that the summary statistics are correctly calculated.
    """
    sample_dataframe = pd.read_csv('python_script/graduate_program_recommendation_data.csv')

    # Calculate the summary statistics for the test
    summary_stats = sample_dataframe.describe().T[['mean', '50%', 'std', 'min', 'max']]
    summary_stats.columns = ['Mean', 'Median', 'Standard Deviation', 'Min', 'Max']
    
    # Check that summary statistics are calculated for all numeric columns
    assert not summary_stats.empty, "Summary statistics should not be empty."
    
    # Check that the correct columns exist
    expected_columns = ['Mean', 'Median', 'Standard Deviation', 'Min', 'Max']
    assert list(summary_stats.columns) == expected_columns, "Column names do not match the expected names."
    
    # Check that the summary stats DataFrame has the correct number of rows
    # (one for each numeric column)
    numeric_columns = sample_dataframe.select_dtypes(include='number').columns
    assert len(summary_stats) == len(numeric_columns), (
        "The number of rows in summary stats does not match the number of numeric columns in the dataframe."
    )


if __name__ == "__main__":
    df = sample_dataframe()
    # test_summary_stats()
    test_summary_stats()