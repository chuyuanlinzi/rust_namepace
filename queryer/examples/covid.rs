use anyhow::Result;
use std::io::Cursor;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::get(url).await?.text().await?;
    // 使用 polars 直接请求
    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;

    let filtered = df.filter(&df["new_deaths"].gt(5))?;
    println!("{:?}",
             filtered.select((
                 "location",
                 "total_cases",
                 "new_cases",
                 "total_deaths",
                 "new_deaths"
             ))
    );

    Ok(())
}