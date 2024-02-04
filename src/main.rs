mod errors;
mod mresponse;
mod uploader;
mod storage;


use axum::{self, Json, Router};
use axum::extract::DefaultBodyLimit;
use axum::routing::{get,post};
use tokio;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing::debug;

use self::errors::Result; //自己定义的错误类型，方便处理不同的错误

#[tokio::main]
async fn main()->Result<()> {
    // println!("Hello, world!");
    //用来支持日志显示，例如通过配置在.cargo/config.toml中的配置来确定
    tracing_subscriber::fmt()
        .without_time() // 关闭不需要时间格式
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Rust-Web-server");
    //首先创建一个简单的hello,world
    let router = init_router();
    let listener = TcpListener::bind("127.0.0.1:8900").await.unwrap();
    info!("{:<12} - {:?}\n"," LISTENING",listener.local_addr());
    //启动 --works for v0.7.4 not v0.6
    axum::serve(listener,router.into_make_service()).await.unwrap();

    Ok(())
}


fn init_router()->Router{
    Router::new()
        .route("/",get(hello_world))
        .route("/get_json",get(get_json))
        .route("/get_message",get(mresponse::get_message))
        //给每个route添加layer来控制，例如下面的用于更改默认的值大小为 500mb
        .route("/upload",post(uploader::upload)).layer(DefaultBodyLimit::max(1024*500*1024))
}
async fn hello_world() -> &'static str{
    debug!("call hello,world");
    "Hello,world"
}

async fn get_json()->Json<Vec<String>>{
    Json(vec!["axum-framework".to_owned(),"json_api".to_owned()])
}



