#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Quotation,
    Fluctuation,
    Symbols,
}

impl FromStr for RequestType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quotation" => Ok(RequestType::Quotation),
            "Fluctuation" => Ok(RequestType::Fluctuation),
            "Symbols" => Ok(RequestType::Symbols),
            _ => Err(format!("{} is not a valid request type", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    date_start: NaiveDate,
    date_end: NaiveDate,
    base_symbol: String,
    request_type: RequestType,
}
