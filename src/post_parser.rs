use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct YamlFrontMatter {
    pub(crate) title: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) metadata: HashMap<String, String>,
}

impl Default for YamlFrontMatter {
    fn default() -> Self {
        YamlFrontMatter {
            title: "Untitled".to_string(),
            date: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

impl YamlFrontMatter {
    pub fn new(
        title: String,
        date: DateTime<Utc>,
        metadata: HashMap<String, String>,
    ) -> YamlFrontMatter {
        YamlFrontMatter {
            title,
            date,
            metadata,
        }
    }
}

pub fn parse_markdown(file_bytes: Vec<u8>) -> (YamlFrontMatter, Vec<u8>) {
    if file_bytes.len() <= 3 || !file_bytes.starts_with(b"---") {
        return (YamlFrontMatter::default(), file_bytes);
    }

    let mut end_idx = file_bytes.len();
    for i in 3..(file_bytes.len() - 3) {
        let slice = file_bytes.get(i..i + 3).unwrap();
        if slice == [b'-', b'-', b'-'].as_slice() {
            end_idx = i - 1;
            break;
        }
    }

    let fm_lines = file_bytes[3..end_idx]
        .split(|&b| b == b'\n')
        .map(|line| str::from_utf8(line.strip_suffix(b"\r").unwrap_or(line)).unwrap_or(""))
        .map(|line| line.split_once(":").unwrap_or(("", "")))
        .map(|line| (line.0, line.1.trim().replace("\"", "").replace("\'", "")))
        .filter(|x| !x.0.is_empty())
        .collect::<HashMap<&str, String>>();

    (
        YamlFrontMatter {
            title: fm_lines.get("title").unwrap_or(&"Untitled".to_string()).to_string(),
            date: DateTime::parse_from_str(
                fm_lines.get("date").unwrap_or(

                    &Utc::now().to_string(),
                ),
                "%Y-%m-%dT%H:%M:%S%z",
            )
            .unwrap()
            .to_utc(),
            metadata: fm_lines
                .into_iter()
                .filter(|x| x.0 != "title" && x.0 != "date")
                .map(|x| (x.0.to_string(), x.1.to_string()))
                .collect(),
        },
        file_bytes[end_idx + 4..].to_vec(),
    )
}

mod tests {
    use super::*;
    use chrono::TimeZone;
    #[test]
    fn test_parsing_default_case() {
        let input = "---\n\
        title: hi\n\
        date: 2023-03-17T20:55:00+08:00\n\
        ---\n\
        m"
        .as_bytes()
        .to_vec();

        let (fm, file_bytes) = parse_markdown(input);
        assert_eq!(fm.title, "hi");
        assert_eq!(
            fm.date,
            Utc.with_ymd_and_hms(2023, 3, 17, 12, 55, 0).unwrap()
        );
        assert_eq!(file_bytes, vec![b'\n', b'm']);
    }

    #[test]
    fn test_parsing_with_extra_meta() {
        let input = "---\n\
title: hi\n\
date: 2023-03-17T20:55:00+08:00\n\
ni: hao
---\n\
        m"
        .as_bytes()
        .to_vec();

        let (fm, file_bytes) = parse_markdown(input);
        assert_eq!(fm.title, "hi");
        assert_eq!(
            fm.date,
            Utc.with_ymd_and_hms(2023, 3, 17, 12, 55, 0).unwrap()
        );
        assert_eq!(
            fm.metadata,
            HashMap::from([("ni".to_string(), "hao".to_string())])
        );
        assert_eq!(file_bytes, vec![b'\n', b'm']);
    }

    #[test]
    fn test_parsing_no_fm() {
        let input = "hiii".as_bytes().to_vec();
        let (fm, file_bytes) = parse_markdown(input);
        assert_eq!(fm.title, "Untitled");
        assert_eq!(file_bytes, vec![b'h', b'i', b'i', b'i']);
    }
}
