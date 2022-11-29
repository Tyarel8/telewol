use wake_on_lan::MagicPacket;

pub fn parse_mac(mac_string: &str) -> Result<Vec<u8>, &'static str> {
    if mac_string.len() != 17 {
        return Err("Invalid MAC address");
    }

    let delimiter = if mac_string.contains('-') {
        '-'
    } else if mac_string.contains(':') {
        ':'
    } else {
        return Err("Invalid MAC address delimiter");
    };

    let mac_address: Vec<u8> = mac_string
        .split(delimiter)
        .flat_map(|hex| u8::from_str_radix(hex, 16))
        .collect::<Vec<u8>>();

    if mac_address.len() != 6 {
        Err("MAC address must be 6 bytes")
    } else {
        Ok(mac_address)
    }
}

pub fn unparse_mac(mac_address: Vec<u8>) -> String {
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac_address[0],
        mac_address[1],
        mac_address[2],
        mac_address[3],
        mac_address[4],
        mac_address[5]
    )
}

pub fn wake(mac_string: &str) -> Result<(), &'static str> {
    let mac_arr: [u8; 6] = parse_mac(mac_string)?.try_into().unwrap();

    let magic_packet = MagicPacket::new(&mac_arr);
    if magic_packet.send().is_err() {
        Err("Unable to send magic packet")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_mac_fail() {
        assert!(vec![
            "",
            "Potato",
            "00:FF:0p:0a:ff:ñ0",
            "FF:01:0a:ff:00",
            "00:FF:0e:0a:ff::00"
        ]
        .into_iter()
        .all(|mac| parse_mac(mac).is_err()));
    }

    #[test]
    fn test_parse_mac_success() {
        assert!(vec![
            "00:FF:0a:0a:ff:00",
            "FF:FF:ba:0a:ff:00",
            "00:FF:0a:fa:dd:00",
            "00:FF:0a:0a:ff:e0",
            "00-FF-0a-0a-ff-e0",
            "00:FF:0a:0a:ff:a0"
        ]
        .into_iter()
        .all(|mac| parse_mac(mac).is_ok()));
    }

    #[test]
    fn test_unparse_mac() {
        assert_eq!(
            unparse_mac(vec![0x00, 0xFF, 0x0a, 0x0a, 0xff, 0x00]),
            "00:FF:0A:0A:FF:00"
        );
    }
}
