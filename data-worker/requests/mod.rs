#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RabbitMQRequestType {
    Quotation,
    Fluctuation,
    Symbols,
}

impl FromStr for RabbitMQRequestType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quotation" => Ok(RabbitMQRequestType::Quotation),
            "Fluctuation" => Ok(RabbitMQRequestType::Fluctuation),
            "Symbols" => Ok(RabbitMQRequestType::Symbols),
            _ => Err(format!("{} is not a valid request type", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQRequest {
    date_start: NaiveDate,
    date_end: NaiveDate,
    base_symbol: String,
    request_type: RabbitMQRequestType,
}
