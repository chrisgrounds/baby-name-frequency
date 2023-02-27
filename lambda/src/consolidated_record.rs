// use lambda_http::Body;
use serde::Serialize;

use crate::csv_data::CsvData;

#[derive(Debug, Serialize)]
pub struct ConsolidatedRecord {
    pub name: String,
    pub avg_rank: i32,
    pub total_count: i32,
    pub avg_count_year: f32,
    pub total_count_as_percentage: f32,
}

fn convert_to_i32(s: &String, default_value: i32) -> i32 {
    return match s.parse::<i32>() {
        Err(_e) => default_value,
        Ok(n) => n,
    };
}

const TOTAL_NUM_BOYS: f32 = 8258200.0;
const TOTAL_NUM_GIRLS: f32 = 7660371.0;

pub fn calculate(csv_data: &CsvData, gender: &str) -> ConsolidatedRecord {
    let total = [
        convert_to_i32(&csv_data.count2021, 0),
        convert_to_i32(&csv_data.count2020, 0),
        convert_to_i32(&csv_data.count2019, 0),
        convert_to_i32(&csv_data.count2018, 0),
        convert_to_i32(&csv_data.count2017, 0),
        convert_to_i32(&csv_data.count2016, 0),
        convert_to_i32(&csv_data.count2015, 0),
        convert_to_i32(&csv_data.count2014, 0),
        convert_to_i32(&csv_data.count2013, 0),
        convert_to_i32(&csv_data.count2012, 0),
        convert_to_i32(&csv_data.count2011, 0),
        convert_to_i32(&csv_data.count2010, 0),
        convert_to_i32(&csv_data.count2009, 0),
        convert_to_i32(&csv_data.count2008, 0),
        convert_to_i32(&csv_data.count2007, 0),
        convert_to_i32(&csv_data.count2006, 0),
        convert_to_i32(&csv_data.count2005, 0),
        convert_to_i32(&csv_data.count2004, 0),
        convert_to_i32(&csv_data.count2003, 0),
        convert_to_i32(&csv_data.count2002, 0),
        convert_to_i32(&csv_data.count2001, 0),
        convert_to_i32(&csv_data.count2000, 0),
        convert_to_i32(&csv_data.count1999, 0),
        convert_to_i32(&csv_data.count1998, 0),
        convert_to_i32(&csv_data.count1997, 0),
        convert_to_i32(&csv_data.count1996, 0),
    ]
    .iter()
    .sum();

    // TODO: Replace 22000 with length of dataset as they should be in the last position
    let all_ranks: [i32; 26] = [
        convert_to_i32(&csv_data.rank2021, 22000),
        convert_to_i32(&csv_data.rank2020, 22000),
        convert_to_i32(&csv_data.rank2019, 22000),
        convert_to_i32(&csv_data.rank2018, 22000),
        convert_to_i32(&csv_data.rank2017, 22000),
        convert_to_i32(&csv_data.rank2016, 22000),
        convert_to_i32(&csv_data.rank2015, 22000),
        convert_to_i32(&csv_data.rank2014, 22000),
        convert_to_i32(&csv_data.rank2013, 22000),
        convert_to_i32(&csv_data.rank2012, 22000),
        convert_to_i32(&csv_data.rank2011, 22000),
        convert_to_i32(&csv_data.rank2010, 22000),
        convert_to_i32(&csv_data.rank2009, 22000),
        convert_to_i32(&csv_data.rank2008, 22000),
        convert_to_i32(&csv_data.rank2007, 22000),
        convert_to_i32(&csv_data.rank2006, 22000),
        convert_to_i32(&csv_data.rank2005, 22000),
        convert_to_i32(&csv_data.rank2004, 22000),
        convert_to_i32(&csv_data.rank2003, 22000),
        convert_to_i32(&csv_data.rank2002, 22000),
        convert_to_i32(&csv_data.rank2001, 22000),
        convert_to_i32(&csv_data.rank2000, 22000),
        convert_to_i32(&csv_data.rank1999, 22000),
        convert_to_i32(&csv_data.rank1998, 22000),
        convert_to_i32(&csv_data.rank1997, 22000),
        convert_to_i32(&csv_data.rank1996, 22000),
    ];

    let total_rank: i32 = all_ranks.iter().sum();

    let all_counts = if gender == "boys" {
        TOTAL_NUM_BOYS
    } else {
        TOTAL_NUM_GIRLS
    };

    return ConsolidatedRecord {
        name: csv_data.name.clone(),
        avg_rank: total_rank / all_ranks.len() as i32,
        total_count: total,
        avg_count_year: total as f32 / 26.0,
        total_count_as_percentage: (total as f32 / all_counts) * 100.0,
    };
}
