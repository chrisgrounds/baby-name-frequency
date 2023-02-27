use lambda_http::aws_lambda_events::query_map::QueryMap;

pub enum Gender {
    Girl,
    Boy,
}

impl Gender {
    pub fn read_gender_or_default_to_girl(path_params: &QueryMap) -> Self {
        let gender_param: Option<&str> = path_params.first("gender");

        match gender_param {
            Some("boy") => self::Gender::Boy,
            Some("girl") => self::Gender::Girl,
            _ => self::Gender::Girl,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            self::Gender::Boy => "boys".to_string(),
            self::Gender::Girl => "girls".to_string(),
        }
    }

    pub fn is_girl(&self) -> bool {
        match self {
            self::Gender::Girl => true,
            _ => false,
        }
    }
}
