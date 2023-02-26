use aws_sdk_s3;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

use crate::consolidated_record::ConsolidatedRecord;

mod consolidated_record;
mod csv_data;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let path_params = event.query_string_parameters();
    let name_param: Option<&str> = path_params.first("name");
    let gender_param: Option<&str> = path_params.first("gender");

    match name_param {
        None => Err("Missing query param 'name' or 'gender'".into()),
        Some(name) => match gender_param {
            None => Err("Missing query param 'name' or 'gender'".into()),
            Some(gender) => {
                let config = aws_config::from_env().region("eu-west-1").load().await;
                let filename = &format!("{}_baby_names_1996_2021.csv", gender.to_string());

                let s3_client = aws_sdk_s3::Client::new(&config);

                let data = s3_client
                    .get_object()
                    .bucket("baby-names-1996-2021")
                    .key(filename)
                    .send()
                    .await
                    .expect(&format!("Failed to get file {}", filename));

                let s3_result = data.body.collect().await.unwrap().into_bytes();
                let response = std::str::from_utf8(&s3_result).unwrap();
                let mut rdr = csv::ReaderBuilder::new().from_reader(response.as_bytes());

                for record in rdr.records() {
                    let row = record.unwrap();

                    if row.iter().any(|field| field == name) {
                        let baby_name_record: Result<csv_data::CsvData, csv::Error> =
                            row.deserialize(None);

                        if let Ok(csv_data) = baby_name_record {
                            let consolidated_record: ConsolidatedRecord =
                                consolidated_record::calculate(&csv_data);

                            let json_data = serde_json::to_string(&consolidated_record)?;

                            let resp: Response<Body> = Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(json_data.into())
                                .map_err(Box::new)?;

                            return Ok::<Response<Body>, Error>(resp);
                        };
                    };
                }

                return Err("No records found".into());
            }
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
