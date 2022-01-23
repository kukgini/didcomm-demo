use didcomm_rs::{Message};

fn main() {
    let sk = vec![];
    let m = Message::new()
        .from("did:xyz:ulapcuhsatnpuhza930hpu34n_")
        .to(&["did:xyz:34r3cu403hnth03r49g03", "did:xyz:03489jnutnjqhiu0uh540u8hunoe"])
        .set_body("This is some payloads".as_bytes());

    let ready_to_send = m.as_raw_json().unwrap();
    let received = Message::receive(&ready_to_send, &sk).unwrap();

    println!("Hello, I received = {:?}", received);
}
