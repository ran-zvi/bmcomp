use crate::bitmap::BitMap;
use crate::traits::Decode;
use crate::types::{Hex, Result};

#[derive(PartialEq, Debug)]
pub struct Decoder {
    data_length: usize,
    data: Hex,
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
            data,
        })
    }

    fn decode_into_bitmaps(&self, encoded_data_length: usize) -> Result<Vec<BitMap>> {
        Ok(self.data
            .as_bytes()
            .chunks(encoded_data_length)
            .map(|chunk| {
                let slice = String::from_utf8(chunk.to_vec()).unwrap();
                BitMap::from_str(&slice, self.data_length).unwrap()
            })
            .collect())
    }
}

impl Decode for Decoder {
    fn decode(&mut self) -> Result<String> {
        let encoded_data_length = ((self.data_length as f64 / 4f64).ceil() + 2f64) as usize;
        let decoded_bitmaps = self.decode_into_bitmaps(encoded_data_length)?;

        let mut decoded_data = (0..self.data_length).map(|_| '0').collect::<String>();
        for bitmap in decoded_bitmaps.iter() {
            let decoded_bitmap = bitmap.decode()?;
            decoded_data = decoded_data
            .chars()
            .zip(decoded_bitmap.chars())
            .map(|(c1, c2)| match c1 {
                '0' => c2,
                _ => c1
            })
            .collect();
        }

        Ok(decoded_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_decoder() {
        let hex_data = "04a32d62bbbbbbbb";
        let expected_decoder = Decoder {
            data_length: 41773,
            data: String::from("62bbbbbbbb"),
        };
        let decoder = Decoder::new(hex_data).unwrap();
        assert_eq!(decoder, expected_decoder);
    }

    #[test]
    fn test_decode() {
        let hex_data = "01b2011061e00620e063007";
        let expected_result = "aaa bbb ccc";

        let decoded_data = Decoder::new(hex_data).unwrap().decode().unwrap();
        assert_eq!(decoded_data, expected_result);
    }
}
