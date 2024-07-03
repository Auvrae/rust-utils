
pub fn insert_utf8_char(string: &str, idx: usize, c: char) -> String {
    let mut chars = string.chars().into_iter().collect::<Vec<char>>();
    chars.insert(idx, c);

    chars.iter().collect::<String>()
}

pub fn get_utf8_slice(string: &str, start: usize, end: usize, inclusive: bool) -> String {
    if end < start { return String::new() };
    //if start == end { return String::new() };
    if end > string.chars().count() { return String::new() };
    if inclusive == true {
        return string.chars().collect::<Vec<char>>()[start..=end].iter().cloned().collect::<String>();
    } else {
        return string.chars().collect::<Vec<char>>()[start..end].iter().cloned().collect::<String>();
    }
}

pub fn split_uft8(string: &str, split: usize) -> (String, String) {
    (get_utf8_slice(&string, 0, split, false), get_utf8_slice(&string, split, string.chars().count(), false))
}

/// Random String Generator (Useful for random IDs and other things). 
/// DO NOT USE FOR PASSWORDS.
/// 
/// Example: 
/// ```
/// use utils::create_random_string;
/// println!("{}", create_random_string(12));
/// // => y4ewqfF2xxW0
/// ```
pub fn create_random_string(length: u32) -> String {
    use rand::*;
    const CHARS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 
    'g', 'h', 'i', 'j', 'k', 'l', 
    'm', 'n', 'o', 'p', 'q', 'r', 
    's', 't', 'u', 'v', 'w', 'x', 
    'y', 'z', '0', '1', '2', '3', 
    '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 
    'G', 'H', 'I', 'J', 'K', 'L', 
    'M', 'N', 'I', 'P', 'Q', 'R', 
    'S', 'T', 'U', 'V', 'W', 'X', 
    'Y', 'Z'
    ];
    let mut id: String = String::new();
    for _ in 1..=length {
        //------------ Chars Length----------Create number between 0 <-> 1-----------------------Convert to String-
        //---------------------------Multiply Length x 0.XXX---------------Closest Int v-------Index--------------
        id += &CHARS[((CHARS.len() as f32 * rand::thread_rng().gen::<f32>()).floor() as u8) as usize].to_string();
    }
    return id
}


pub fn create_random_hex_string(length: u32) -> String {
    use rand::*;
    const CHARS: [char; 16] = [
        'a', 'b', 'c', 'd', 'e', 'f', 
        '0', '1', '2', '3', '4', '5', 
        '6', '7', '8', '9'
    ];
    let mut id: String = String::new();
    for _ in 1..=length {
        //------------ Chars Length----------Create number between 0 <-> 1-----------------------Convert to String-
        //---------------------------Multiply Length x 0.XXX---------------Closest Int v-------Index--------------
        id += &CHARS[((CHARS.len() as f32 * rand::thread_rng().gen::<f32>()).floor() as u8) as usize].to_string();
    }
    return id
}

/// Creates a UUIDv4 using the RFC-4122 v4 specification.
/// UUIDv4 is a completely random string of characters using 0-9 a-f. 
pub fn create_uuidv4() -> String {
    use rand::*;
    const CHARS: [char; 16] = [
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', 
        '2', '3', '4', '5', '6', '7', '8', '9'
    ];
    let mut uuid: String = String::new();
    //let time: u128 = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
    for index in 1..=36 {
        if index == 9 || index == 14 || index == 19 || index == 24 {
            uuid.push('-');
        } else {
            let c: char = CHARS[((CHARS.len() as f32 * rand::thread_rng().gen::<f32>()).floor() as u8) as usize];
            if index == 15 {
                uuid.push('4');
            } else {
                uuid.push(c);
            }
        }
    }
    return uuid;
}

/// Creates a UUIDv7 using the RFC-4122 v7 specification.
/// UUIDv7 uses the first 48bits of the UNIX-EPOCH in Milliseconds converted to hex.
pub fn create_uuidv7() -> String {
    use rand::*;
    use std::io::Read;
    use std::time;
    const CHARS: [char; 16] = [
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', 
        '2', '3', '4', '5', '6', '7', '8', '9'
    ];
    let mut buffer: [u8; 8] = [0,0,0,0,0,0,0,0];
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_le_bytes()
        .take(16)
        .read(&mut buffer)
        .unwrap();
    let mut uuid: String;
    uuid = format!("0{:x}", u64::from_le_bytes(buffer));
    uuid.insert(8, '-');
    for index in 14..=36 {
        if index == 14 || index == 19 || index == 24 {
            uuid.push('-');
        } else {
            let c: char = CHARS[((CHARS.len() as f32 * rand::thread_rng().gen::<f32>()).floor() as u8) as usize];
            if index == 15 {
                uuid.push('7');
            } else {
                uuid.push(c);
            }
        }
    }
    return uuid;
}

pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}
