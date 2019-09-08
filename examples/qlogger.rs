

fn main() {

    //let pkt_sent_evt_ty = quiche::qlog::TransportEventType::PacketSent;
    //println!("{}", serde_json::to_string(&pkt_sent_evt_ty).unwrap());

    let pkt_sent_evt_0 = quiche::qlog::EventPacketSent {
        raw_encrypted: None,
        packet_type: quiche::qlog::PacketType::Initial,
        header: None,
        frames: None,
    };

    println!("packet 0: {}", serde_json::to_string(&pkt_sent_evt_0).unwrap());

    let pkt_hdr_1 = quiche::qlog::PacketHeader {
        packet_number: "0".to_string(),
        packet_size: Some(1251),
        payload_length: Some(1224),
        version: Some("0xff000016".to_string()),
        scil: Some("8".to_string()),
        dcil: Some("8".to_string()),
        scid: Some("7e37e4dcc6682da8".to_string()),
        dcid: Some("36ce104eee50101c".to_string())
    };

    let pkt_sent_evt_1 = quiche::qlog::EventPacketSent {
        raw_encrypted: None,
        packet_type: quiche::qlog::PacketType::Initial,
        header: Some(pkt_hdr_1),
        frames: None,
    };
    println!("packet 1: {}", serde_json::to_string(&pkt_sent_evt_1).unwrap());

    let pkt_hdr_2 = quiche::qlog::PacketHeader {
        packet_number: "0".to_string(),
        packet_size: Some(1251),
        payload_length: Some(1224),
        version: Some("0xff000016".to_string()),
        scil: None,
        dcil: None,
        scid: None,
        dcid: None
    };

    let pkt_sent_evt_2 = quiche::qlog::EventPacketSent {
        raw_encrypted: None,
        packet_type: quiche::qlog::PacketType::Initial,
        header: Some(pkt_hdr_2),
        frames: None,
    };
    println!("packet 2: {}", serde_json::to_string(&pkt_sent_evt_2).unwrap());
}