use crate::csv_data::CsvData;

#[derive(Debug)]
pub struct ConsolidatedRecord {
    name: String,
    avg_rank: i32,
    total_count: i32,
}

fn convert_to_i32(s: &String) -> i32 {
    return match s.parse::<i32>() {
        Err(_e) => 0,
        Ok(n) => n,
    };
}

pub fn calculate(csv_data: &CsvData) -> ConsolidatedRecord {
    let total = [
        convert_to_i32(&csv_data.count2021),
        convert_to_i32(&csv_data.count2020),
        convert_to_i32(&csv_data.count2019),
        convert_to_i32(&csv_data.count2018),
        convert_to_i32(&csv_data.count2017),
        convert_to_i32(&csv_data.count2016),
        convert_to_i32(&csv_data.count2015),
        convert_to_i32(&csv_data.count2014),
        convert_to_i32(&csv_data.count2013),
        convert_to_i32(&csv_data.count2012),
        convert_to_i32(&csv_data.count2011),
        convert_to_i32(&csv_data.count2010),
        convert_to_i32(&csv_data.count2009),
        convert_to_i32(&csv_data.count2008),
        convert_to_i32(&csv_data.count2007),
        convert_to_i32(&csv_data.count2006),
        convert_to_i32(&csv_data.count2005),
        convert_to_i32(&csv_data.count2004),
        convert_to_i32(&csv_data.count2003),
        convert_to_i32(&csv_data.count2002),
        convert_to_i32(&csv_data.count2001),
        convert_to_i32(&csv_data.count2000),
        convert_to_i32(&csv_data.count1999),
        convert_to_i32(&csv_data.count1998),
        convert_to_i32(&csv_data.count1997),
        convert_to_i32(&csv_data.count1996),
    ]
    .iter()
    .sum();

    let all_ranks: [i32; 26] = [
        convert_to_i32(&csv_data.rank2021),
        convert_to_i32(&csv_data.rank2020),
        convert_to_i32(&csv_data.rank2019),
        convert_to_i32(&csv_data.rank2018),
        convert_to_i32(&csv_data.rank2017),
        convert_to_i32(&csv_data.rank2016),
        convert_to_i32(&csv_data.rank2015),
        convert_to_i32(&csv_data.rank2014),
        convert_to_i32(&csv_data.rank2013),
        convert_to_i32(&csv_data.rank2012),
        convert_to_i32(&csv_data.rank2011),
        convert_to_i32(&csv_data.rank2010),
        convert_to_i32(&csv_data.rank2009),
        convert_to_i32(&csv_data.rank2008),
        convert_to_i32(&csv_data.rank2007),
        convert_to_i32(&csv_data.rank2006),
        convert_to_i32(&csv_data.rank2005),
        convert_to_i32(&csv_data.rank2004),
        convert_to_i32(&csv_data.rank2003),
        convert_to_i32(&csv_data.rank2002),
        convert_to_i32(&csv_data.rank2001),
        convert_to_i32(&csv_data.rank2000),
        convert_to_i32(&csv_data.rank1999),
        convert_to_i32(&csv_data.rank1998),
        convert_to_i32(&csv_data.rank1997),
        convert_to_i32(&csv_data.rank1996),
    ];

    let total_rank: i32 = all_ranks.iter().sum();

    return ConsolidatedRecord {
        name: csv_data.name.clone(),
        avg_rank: total_rank / all_ranks.len() as i32,
        total_count: total,
    };
}
