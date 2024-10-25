import pandas as pd
import matplotlib.pyplot as plt

# Load the dataset
df = pd.read_csv('graduate_program_recommendation_data.csv')

# Generate summary statistics including min and max
summary_stats = df.describe().T[['mean', '50%', 'std', 'min', 'max']]
summary_stats.columns = ['Mean', 'Median', 'Standard Deviation', 'Min', 'Max']
print(summary_stats)

# Visualizations

# 1. Histogram of CGPA
plt.figure(figsize=(10, 6))
df['cgpa'].hist(bins=20, edgecolor='black')
plt.title('Distribution of CGPA')
plt.xlabel('CGPA')
plt.ylabel('Frequency')
plt.grid(False)
plt.show()

# 2. Scatter plot for GRE Quantitative vs GRE Verbal
plt.figure(figsize=(10, 6))
plt.scatter(df['greQ'], df['greV'], alpha=0.5)
plt.title('GRE Quantitative vs Verbal Scores')
plt.xlabel('GRE Quantitative')
plt.ylabel('GRE Verbal')
plt.grid(True)
plt.show()

# 3. Bar plot for TOEFL Scores
plt.figure(figsize=(10, 6))
df['toeflScore'].value_counts().sort_index().plot(kind='bar', color='skyblue')
plt.title('Distribution of TOEFL Scores')
plt.xlabel('TOEFL Score')
plt.ylabel('Frequency')
plt.grid(False)
plt.show()
