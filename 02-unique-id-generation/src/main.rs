use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::io::Write;
use uuid::Uuid;

// Challenge #2: Unique ID Generation
// In this challenge, you'll need to implement a globally-unique ID generation system that runs against Maelstrom's unique-ids workload. Your service should be totally available, meaning that it can continue to operate even in the face of network partitions.

// Specification
// RPC: generate
// Your node will receive a request message body that looks like this:
// {
//   "type": "generate"
// }
// and it will need to return a "generate_ok" message with a unique ID:
// {
//   "type": "generate_ok",
//   "id": 123
// }
// IDs may be of any type--strings, booleans, integers, floats, arrays, etc.

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

#[derive(Serialize, Deserialize)]
struct RequestGenerateMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
}

#[derive(Serialize, Deserialize)]
struct RequestGenerateMessage {
    src: String,
    dest: String,
    body: RequestGenerateMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyGenerateMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    in_reply_to: u32,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct ReplyGenerateMessage {
    src: String,
    dest: String,
    body: ReplyGenerateMessageBody,
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
fn reply_generate(
    node_id: &String,
    echo_msg: &RequestGenerateMessage,
    next_msg_id: u32,
) -> serde_json::Result<()> {
    let echo_reply_msg = ReplyGenerateMessage {
        src: node_id.clone(),
        dest: echo_msg.src.to_string(),
        body: ReplyGenerateMessageBody {
            kind: String::from("generate_ok"),
            msg_id: next_msg_id,
            in_reply_to: echo_msg.body.msg_id,

            // Technically a v4 UUID is not guaranteed to be unique
            // but for now this is probably good enough?
            // Probably should instead use something like a
            // https://en.wikipedia.org/wiki/Snowflake_ID
            id: Uuid::new_v4().to_string(),
        },
    };
    let j = serde_json::to_string(&echo_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn main() -> io::Result<()> {
    let mut next_msg_id = 0;
    let mut node_id: String = "".to_string();

    for line in std::io::stdin().lines() {
        let buffer = line.unwrap();
        // Note: we deserialize buffer twice, could do it once using an enum instead?
        // Look at https://serde.rs/enum-representations.html
        let v: Value = serde_json::from_str(&buffer)?;

        if v["body"]["type"] == "init" {
            let msg: RequestInitMessage = serde_json::from_str(&buffer)?;
            node_id = msg.dest.clone();
            reply_init(&node_id, &msg, next_msg_id)?;
            next_msg_id = next_msg_id + 1;
        } else {
            let echo_msg: RequestGenerateMessage = serde_json::from_str(&buffer)?;
            reply_generate(&node_id, &echo_msg, next_msg_id)?;
            next_msg_id = next_msg_id + 1;
        }
    }
    Ok(())
}
