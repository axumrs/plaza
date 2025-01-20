use axum::Json;
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Response { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Response::new(0, "OK".to_string(), data)
    }

    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Response<()> {
    pub fn err(e: Error) -> Self {
        Self::new(e.code(), e.to_string(), ())
    }
    pub fn empty() -> Self {
        Self::ok(())
    }
}

#[derive(Serialize)]
pub struct IdResp {
    pub id: String,
}

#[derive(Serialize)]
pub struct AffResp {
    pub rows: u64,
}

pub type JsonResp<T> = Json<Response<T>>;
pub type JsonIdResp = Json<Response<IdResp>>;
pub type JsonAffResp = Json<Response<AffResp>>;

pub fn ok<T: Serialize>(data: T) -> crate::Result<JsonResp<T>> {
    Ok(Response::ok(data).to_json())
}

pub fn empty() -> crate::Result<JsonResp<()>> {
    Ok(Response::empty().to_json())
}

pub fn id(id: impl Into<String>) -> crate::Result<JsonIdResp> {
    Ok(Response::ok(IdResp { id: id.into() }).to_json())
}

pub fn aff(rows: impl Into<u64>) -> crate::Result<JsonAffResp> {
    Ok(Response::ok(AffResp { rows: rows.into() }).to_json())
}
