use std::sync::Arc;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;
// use crate::errors::Result;

// 封装`JSON`返回的数据
#[derive(Serialize)]
pub struct  Message {
    message:  String
}

//响应数据类型 OK Created JsonData 保存返回的数据局
pub enum ApiReponse {

    OK,
    Created,
    JsonData(Vec<Message>), //如果是响应数据的 json，则返回
}

pub enum ApiError {
    BadRequest,
    Forbidden
}


//我们需要实现axum::IntoReponse的trait来提供给axum的数据
impl IntoResponse for ApiReponse{
    fn into_response(self) -> Response {
        //将结果返回
        match self {
            ApiReponse::OK =>(StatusCode::OK).into_response(),
            ApiReponse::Created => (StatusCode::CREATED).into_response(),
            ApiReponse::JsonData(data) =>(StatusCode::OK,Json(data)).into_response()
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        debug!(" {:<12} {:?}","","ApiError");
        let mut response =  StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

/**

 返回
[
 {
"message":"DataFromApi"
}
]

//下面的ApiError需要实现这个错误IntoResponse才能形成一个可用的接口
 **/
  pub async  fn get_message()-> std::result::Result<ApiReponse,ApiError> {
     Ok(
         ApiReponse::JsonData(
             vec![
                 Message {
                     message: "DataFromAPi".to_owned(),
                 }
             ]
         )
     )
  }