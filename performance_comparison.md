# Performance Report: Python vs. Rust Implementation

## Overview

This report provides a comparative analysis of two scripts—one written in Python and the other in Rust. Both scripts implement the same logic and perform identical operations. The primary goal of this analysis is to evaluate the differences in execution time, system usage, and CPU utilization between the two languages.

## Performance Metrics

### Python Script

- **Command**: `python descriptive_stats.py`
- **User Time**: 2.42 seconds
- **System Time**: 1.14 seconds
- **CPU Usage**: 54%
- **Total Time**: 6.517 seconds

### Rust Script

- **Command**: `cargo run`
- **User Time**: 1.46 seconds
- **System Time**: 0.12 seconds
- **CPU Usage**: 7%
- **Total Time**: 21.163 seconds

## Analysis

1. **Execution Time (User + System)**:

   - **Python**: 3.56 seconds (2.42s user + 1.14s system)
   - **Rust**: 1.58 seconds (1.46s user + 0.12s system)
   - The Rust script performs the task approximately **44% faster** in terms of execution time, with significantly lower system time usage.

2. **Total Time (Elapsed)**:

   - **Python**: 6.517 seconds
   - **Rust**: 21.163 seconds
   - While the Rust script exhibits faster execution in terms of raw processing time, the total elapsed time is higher, which may indicate differences in I/O handling, startup time, or task completion on the system.

3. **CPU Utilization**:
   - **Python**: 54% CPU usage
   - **Rust**: 7% CPU usage
   - The Python script utilizes a significantly higher percentage of CPU resources compared to Rust. Rust's lower CPU usage may imply a more efficient allocation of resources for this specific task.

## Summary

The Rust implementation demonstrates better raw execution performance with reduced system and user time, achieving the task with a **44% faster execution time** and **significantly lower CPU utilization**. However, the higher total elapsed time in Rust suggests further investigation into possible bottlenecks outside of computation, such as I/O handling or process management.

### Conclusion

Rust is advantageous for computational efficiency and CPU utilization in this specific task, while Python’s lower total elapsed time suggests it may be preferable for tasks with minimal I/O overhead. Further optimization in Rust could enhance its performance for broader use cases.
