use crate::traits::Decode;
use crate::types::{Hex, Result};


#[derive(PartialEq, Debug)]
pub struct Decoder {
    data_length: usize,
    data: Hex
}

impl Decoder {
    pub fn new(data: &str) -> Result<Self> {
        let hex_data = String::from(data);
        let data_length_length = u8::from_str_radix(&hex_data[0..2], 16)?;
        let mut data_length = String::new();
        for (i, ch) in hex_data[2..].char_indices() {
            if i == (data_length_length).into() {
                break;
            }
            data_length.push(ch);
        }
        let data = hex_data.split_at(data_length.len() + 2).1.into();

        Ok(Decoder {
            data_length: usize::from_str_radix(&data_length, 16)?,
            data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_decoder() {
        let hex_data = "04a32dbbbbbbbb";
        let expected_decoder = Decoder {
            data_length: 41773,
            data: String::from("bbbbbbbb")
        };
        let decoder = Decoder::new(hex_data).unwrap();
        assert_eq!(decoder, expected_decoder);
    }
}