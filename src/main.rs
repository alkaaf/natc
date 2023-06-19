use nats::Connection;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!(r#"Usage: {} <nats url> "<topic>" [user] [password]"#, args[0]);
        return;
    }
    let nats_url = &args[1];
    let topic = &args[2];

    let mut use_user_pass = false;
    let user: &str;
    let pass: &str;
    if args.len() > 3 {
        use_user_pass = true;
    }
    let conn: Connection;

    if use_user_pass {
        user = &args[3];
        pass = &args[4];
        conn = nats::Options::with_user_pass(user, pass).connect(nats_url).unwrap();
    } else {
        conn = nats::connect(nats_url).unwrap();
    }

    let sub = conn.subscribe(topic).unwrap();
    unsafe {
        while let message = sub.next().unwrap() {
            let s = String::from_utf8_unchecked(message.data);
            println!("{} -> {}", message.subject, s);
        }
    }
}
