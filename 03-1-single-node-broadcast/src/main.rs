mod message_models;
use crate::message_models::*;

use serde_json::Value;
use std::io;

// Struct to store the app's state in
struct State {
    next_msg_id: u32,
    node_id: String, // TODO, this should probably be an Option<String>
    broadcasts: Vec<i32>,
}

// ----- Reply handlers -----
// Reply to an init message
fn handle_init(state: &State, msg: &RequestInitMessage) -> serde_json::Result<()> {
    let reply = ReplyInitMessage {
        src: state.node_id.to_string(),
        dest: msg.src.to_string(),
        body: ReplyInitMessageBody {
            msg_id: state.next_msg_id,
            in_reply_to: msg.body.msg_id,
            kind: String::from("init_ok"),
        },
    };

    let j = serde_json::to_string(&reply)?;
    println!("{}", j);

    Ok(())
}

// Reply to a generate message
fn handle_broadcast_msg(
    state: &mut State,
    broadcast_msg: &RequestBroadcastMessage,
) -> serde_json::Result<()> {
    let broadcast_reply_msg = ReplyBroadcastMessage {
        src: state.node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyBroadcastMessageBody {
            kind: String::from("broadcast_ok"),
            msg_id: state.next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };

    state.broadcasts.push(broadcast_msg.body.message);

    let j = serde_json::to_string(&broadcast_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn handle_topology_msg(
    state: &State,
    broadcast_msg: &RequestTopologyMessage,
) -> serde_json::Result<()> {
    let topology_reply_msg = ReplyTopologyMessage {
        src: state.node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyTopologyMessageBody {
            kind: String::from("topology_ok"),
            msg_id: state.next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };
    let j = serde_json::to_string(&topology_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn handle_read_msg(state: &State, broadcast_msg: &RequestReadMessage) -> serde_json::Result<()> {
    let topology_reply_msg = ReplyReadMessage {
        src: state.node_id.clone(),
        dest: broadcast_msg.src.to_string(),
        body: ReplyReadMessageBody {
            kind: String::from("read_ok"),
            messages: state.broadcasts.clone(),
            msg_id: state.next_msg_id,
            in_reply_to: broadcast_msg.body.msg_id,
        },
    };
    let j = serde_json::to_string(&topology_reply_msg)?;
    println!("{}", j);

    Ok(())
}

fn main() -> io::Result<()> {
    let mut state = State {
        next_msg_id: 0,
        node_id: String::from(""),
        broadcasts: Vec::new(),
    };

    for line in std::io::stdin().lines() {
        let buffer = line.unwrap();
        // Note: we deserialize buffer twice, could do it once using an enum instead?
        // Look at https://serde.rs/enum-representations.html
        let v: Value = serde_json::from_str(&buffer)?;

        let msg_type: &str = v["body"]["type"].as_str().unwrap();

        match msg_type {
            "init" => {
                let msg: RequestInitMessage = serde_json::from_str(&buffer)?;
                state.node_id = msg.dest.clone();
                handle_init(&state, &msg)?;
            }
            "broadcast" => {
                let broadcast_msg: RequestBroadcastMessage = serde_json::from_str(&buffer)?;
                handle_broadcast_msg(&mut state, &broadcast_msg)?;
            }
            "topology" => {
                let topology_msg: RequestTopologyMessage = serde_json::from_str(&buffer)?;
                handle_topology_msg(&state, &topology_msg)?;
            }
            "read" => {
                let read_msg: RequestReadMessage = serde_json::from_str(&buffer)?;
                handle_read_msg(&state, &read_msg)?;
            }
            unhandled => {
                unimplemented!("msg type: {}", unhandled);
            }
        };
        state.next_msg_id += 1;
    }
    Ok(())
}
