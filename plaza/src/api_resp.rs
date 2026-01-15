use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Resp<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: impl Into<String>, data: T) -> Self {
        Self {
            code,
            msg: msg.into(),
            data,
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "OK", data)
    }

    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Resp<()> {
    pub fn err(e: crate::Error) -> Self {
        Self::new(e.code(), e.msg(), ())
    }
}

pub type JsonResp<T> = Json<Resp<T>>;

#[derive(Serialize)]
pub struct Id {
    pub id: String,
}

#[derive(Serialize)]
pub struct Aff {
    pub rows: u64,
}

pub fn ok<T: Serialize>(data: T) -> Resp<T> {
    Resp::ok(data)
}

pub fn err(e: crate::Error) -> Resp<()> {
    Resp::err(e)
}

pub fn ok_empty() -> Resp<()> {
    Resp::ok(())
}
