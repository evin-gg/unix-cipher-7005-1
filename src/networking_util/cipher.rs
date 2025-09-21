use std::str;

pub fn split_payload(input: &[u8]) -> String {

    let input_str = str::from_utf8(input).unwrap();

    let mut parts = input_str.splitn(2, ' ');

    let cipher = parts.next().unwrap();
    let text = parts.next().unwrap();

    println!("Cipher: {}", cipher);
    println!("Text: {}", text); 

    let encoded_message = shift(text, cipher.parse::<i32>().unwrap());

    return encoded_message;
}

pub fn shift(text: &str, shift: i32) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c >= 'a' && c <= 'z' {
            let mut shifted_char = c as i32 - 'a' as i32 + shift;

            if shifted_char > 26 {
                shifted_char = shifted_char % 26;
                result.push(std::char::from_u32((shifted_char + 'a' as i32) as u32).unwrap());
            }

            else if shifted_char < 0 {
                shifted_char = (shifted_char * (-1) % 26) * (-1) + 26;
                result.push(std::char::from_u32((shifted_char + 'a' as i32) as u32).unwrap());
            }

            else {
                result.push(std::char::from_u32((shifted_char + 'a' as i32) as u32).unwrap());
            }
        }

        else if c >= 'A' && c <= 'Z' {
            let mut shifted_char = c as i32 - 'A' as i32 + shift;

            if shifted_char > 26 {
                shifted_char = shifted_char % 26;
                result.push(std::char::from_u32((shifted_char + 'A' as i32) as u32).unwrap());
            }

            else if shifted_char < 0 {
                shifted_char = (shifted_char * (-1) % 26) * (-1) + 26;
                result.push(std::char::from_u32((shifted_char + 'A' as i32) as u32).unwrap());
            }

            else {
                result.push(std::char::from_u32((shifted_char + 'A' as i32) as u32).unwrap());
            }
        }

        else {
            result.push(c);
        }
    }

    return result;
}