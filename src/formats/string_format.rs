#[path = "../format.rs"]
mod format;

use format::Format;

pub struct StringFormat;

impl Format<String> for StringFormat {

    fn deserialize(&mut self, input: Vec<u8>, defaults: Option<&String>) -> String {
        if input.len() > 0 {
            match String::from_utf8(input) {
                Ok(__input) => __input,
                Err(err) => panic!(err)
            }
        } else {
            match defaults {
                Some(__defaults) => __defaults.clone(),
                None => String::new()
            }
        }
    }

    fn serialize(&mut self, input: Option<&String>) -> Vec<u8> {
        match input {
            Some(__input) => __input.as_bytes().to_vec(),
            None => Vec::new()
        }
    }

}

impl StringFormat {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize_bytes_to_string() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.deserialize(s.as_bytes().to_vec(), None), s);
    }

    #[test]
    fn serialize_string_to_bytes() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.serialize(Some(&s)), s.as_bytes().to_vec());
    }

}