use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::io::Write;

// https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md
// At the start of a test, Maelstrom issues a single init message to each node, like so:
// {
//   "type":     "init",
//   "msg_id":   1,
//   "node_id":  "n3",
//   "node_ids": ["n1", "n2", "n3"]
// }
// The node_id field indicates the ID of the node which is receiving this message: here, the node ID is "n1". Your node should remember this ID and include it as the src of any message it sends.

// The node_ids field lists all nodes in the cluster, including the recipient. All nodes receive an identical list; you may use its order if you like

// In this challenge, your node will receive an "echo" message from Maelstrom that looks like this:
// {
//   "src": "c1",
//   "dest": "n1",
//   "body": {
//     "type": "echo",
//     "msg_id": 1,
//     "echo": "Please echo 35"
//   }
// }

// Your job is to send a message with the same body back to the client but with a message type of "echo_ok". It should also associate itself with the original message by setting the "in_reply_to" field to the original message ID.
// It should look something like:
// {
//   "src": "n1",
//   "dest": "c1",
//   "body": {
//     "type": "echo_ok",
//     "msg_id": 1,
//     "in_reply_to": 1,
//     "echo": "Please echo 35"
//   }
// }

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
struct RequestEchoMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct RequestEchoMessage {
    src: String,
    dest: String,
    body: RequestEchoMessageBody,
}

#[derive(Serialize, Deserialize)]
struct ReplyEchoMessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    msg_id: u32,
    in_reply_to: u32,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct ReplyEchoMessage {
    src: String,
    dest: String,
    body: ReplyEchoMessageBody,
}

// ----- Reply handlers -----
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

fn reply_echo(
    node_id: &String,
    echo_msg: &RequestEchoMessage,
    next_msg_id: u32,
) -> serde_json::Result<()> {
    let echo_reply_msg = ReplyEchoMessage {
        src: node_id.clone(),
        dest: echo_msg.src.to_string(),
        body: ReplyEchoMessageBody {
            kind: String::from("echo_ok"),
            msg_id: next_msg_id,
            in_reply_to: echo_msg.body.msg_id,
            echo: echo_msg.body.echo.to_string(),
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
            let echo_msg: RequestEchoMessage = serde_json::from_str(&buffer)?;
            reply_echo(&node_id, &echo_msg, next_msg_id)?;
            next_msg_id = next_msg_id + 1;
        }
    }
    Ok(())
}
