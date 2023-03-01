pub enum Gender {
    Girl,
    Boy,
}

impl Gender {
    pub fn read_gender_or_default_to_girl(gender_str: &str) -> Self {
        match gender_str {
            "boy" => self::Gender::Boy,
            "girl" => self::Gender::Girl,
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
