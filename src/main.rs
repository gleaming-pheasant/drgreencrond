mod forecast;

fn main() {
    let forecast = forecast::Forecast::fetch_fw_24h_postcode(
        "YO1", time::UtcDateTime::now()
    ).unwrap();

    println!("{:#?}", forecast);

    println!("The lowest carbon intensity period is: {:#?}", forecast.lowest());
}