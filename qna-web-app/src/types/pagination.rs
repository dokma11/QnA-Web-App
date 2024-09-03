use std::collections::HashMap;
use handle_errors::Error;

#[derive(Default, Debug)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32
}

pub fn extract_pagination (
    params: HashMap<String, String>
) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
