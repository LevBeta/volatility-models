use volatility_models::csv_reader::read_csv;

use volatility_models::garman_klass;

pub fn main() {
    let (open, high, low, close) = read_csv("./test_data/BTCUSDT-1m-2023-04-15.csv").unwrap();

    let gk = garman_klass(
        &open,
        &high,
        &low,
        &close,
        30,
        365.0
    );

    println!("{:?}", gk);
}