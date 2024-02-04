//上传文件

use std::collections::HashMap;
use axum::Json;
// use crate::errors::Result;
use  std::result::Result;
use axum::extract::Multipart;

use axum::http::StatusCode;
use tokio::io::AsyncWriteExt;
use tracing::info;
use tracing::log::debug;
use crate::storage::LocalStorage;


// 在依赖中还需要增加 features = ["multipart"]才能使用这个multipart功能
pub async  fn upload(mut files: Multipart) ->Result<Json<serde_json::Value>,(StatusCode,Json<serde_json::Value>)>{


    let mut result = HashMap::new();

        //保存数据
        while let Some(mut upload_file) = files.next_field().await.unwrap(){
            let name = upload_file.file_name().unwrap();
            // upload_file.file_name()
            info!("{:<12} {name:?}","Uploading ");
            //创建一个buffer来读取是最好的方案
            let local_storage = LocalStorage::new(name);
            let save =  local_storage.create_save_file().await.unwrap();// 无所谓，错误的话？，这里的错误类型不同，导致无法使用?来操作
            //创建文件
            debug!("create save file at {}",&save);
            let mut file = tokio::fs::File::create(save).await.unwrap();
            //写入大文件
            while let Some(chunk) = upload_file.chunk().await.unwrap() {
                file.write_all(&chunk).await.unwrap();
            }

        }

    result.insert("code",1);


    Ok(Json(serde_json::json!(result)))

}