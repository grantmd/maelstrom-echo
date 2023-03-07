use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    msg_type: String,
    msg_id: u128,
    #[serde(default)]
    in_reply_to: u128,
    #[serde(default)]
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[tokio::main]
async fn  main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        eprint!("Received: {}", buffer);

        let mut msg: Message = serde_json::from_str(&buffer)?;
        if msg.body.msg_type != "echo" {
            eprintln!("Skipping");
            continue;
        }

        let src = msg.src;
        let dest = msg.dest;
        msg.src = dest;
        msg.dest = src;

        msg.body.msg_type = "echo_ok".to_string();
        msg.body.in_reply_to = msg.body.msg_id;

        let out = serde_json::to_string(&msg)?;
        eprintln!("Sending: {}", out);
        println!("{}", out);
    }
}
