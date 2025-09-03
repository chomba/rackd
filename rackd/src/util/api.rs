use std::{collections::{HashMap, HashSet}, default, fmt::Display, marker::PhantomData, time::SystemTime};
use axum::{body::Bytes, extract::{FromRequest, FromRequestParts, OriginalUri}, http::{header, StatusCode}};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct Error {
    pub code: String,
    pub message: String
    // source ..
}

impl Error {
    pub fn new<M>(code: &str, message: M) -> Self where M: Into<String> {
        Self {
            code: String::from(code),
            message: message.into(),
            ..Default::default()
        }
    }
}

// impl From<Vec<Error>> for Response {
//     fn from(errors: Vec<Error>) -> Self {
//         Response {
//             errors,
//             success: false,
//             ..Default::default()
//         }
//     }
// }

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct Response<T = ()> {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    success: bool,
    errors: Vec<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
}

impl<T> Response<T> where T: Default {
    // TBD: ok and err need to take a reference to the request to 
    // extract the path, add a request Id and timestamp the request
    pub fn ok(result: T, path: &str) -> Self {
        Self {
            result: Some(result),
            success: true,
            path: Some(String::from(path)),
            timestamp: Some(chrono::offset::Utc::now().to_string()),
            errors: Vec::with_capacity(0),
            ..Default::default()
        }
    }

    pub fn error<E, S>(error: E, path: S) -> Self where E: Into<Error>, S: Into<String> {
        Self {
            result: None,
            success: false,
            path: Some(path.into()),
            timestamp: Some(chrono::offset::Utc::now().to_string()),
            errors: vec![error.into()],
            ..Default::default()
        }
    }

    pub fn errors<E, I, S>(errors: I, path: S) -> Self where I: IntoIterator<Item = E>, E: Into<Error>, S: Into<String> {
        Self {
            result: None,
            success: false,
            path: Some(path.into()),
            timestamp: Some(chrono::offset::Utc::now().to_string()),
            errors: errors.into_iter().map(|e| e.into()).collect(),
            ..Default::default()
        }
    }
    
    pub fn push_err<E>(mut self, error: E) -> Self where E: Into<Error> {
        self.errors.push(error.into());
        self
    }

    pub fn to_axum_json(&self) -> axum::Json<serde_json::Value> where T: Serialize {
        axum::Json(serde_json::to_value(&self).unwrap())
    }
}


pub struct Json<T>(pub T);

pub trait TryFromJson: Sized {
    fn try_from(map: HashMap<String, Value>) -> Result<Self, Vec<Error>>;

    fn check_keys<'a, T>(map: &HashMap<String, Value>, fields: T) -> Result<(), Vec<Error>>
        where T: IntoIterator<Item = &'a str> {
        let keys = HashSet::<&str>::from_iter(map.keys().map(|s| s.as_str()));
        let fields = HashSet::from_iter(fields); 
        let missing: Vec<String> = fields.difference(&keys).into_iter().map(|s| String::from(*s)).collect();
        let unknown: Vec<String> = keys.difference(&fields).into_iter().map(|s| String::from(*s)).collect();

        let mut errors = Vec::with_capacity(2);
        if !missing.is_empty() {
            errors.push(Error::from(JsonKeyError::Missing(missing)));
        }
        if !unknown.is_empty() {
            errors.push(Error::from(JsonKeyError::Unknown(unknown)));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("The request body is empty.")]
    IsEmpty,
    #[error("The request header content-type is empty.")]
    MissingContentType,
    #[error("The request header content-type: '{}' needs to be 'application/json'.", .0)]
    BadContentType(String),
    #[error("The request body is not a valid JSON object.")]
    BadFormat
}

#[derive(Debug, Error)]
pub enum JsonKeyError {
    #[error("Fields {} are missing from the JSON object.", .0.join(","))]
    Missing(Vec<String>),
    #[error("Fields '{}' are unknown, please remove them.", .0.join(","))]
    Unknown(Vec<String>)
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        let msg = error.to_string();
        match error {
            JsonError::IsEmpty => Error::new("JSON_REQ_BODY_IS_EMPTY", msg),
            JsonError::MissingContentType => Error::new("JSON_REQ_MISSING_CT", msg),
            JsonError::BadContentType(_) => Error::new("JSON_REQ_BAD_CT", msg),
            JsonError::BadFormat => Error::new("JSON_REQ_BAD_FORMAT", msg)
        }
    }
}

impl From<JsonKeyError> for Error {
    fn from(error: JsonKeyError) -> Self {
        let msg = error.to_string();
        match error {
            JsonKeyError::Missing(_) => Error::new("JSON_MISSING_KEYS", msg),
            JsonKeyError::Unknown(_) => Error::new("JSON_UNKNOWN_KEYS", msg)
        }
    }
}

impl<T, S> FromRequest<S> for Json<T> where S: Send + Sync, T: TryFromJson {
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let path = match req.extensions().get::<OriginalUri>() {
            Some(uri) => String::from(uri.path()),
            None => String::new()
        };
        req.headers().get(header::CONTENT_TYPE)
            .ok_or(JsonError::MissingContentType)
            .and_then(|ct| match is_json_content_type(ct) {
                true => Ok(()),
                false => Err(JsonError::BadContentType(String::from(ct.to_str().unwrap())))
            })
            .map_err(|e| {
                let response = Response::<()>::error(Error::from(e), &path);
                (StatusCode::OK, response.to_axum_json())
            })?;

        Bytes::from_request(req, state).await
            .map_err(|e| JsonError::IsEmpty)
            .and_then(|bytes| match serde_json::from_slice::<HashMap<String, Value>>(bytes.as_ref()) {
                Ok(map) => Ok(map),
                Err(error) => Err(JsonError::BadFormat)
            })
            .map_err(|e| {
                let response = Response::<()>::error(Error::from(e), &path);
                (StatusCode::OK, response.to_axum_json())
            })
            .and_then(|map| match T::try_from(map) {
                Ok(value) => Ok(Json(value)),
                Err(e) => Err((StatusCode::OK, Response::<()>::errors(e, &path).to_axum_json()))
            })



        // let content_type = match req.headers().get(header::CONTENT_TYPE) {
        //     Some(value) => value,
        //     None => {
        //         let response = Response::err(Error::from(JsonError::MissingContentType));
        //         return Err((StatusCode::OK, response.to_axum_json()));
        //     }
        // };

        // if !is_json_content_type(content_type) {
        //     let error = JsonError::BadContentType(String::from(content_type.to_str().unwrap()));
        //     let response = Response::err(Error::from(error));
        //     return Err((StatusCode::OK, response.to_axum_json()));
        // }

        // let bytes = match Bytes::from_request(req, state).await {
        //     Ok(bytes) => bytes,
        //     Err(error) => {
        //         let response = Response::err(Error::from(JsonError::IsEmpty));
        //         return Err((StatusCode::OK, response.to_axum_json()));
        //     }
        // };

        // let map: HashMap<String, Value> = match serde_json::from_slice(bytes.as_ref()) {
        //     Ok(map) => map,
        //     Err(error) => {
        //         let response = Response::err(Error::from(JsonError::BadFormat));
        //         return Err((StatusCode::OK, response.to_axum_json()));    
        //     }
        // };

        // match T::from_json(map) {
        //     Ok(value) => Ok(Response::ok(value).to_axum_json),
        //     Err(res) => Err((StatusCode::OK, res.to_json()))
        // }
    }
}



pub fn is_json_content_type(content_type: &axum::http::HeaderValue) -> bool { 
    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"));

    is_json_content_type
}