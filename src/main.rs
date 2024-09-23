use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::Message;

const SUPABASE_URL: &str = "sotltzbimtxdcufpqxrq.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InNvdGx0emJpbXR4ZGN1ZnBxeHJxIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjY0OTQ1MzYsImV4cCI6MjA0MjA3MDUzNn0.uXx-ALpUNSvwdhmz7-9VbqiRL6OkKGuFWLxDxCt-GNE";

#[tokio::main]
async fn main() {
    let supabase_wss_url =
        format!("wss://{SUPABASE_URL}/realtime/v1/websocket?apikey={SUPABASE_ANON_KEY}");
    let url = supabase_wss_url.into_client_request().unwrap();
    let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let table_name = "test";
    let join_message = json!({
        "topic" : format!("realtime:public:{table_name}:node_id=eq.1"),
        "event": "phx_join",
        "payload": {},
        "ref":"1"
    });

    write
        .send(Message::Text(join_message.to_string()))
        .await
        .expect("Failed to send join message");

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    println!(
                        "Received JSON: {}",
                        serde_json::to_string_pretty(&json).unwrap()
                    );
                    // println!("Received JSON: {:?}", json);
                } else {
                    println!("Received: {:?}", text);
                }
            }
            Ok(Message::Ping(ping)) => {
                write
                    .send(Message::Pong(ping))
                    .await
                    .expect("Failed to send pong");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
            _ => {
                println!("Received non-text message {:?}", message);
            }
        }
    }
}
