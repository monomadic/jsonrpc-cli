use serde::{Deserialize, Serialize};

/// Perform an RPC socket request
pub(crate) fn socket_request(
    host: &str,
    request: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use websocket::{ClientBuilder, Message};

    // println!("request -> {}", request);

    let mut client = ClientBuilder::new(host)?.connect_insecure()?;
    client.send_message(&Message::text(request))?; // Send message

    if let websocket::OwnedMessage::Text(response) = client.recv_message()? {
        // println!("response -> {}", response);
        Ok(response)
    } else {
        // todo: Err here
        Ok("{}".into())
    }
}

pub(crate) fn rpc_request(
    host: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let json_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": "1",
    })
    .to_string();

    socket_request(host, &json_request)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct RPCResponse<T> {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub error: Option<RPCErrorCode>,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct RPCErrorCode {
    pub code: i32,
    pub message: String,
    pub data: Option<String>,
}
