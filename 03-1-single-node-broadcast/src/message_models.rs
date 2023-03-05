use serde::{Deserialize, Serialize};

// ----- Message Structs -----
#[derive(Serialize, Deserialize)]
pub struct RequestInitMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestInitMessage {
    pub id: u32,
    pub src: String,
    pub dest: String,
    pub body: RequestInitMessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyInitMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
    pub in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyInitMessage {
    pub src: String,
    pub dest: String,
    pub body: ReplyInitMessageBody,
}

// ----------- Broadcast messages
#[derive(Serialize, Deserialize)]
pub struct RequestBroadcastMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub message: i32,
    pub msg_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBroadcastMessage {
    pub src: String,
    pub dest: String,
    pub body: RequestBroadcastMessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyBroadcastMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
    pub in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyBroadcastMessage {
    pub src: String,
    pub dest: String,
    pub body: ReplyBroadcastMessageBody,
}

// ----------- Topology messages
#[derive(Serialize, Deserialize)]
pub struct RequestTopologyMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RequestTopologyMessage {
    pub src: String,
    pub dest: String,
    pub body: RequestTopologyMessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyTopologyMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
    pub in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyTopologyMessage {
    pub src: String,
    pub dest: String,
    pub body: ReplyTopologyMessageBody,
}

// ----------- Read messages
#[derive(Serialize, Deserialize)]
pub struct RequestReadMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub msg_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RequestReadMessage {
    pub src: String,
    pub dest: String,
    pub body: RequestReadMessageBody,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyReadMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub messages: Vec<i32>,
    pub msg_id: u32,
    pub in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyReadMessage {
    pub src: String,
    pub dest: String,
    pub body: ReplyReadMessageBody,
}
