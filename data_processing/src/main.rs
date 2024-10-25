use data_processing::{
    create_bar_plot, create_histogram, create_scatter_plot, generate_summary_statistics,
    load_records,
};
use ndarray::s;
use ndarray::Array2;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "../graduate_program_recommendation_data.csv";
    let records = load_records(file_path)?;

    // Generate and print summary statistics
    let (
        cgpa_min,
        cgpa_max,
        gre_quant_min,
        gre_quant_max,
        gre_verbal_min,
        gre_verbal_max,
        toefl_min,
        toefl_max,
    ) = generate_summary_statistics(&records);

    println!("CGPA: Min = {:.2}, Max = {:.2}", cgpa_min, cgpa_max);
    println!(
        "GRE Quantitative: Min = {:.2}, Max = {:.2}",
        gre_quant_min, gre_quant_max
    );
    println!(
        "GRE Verbal: Min = {:.2}, Max = {:.2}",
        gre_verbal_min, gre_verbal_max
    );
    println!(
        "TOEFL Score: Min = {:.2}, Max = {:.2}",
        toefl_min, toefl_max
    );

    // Convert records into ndarray array for plotting
    let num_records = records.len();
    let mut array = Array2::<f64>::zeros((num_records, 4));
    for (i, record) in records.iter().enumerate() {
        array[[i, 0]] = record.cgpa;
        array[[i, 1]] = record.gre_q.unwrap_or(0.0);
        array[[i, 2]] = record.gre_v.unwrap_or(0.0);
        array[[i, 3]] = record.toefl_score.unwrap_or(0.0);
    }

    // Extract columns and call plotting functions
    let cgpa_col = array.slice(s![.., 0]);
    let gre_quant_col = array.slice(s![.., 1]);
    let gre_verbal_col = array.slice(s![.., 2]);
    let toefl_col = array.slice(s![.., 3]);

    create_histogram(&cgpa_col)?;
    create_scatter_plot(&gre_quant_col, &gre_verbal_col)?;
    create_bar_plot(&toefl_col)?;

    Ok(())
}
