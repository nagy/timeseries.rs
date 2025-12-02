use chrono::prelude::*;
use csv;
use dtinfer;
use serde::Serialize;
use std::error::Error;
use std::io::{Read, Write};

use crate::TimeSeries;

#[derive(Serialize)]
struct Row {
    timestamp: String,
    value: f64,
}

/// Load series from the given CSV reader
pub fn read<R: Read>(reader: R) -> Result<TimeSeries, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut index: Vec<i64> = Vec::new();
    let mut data: Vec<f64> = Vec::new();
    let mut inferred_format: Option<String> = None;
    for result in rdr.records() {
        let record = result?;
        if inferred_format.is_none() {
            inferred_format = dtinfer::infer_best(&record[0]);
        }
        if let Some(datetime_format) = &inferred_format {
            println!("[{}]", &record[0]);
            println!("{}", &datetime_format);
            println!(
                "{:?}",
                NaiveDateTime::parse_from_str(&record[0], datetime_format)
            );
            let idx = NaiveDateTime::parse_from_str(&record[0], datetime_format)?
                .and_utc()
                .timestamp_millis();
            let v: f64 = record[1].parse::<f64>()?;
            index.push(idx);
            data.push(v);
        }
    }

    Ok(TimeSeries::new(index, data))
}

fn timestamp_format(ts: i64, format: &str) -> String {
    let dt = Utc.timestamp_opt(ts / 1000, 0).unwrap();
    dt.format(format).to_string()
}

/// Save series as CSV writer
pub fn write<W: Write>(
    writer: W,
    ts: &TimeSeries,
    datetime_format: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    ts.iter()
        .map(|dp| Row {
            timestamp: timestamp_format(dp.timestamp, datetime_format),
            value: dp.value,
        })
        .for_each(|row| wtr.serialize(&row).unwrap());
    Ok(())
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read() {
        let file = File::open("testdata/rain.csv").unwrap();
        let ts = read(file).unwrap();
        assert_eq!(ts.len(), 96670);
    }
}
