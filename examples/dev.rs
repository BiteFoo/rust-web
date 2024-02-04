use axum::routing::get;
use tokio;


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{

    // println!("> dev");
    let client  = httpc_test::new_client("http://127.0.0.1:8900")?;
    let hello_world = client.do_get("/");
    hello_world.await?.print().await?;
    //尝试读取 josn
    let get_json = client.do_get("/get_json");
    get_json.await?.print().await?;

    // 尝试获取get_message
    let get_message = client.do_get("/get_message");
    get_message.await?.print().await?;
    Ok(())
}

