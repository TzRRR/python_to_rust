"""
This module performs descriptive statistics and visualizations
on a dataset related to graduate program recommendations.
"""

import pandas as pd
import matplotlib.pyplot as plt

# from ydata_profiling import ProfileReport


def main():
    # Load the dataset
    df = pd.read_csv("graduate_program_recommendation_data.csv")
    print(df.size)

    # Generate summary statistics including min and max
    summary_stats = df.describe().T[["mean", "50%", "std", "min", "max"]]
    summary_stats.columns = ["Mean", "Median", "Standard Deviation",
                             "Min", "Max"]
    print(summary_stats)

    # Visualizations
    plot_histogram(df)
    plot_scatter(df)
    plot_bar(df)


def summary_stats(df):
    return df


def plot_histogram(df):
    """
    Plot a histogram of CGPA.
    """
    plt.figure(figsize=(10, 6))
    df["cgpa"].hist(bins=20, edgecolor="black")
    plt.title("Distribution of CGPA")
    plt.xlabel("CGPA")
    plt.ylabel("Frequency")
    plt.grid(False)
    plt.show()


def plot_scatter(df):
    """
    Plot a scatter plot of GRE Quantitative vs Verbal scores.
    """
    plt.figure(figsize=(10, 6))
    plt.scatter(df["greQ"], df["greV"], alpha=0.5)
    plt.title("GRE Quantitative vs Verbal Scores")
    plt.xlabel("GRE Quantitative")
    plt.ylabel("GRE Verbal")
    plt.grid(True)
    plt.show()


def plot_bar(df):
    """
    Plot a bar chart of TOEFL Scores distribution.
    """
    plt.figure(figsize=(10, 6))
    df["toeflScore"].value_counts().sort_index().plot(kind="bar",
                                                      color="skyblue")
    plt.title("Distribution of TOEFL Scores")
    plt.xlabel("TOEFL Score")
    plt.ylabel("Frequency")
    plt.grid(False)
    plt.show()


if __name__ == "__main__":
    main()
