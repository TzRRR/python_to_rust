use csv::ReaderBuilder;
use ndarray::{s, Array2, ArrayView1};
use plotters::prelude::*;
use std::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct GraduateRecord {
    pub cgpa: f64,
    pub gre_q: Option<f64>,
    pub gre_v: Option<f64>,
    pub toefl_score: Option<f64>,
}

pub fn load_records(file_path: &str) -> Result<Vec<GraduateRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: GraduateRecord = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn generate_summary_statistics(
    records: &[GraduateRecord],
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let num_records = records.len();
    let mut array = Array2::<f64>::zeros((num_records, 4));

    for (i, record) in records.iter().enumerate() {
        array[[i, 0]] = record.cgpa;
        array[[i, 1]] = record.gre_q.unwrap_or(0.0);
        array[[i, 2]] = record.gre_v.unwrap_or(0.0);
        array[[i, 3]] = record.toefl_score.unwrap_or(0.0);
    }

    // Extract columns
    let cgpa_col = array.slice(s![.., 0]);
    let gre_quant_col = array.slice(s![.., 1]);
    let gre_verbal_col = array.slice(s![.., 2]);
    let toefl_col = array.slice(s![.., 3]);

    // Compute summary statistics
    let cgpa_min = cgpa_col.fold(f64::INFINITY, |a, &b| a.min(b));
    let cgpa_max = cgpa_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let gre_quant_min = gre_quant_col.fold(f64::INFINITY, |a, &b| a.min(b));
    let gre_quant_max = gre_quant_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let gre_verbal_min = gre_verbal_col.fold(f64::INFINITY, |a, &b| a.min(b));
    let gre_verbal_max = gre_verbal_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let toefl_min = toefl_col.fold(f64::INFINITY, |a, &b| a.min(b));
    let toefl_max = toefl_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    (
        cgpa_min,
        cgpa_max,
        gre_quant_min,
        gre_quant_max,
        gre_verbal_min,
        gre_verbal_max,
        toefl_min,
        toefl_max,
    )
}

pub fn create_histogram(cgpa: &ArrayView1<f64>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("cgpa_histogram.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("CGPA Histogram", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10f64, 0u32..10)?;

    chart.configure_mesh().draw()?;

    // Manually binning CGPA values
    let mut bins = [0; 10]; // 10 bins for CGPA from 0.0 to 10.0
    for &value in cgpa.iter() {
        let bin_index = (value.floor() as usize).min(9); // Binning CGPA values
        bins[bin_index] += 1;
    }

    chart.draw_series(bins.iter().enumerate().map(|(i, &count)| {
        let x0 = i as f64;
        let x1 = (i + 1) as f64;
        Rectangle::new([(x0, 0), (x1, count as u32)], BLUE.filled())
    }))?;

    root.present()?;
    Ok(())
}

pub fn create_scatter_plot(
    gre_quant: &ArrayView1<f64>,
    gre_verbal: &ArrayView1<f64>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("gre_scatter_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "GRE Quantitative vs GRE Verbal",
            ("sans-serif", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..170f64, 0f64..170f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        gre_quant
            .iter()
            .zip(gre_verbal.iter())
            .map(|(&x, &y)| Circle::new((x, y), 3, BLUE.filled())),
    )?;

    root.present()?;
    Ok(())
}

pub fn create_bar_plot(toefl_scores: &ArrayView1<f64>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("toefl_bar_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("TOEFL Scores", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0usize..toefl_scores.len(), 0f64..120f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        toefl_scores
            .iter()
            .enumerate()
            .map(|(i, &score)| Rectangle::new([(i, 0.0), (i + 1, score)], BLUE.filled())),
    )?;

    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use std::path::Path;

    #[test]
    fn test_load_records() {
        // Load records from a sample file
        let file_path = format!(
            "{}/sample_graduate_program_data.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let records = load_records(file_path.as_str()).expect("Failed to load records");

        // Ensure records are loaded
        assert!(!records.is_empty(), "No records loaded");
        // Check the first record's CGPA
        assert_eq!(records[0].cgpa, 3.8);
        // Ensure GRE Quantitative score is present
        assert!(
            records[0].gre_q.is_some(),
            "GRE Quantitative score missing in first record"
        );
    }

    #[test]
    fn test_summary_statistics() {
        let test_data = array![
            [3.8, 160.0, 150.0, 110.0],
            [4.0, 165.0, 155.0, 115.0],
            [3.9, 162.0, 151.0, 112.0],
            [3.7, 158.0, 149.0, 108.0]
        ];

        let cgpa_col = test_data.slice(s![.., 0]);
        let gre_quant_col = test_data.slice(s![.., 1]);
        let gre_verbal_col = test_data.slice(s![.., 2]);
        let toefl_col = test_data.slice(s![.., 3]);

        // CGPA statistics
        let cgpa_min = cgpa_col.fold(f64::INFINITY, |a, &b| a.min(b));
        let cgpa_max = cgpa_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(cgpa_min, 3.7);
        assert_eq!(cgpa_max, 4.0);

        // GRE Quantitative statistics
        let gre_quant_min = gre_quant_col.fold(f64::INFINITY, |a, &b| a.min(b));
        let gre_quant_max = gre_quant_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(gre_quant_min, 158.0);
        assert_eq!(gre_quant_max, 165.0);

        // GRE Verbal statistics
        let gre_verbal_min = gre_verbal_col.fold(f64::INFINITY, |a, &b| a.min(b));
        let gre_verbal_max = gre_verbal_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(gre_verbal_min, 149.0);
        assert_eq!(gre_verbal_max, 155.0);

        // TOEFL statistics
        let toefl_min = toefl_col.fold(f64::INFINITY, |a, &b| a.min(b));
        let toefl_max = toefl_col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(toefl_min, 108.0);
        assert_eq!(toefl_max, 115.0);
    }

    #[test]
    fn test_create_histogram() {
        let cgpa_col = array![3.8, 4.0, 3.9, 3.7];
        let result = create_histogram(&cgpa_col.view());
        assert!(result.is_ok());
        // You can add further validation by checking if the "cgpa_histogram.png" exists
    }

    #[test]
    fn test_create_scatter_plot() {
        let gre_quant_col = array![160.0, 165.0, 162.0, 158.0];
        let gre_verbal_col = array![150.0, 155.0, 151.0, 149.0];
        let result = create_scatter_plot(&gre_quant_col.view(), &gre_verbal_col.view());
        assert!(result.is_ok());
        // You can add further validation by checking if the "gre_scatter_plot.png" exists
    }

    #[test]
    fn test_create_bar_plot() {
        let toefl_col = array![110.0, 115.0, 112.0, 108.0];
        let result = create_bar_plot(&toefl_col.view());
        assert!(result.is_ok());
        // You can add further validation by checking if the "toefl_bar_plot.png" exists
    }
}
