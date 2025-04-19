use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonIn{
    message: String,
}

#[derive(Serialize)]
pub struct JsonOut{
    message: String,
    message_From_Server : String,
}


pub async fn hello_world()-> String {
    "Hello world ehhe".to_string()
}

pub async fn data_store(Json(body): Json<JsonIn>) -> Json<JsonOut>{
    Json(JsonOut { message: body.message , message_From_Server: "hello from Axum".to_string() })
}
