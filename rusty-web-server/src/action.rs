use crate::request::{ParsedHttpRequest, RawHttpRequest};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Action {
    pub operation: Operation,
    pub stream: RawHttpRequest,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Get,
    Add(i64),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s
            .to_string()
            .split("/")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let path_action = path
            .get(1)
            .map(|a| a.to_string())
            .ok_or("Invalid Path".to_string())?;

        let action = match path_action.as_str() {
            "add" => {
                let value = path
                    .get(2)
                    .ok_or("Add value not specified".to_string())?
                    .parse::<i64>()
                    .map_err(|e| e.to_string())?;
                Self::Add(value)
            }
            "get" => Self::Get,
            _unsupported => return Err("Unsupported operation".to_string()),
        };

        Ok(action)
    }
}

impl Action {
    pub fn from_parsed_http_request(http_request: ParsedHttpRequest) -> Result<Self, String> {
        let ParsedHttpRequest { stream, path } = http_request;

        let operation = Operation::from_str(&path)?;

        Ok(Self { operation, stream })
    }
}
