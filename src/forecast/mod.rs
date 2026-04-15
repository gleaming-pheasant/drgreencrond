mod response;

#[derive(Debug)]
pub struct ForecastPeriod {
    from: time::UtcDateTime,
    to: time::UtcDateTime,
    pub forecast: u16
}

#[derive(Debug)]
pub struct Forecast(Vec<ForecastPeriod>);

impl Forecast {
    pub fn fetch_fw_24h_postcode<S: AsRef<str>>(
        postcode: S,
        from: time::UtcDateTime,
    ) -> Result<Forecast, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.carbonintensity.org.uk/regional/intensity/{}/fw24h/postcode/{}",
            from.format(&time::format_description::well_known::Iso8601::DEFAULT).unwrap(),
            postcode.as_ref()
        );

        println!("{url}");
        
        let response = ureq::get(url)
            .call()?
            .into_body()
            .read_json::<response::Body>()?;

        Ok(response.into())
    }

    pub fn lowest(&self) -> &ForecastPeriod {
        self.0.iter().min_by_key(|period| period.forecast ).unwrap()
    }
}

impl From<response::Body> for Forecast {
    fn from(value: response::Body) -> Self {
        Forecast(
            value
                .data
                .data
                .into_iter()
                .map(|period| ForecastPeriod {
                    from: period.from.to_utc(),
                    to: period.to.to_utc(),
                    forecast: period.intensity.forecast,
                }).collect()
        )
    }
}