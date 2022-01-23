use didcomm_rs::{Message};

fn main() {
    let m = Message::new()
        .from("did:xyz:ulapcuhsatnpuhza930hpu34n_")
        .to(&["did:xyz:34r3cu403hnth03r49g03", "did:xyz:03489jnutnjqhiu0uh540u8hunoe"])
        .set_body("This is some payloads".as_bytes());

    let ready_to_send = m.as_raw_json().unwrap();
    // None 대신 secret key &[u8] 을 넘겨야 할 듯
    let received = Message::receive(&ready_to_send, None).unwrap();

    println!("Hello, I received = {}", received);
}
