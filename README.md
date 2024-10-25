# python_to_rust

[![CI](https://github.com/TzRRR/python_to_rust/actions/workflows/cicd.yml/badge.svg)](https://github.com/TzRRR/python_to_rust/actions/workflows/cicd.yml)

This project demonstrates the conversion of a Python data processing script to Rust, with a focus on improving performance. It includes a comparison of execution time, system usage, and CPU utilization between the Python and Rust implementations.

## Project Structure

```bash
.
├── .github/
│   └── workflows/
│       └── cicd.yml               # GitHub Actions CI/CD pipeline
├── data_processing/
│   ├── src/                       # Rust source files
│   │   ├── main.rs                # Main entry point for the Rust project
│   │   └── lib.rs                 # Functions and logic for the project
│   ├── target/                    # Build output directory
│   ├── Cargo.toml                 # Rust dependencies and project metadata
│   ├── Makefile                   # Build and test automation
│   ├── sample_graduate_program_data.csv  # Sample dataset
│   └── generated plots (PNG files)
├── venv/                          # Python virtual environment
├── descriptive_stats.py            # Original Python script
├── graduate_program_recommendation_data.csv  # Original dataset
├── performance_comparison.md       # Performance report comparing Python and Rust implementations
├── requirements.txt                # Python dependencies
└── README.md                       # This README file

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
