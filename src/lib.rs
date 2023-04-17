pub mod csv_reader;
// pub mod GermanKlass;

//GarmanKlass
pub fn garman_klass(
    // Using each vector as param instead a struct so i dont need to define one, and its easier to add on new projects
    open: &Vec<f64>,
    high: &Vec<f64>,
    low: &Vec<f64>,
    close: &Vec<f64>,
    window: usize,
    trading_days: f64
) -> Vec<f64> {
    let log_hl: Vec<f64> = high.iter().zip(low.iter())
                            .map(|(&h, &l)| (h / l).ln())
                            .collect();

    let log_co: Vec<f64> = close.iter().zip(open.iter())
                            .map(|(&c, &o)| (c / o).ln())
                            .collect();

    let rs: Vec<f64> = log_hl.iter().zip(log_co.iter())
                        .map(|(&hl, &co)| 0.5 * hl.powi(2) - (2.0 * f64::ln(2.0) - 1.0) * co.powi(2))
                        .collect();

    let result: Vec<f64> = rs.windows(window)
                                .map(|window| (trading_days * window.iter().sum::<f64>() / (window.len() as f64)).sqrt())
                                .collect();

    result
}

