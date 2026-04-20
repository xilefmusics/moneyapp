pub mod yyyy_mm_dd_option {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer);
        if s.is_err() {
            return Ok(None);
        }
        let s = s.unwrap();

        let mut iter = s.split('-');

        let year = iter.next();
        if year.is_none() {
            return Ok(None);
        }
        let year = year.unwrap().parse::<i32>();
        if year.is_err() {
            return Ok(None);
        }
        let year = year.unwrap();
        let month = iter.next().unwrap_or("1").parse::<u32>().unwrap_or(1);
        let day = iter.next().unwrap_or("1").parse::<u32>().unwrap_or(1);

        Ok(Some(
            Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
        ))
    }
}

pub mod yyyy_mm_dd {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = format!("{} 0:0:0", String::deserialize(deserializer)?);
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local::now().offset().clone(),
        ))
    }
}

pub mod dd_mm_yyyy_option {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let s = format!("{} 0:0:0", s);
        let dt = NaiveDateTime::parse_from_str(&s, "%d.%m.%Y %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local::now().offset().clone(),
        )))
    }
}

pub mod amount {
    use serde::{self, Deserialize, Deserializer};

    fn trim(s: &str) -> &str {
        let mut start = 0;
        let mut end = s.len();

        for (i, c) in s.char_indices().rev() {
            if c.is_digit(10) {
                end = i + c.len_utf8();
                break;
            }
        }

        for (i, c) in s.char_indices() {
            if c.is_digit(10) {
                start = i;
                break;
            }
        }

        &s[start..end]
    }

    fn handle_comma_dot(s: &str) -> String {
        if s.contains(',') {
            s.to_string().replace('.', "").replace(',', ".")
        } else {
            s.to_string()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = handle_comma_dot(trim(&String::deserialize(deserializer)?));
        let mut iter = s.split('.');
        let before = iter.next().unwrap().parse::<i64>().unwrap();
        let after_str = iter.next().unwrap_or("0");
        let after_mul = if !after_str.starts_with('0') && after_str.len() == 1 {
            10
        } else {
            1
        };
        let after = after_str.parse::<i64>().unwrap() * after_mul;
        Ok((before * 100 + after) as usize)
    }
}
