use crate::prelude::*;

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub(crate) method: String,
    pub(crate) url: String,
    pub(crate) headers: Vec<HeaderField>,
    pub(crate) body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub(crate) status_code: u16,
    pub(crate) headers: Vec<HeaderField>,
    pub(crate) body: Vec<u8>,
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}
