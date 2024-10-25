# python_to_rust

[![CI](https://github.com/TzRRR/python_to_rust/actions/workflows/python_cicd.yml/badge.svg)](https://github.com/TzRRR/python_to_rust/actions/workflows/python_cicd.yml)
[![CI](https://github.com/TzRRR/python_to_rust/actions/workflows/rust_cicd.yml/badge.svg)](https://github.com/TzRRR/python_to_rust/actions/workflows/rust_cicd.yml)

This project demonstrates the conversion of a Python data processing script to Rust, with a focus on improving performance. It includes a comparison of execution time, system usage, and CPU utilization between the Python and Rust implementations.

## Project Structure

```bash
python_to_rust/
│
├── .github/
│   └── workflows/
│       ├── python_cicd.yml
│       └── rust_cicd.yml
│
├── .pytest_cache/                   # Cache files for pytest
├── data_processing/
│   ├── src/                         # Source files for the Rust project
│   │   ├── lib.rs                   # Library file for Rust project (includes tests)
│   │   └── main.rs                  # Main file for Rust project
│   ├── Cargo.lock                   # Dependency lock file for Rust
│   ├── Cargo.toml                   # Rust project configuration file
│   ├── cgpa_histogram.png            # Output image: Histogram of CGPA
│   ├── graduate_program_recommendation_data.csv # Dataset for Rust project
│   ├── gre_scatter_plot.png          # Output image: Scatter plot of GRE scores
│   ├── sample_graduate_program_data.csv # Sample dataset
│   ├── toefl_bar_plot.png            # Output image: Bar plot of TOEFL scores
│   └── Makefile                      # Makefile for building, running, and testing Rust project
│
├── python_script/
│   ├── __pycache__/                  # Python cache files
│   ├── .pytest_cache/                # Cache files for pytest
│   ├── venv/                         # Python virtual environment folder
│   ├── descriptive_stats.py          # Main Python script
│   ├── graduate_program_recommendation_data.csv # Dataset for Python project
│   ├── requirements.txt              # Python dependencies file
│   └── test_descriptive_stats.py     # Unit tests for the Python script
│
├── .gitignore                        # Git ignore file
├── performance_comparison.md         # Performance comparison between Python and Rust
└── README.md                         # Project README file
```


## Installing Dependencies

to run the Python script, first run:

```bash
pip install -r requirements.txt
```

Rust cargo automatically installs dependencies in the build process.

## Build and run the Rust project

```bash
make build
make run
```

## Run the Python script

```bash
python descriptive_stats.py
```

## time the scripts

```bash
time cargo run
time python descriptive_stats.py
```
