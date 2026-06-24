pub mod ancora {
    include!(concat!(env!("OUT_DIR"), "/ancora.rs"));
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::ancora::{Ping, Pong};

    #[test]
    fn ping_round_trip() {
        let original = Ping {
            id: "test-ping-1".to_string(),
        };
        let encoded = original.encode_to_vec();
        let decoded = Ping::decode(encoded.as_slice()).expect("decode Ping");
        assert_eq!(original, decoded);
    }

    #[test]
    fn pong_round_trip() {
        let original = Pong {
            id: "test-pong-1".to_string(),
        };
        let encoded = original.encode_to_vec();
        let decoded = Pong::decode(encoded.as_slice()).expect("decode Pong");
        assert_eq!(original, decoded);
    }
}
