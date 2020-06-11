use std::str;

static DICT: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

static MASK: u8 = (1<<6) - 1;

pub fn encode(input: &String) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();

    let mut result: Vec<u8> = Vec::new();
    let transform = |bytes: Vec<u8>| -> Vec<u8> {
        vec![
            DICT[(bytes[0]>>2) as usize] as u8,
            DICT[(((bytes[0]<<4) & MASK) | (bytes[1]>>4)) as usize] as u8,
            DICT[(((bytes[1]<<2) & MASK) | (bytes[2]>>6)) as usize] as u8,
            DICT[(bytes[2] & MASK) as usize] as u8
        ]
    };
    for i in 0..(len / 3) {
        let index = i * 3;
        let mut group = transform(vec![bytes[index], bytes[index + 1], bytes[index + 2]]);
        result.append(&mut group);
    }

    match len % 3 {
        1 => {
            result.push(DICT[(bytes[len - 1]>>2) as usize] as u8);
            result.push(DICT[(bytes[len - 1]<<4 & MASK) as usize] as u8);
            result.push('=' as u8);
            result.push('=' as u8);
        },
        2 => {
            result.push(DICT[(bytes[len - 2]>>2) as usize] as u8);
            result.push(DICT[(((bytes[len - 2]<<4) & MASK) | (bytes[len - 1]>>4)) as usize] as u8);
            result.push(DICT[((bytes[len - 1]<<2) & MASK) as usize] as u8);
            result.push('=' as u8);
        },
        _ => {

        }
    }

    let result = match str::from_utf8(result.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    String::from(result)
}


#[cfg(test)]
mod tests {
    #[test]
    fn encode_test() {
        assert_eq!(super::encode(&"Man".to_string()), "TWFu".to_string());
        assert_eq!(super::encode(&"A".to_string()), "QQ==".to_string());
        assert_eq!(super::encode(&"BC".to_string()), "QkM=".to_string());
    }
}