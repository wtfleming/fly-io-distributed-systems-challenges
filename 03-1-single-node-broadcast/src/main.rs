use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::io::Write;

// ----- Message Structs -----
#[derive(Serialize, Deserialize)]
struct RequestInitMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct RequestInitMessage {
    id: u32,
    src: String,
    dest: String,
    body: RequestInitMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyInitMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
struct ReplyInitMessage {
    src: String,
    dest: String,
    body: ReplyInitMessageBody,
}

// ----------- Broadcast messages
#[derive(Serialize, Deserialize)]
struct RequestBroadcastMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    message: i32,
    msg_id: u32,
}

#[derive(Serialize, Deserialize)]
struct RequestBroadcastMessage {
    src: String,
    dest: String,
    body: RequestBroadcastMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyBroadcastMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
struct ReplyBroadcastMessage {
    src: String,
    dest: String,
    body: ReplyBroadcastMessageBody,
}

// ----------- Topology messages
#[derive(Serialize, Deserialize)]
struct RequestTopologyMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
}

#[derive(Serialize, Deserialize)]
struct RequestTopologyMessage {
    src: String,
    dest: String,
    body: RequestTopologyMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyTopologyMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
struct ReplyTopologyMessage {
    src: String,
    dest: String,
    body: ReplyTopologyMessageBody,
}

// ----------- Read messages
#[derive(Serialize, Deserialize)]
struct RequestReadMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
}

#[derive(Serialize, Deserialize)]
struct RequestReadMessage {
    src: String,
    dest: String,
    body: RequestReadMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyReadMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    messages: Vec<i32>,
    msg_id: u32,
    in_reply_to: u32,
}

#[derive(Serialize, Deserialize)]
struct ReplyReadMessage {
    src: String,
    dest: String,
    body: ReplyReadMessageBody,
}

// ----- Reply handlers -----
// Reply to an init message
fn reply_init(
    node_id: &String,
    msg: &RequestInitMessage,
    next_msg_id: u32,
) -> serde_json::Result<()> {
    let reply = ReplyInitMessage {
        src: node_id.to_string(),
        dest: msg.src.to_string(),
        body: ReplyInitMessageBody {
            msg_id: next_msg_id,
            in_reply_to: msg.body.msg_id,
            kind: String::from("init_ok"),
        },
    };

    let j = serde_json::to_string(&reply)?;
    println!("{}", j);
    io::stdout().flush().unwrap();
    Ok(())
}

// Reply to a generate message
fn handle_broadcast_msg(
    node_id: &String,
    broadcast_msg: &RequestBroadcastMessage,
    next_msg_id: u32,
    broadcasts: &mut Vec<i32>,
) -> serde_json::Result<()> {
    let broadcast_reply_msg = ReplyBroadcastMessage {
        src: node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyBroadcastMessageBody {
            kind: String::from("broadcast_ok"),
            msg_id: next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };

    broadcasts.push(broadcast_msg.body.message);

    let j = serde_json::to_string(&broadcast_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn handle_topology_msg(
    node_id: &String,
    broadcast_msg: &RequestTopologyMessage,
    next_msg_id: u32,
) -> serde_json::Result<()> {
    let topology_reply_msg = ReplyTopologyMessage {
        src: node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyTopologyMessageBody {
            kind: String::from("topology_ok"),
            msg_id: next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };
    let j = serde_json::to_string(&topology_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn handle_read_msg(
    node_id: &String,
    broadcast_msg: &RequestReadMessage,
    next_msg_id: u32,
    broadcasts: &Vec<i32>,
) -> serde_json::Result<()> {
    let topology_reply_msg = ReplyReadMessage {
        src: node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyReadMessageBody {
            kind: String::from("read_ok"),
            //messages: [].to_vec(),
            messages: broadcasts.to_vec(),
            msg_id: next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };
    let j = serde_json::to_string(&topology_reply_msg)?;
    println!("{}", j);

    Ok(())
}


fn main() -> io::Result<()> {
    let mut next_msg_id = 0;
    let mut node_id: String = "".to_string();
    let mut broadcasts: Vec<i32> = Vec::new();

    for line in std::io::stdin().lines() {
        let buffer = line.unwrap();
        // Note: we deserialize buffer twice, could do it once using an enum instead?
        // Look at https://serde.rs/enum-representations.html
        let v: Value = serde_json::from_str(&buffer)?;

        let msg_type: &str = v["body"]["type"].as_str().unwrap();

        match msg_type {
            "init" => {
                let msg: RequestInitMessage = serde_json::from_str(&buffer)?;
                node_id = msg.dest.clone();
                reply_init(&node_id, &msg, next_msg_id)?;
                next_msg_id = next_msg_id + 1
            }
            "broadcast" => {
                let broadcast_msg: RequestBroadcastMessage = serde_json::from_str(&buffer)?;
                handle_broadcast_msg(&node_id, &broadcast_msg, next_msg_id, &mut broadcasts)?;
                next_msg_id = next_msg_id + 1;
            }
            "topology" => {
                let topology_msg: RequestTopologyMessage = serde_json::from_str(&buffer)?;
                handle_topology_msg(&node_id, &topology_msg, next_msg_id)?;
                next_msg_id = next_msg_id + 1;
            }
            "read" => {
                let read_msg: RequestReadMessage = serde_json::from_str(&buffer)?;
                handle_read_msg(&node_id, &read_msg, next_msg_id, &broadcasts)?;
                next_msg_id = next_msg_id + 1;
            }

            unhandled => {
                unimplemented!("msg type: {}", unhandled);
            }
        };
    }
    Ok(())
}
