use crate::lib::trim_binary_number;
use crate::traits::Encode;
use crate::types::{BitMapError, Hex, Result};

#[derive(PartialEq, Debug)]
pub struct BitMap {
    map: Vec<u8>,
    character: char,
}

impl BitMap {
    pub fn new(s: &str, character: char) -> Self {
        let map = s
            .chars()
            .map(|c| match c {
                _ if c == character => 1,
                _ => 0,
            })
            .collect();
        BitMap { map, character }
    }

    pub fn map(&self) -> &Vec<u8> {
        &self.map
    }

    pub fn to_hex(&self) -> Hex {
        self.map
            .chunks(4)
            .map(|chunk| chunk.iter().fold(0, |acc, &b| (acc << 1) | b))
            .map(|i| format!("{:x}", i))
            .collect()
    }

    pub fn from_str(s: &str, expected_length: usize) -> Result<Self> {
        let s_len = s.len();
        if s_len < 3 {
            return Err(BitMapError::Decode("String too short".into()).into());
        }
        let character = char::from_u32(u32::from_str_radix(&s[0..2], 16)?).unwrap();
        let mut map = &s[2..s_len as usize]
            .chars()
            .map(|h| crate::lib::to_binary(h))
            .collect::<String>()
            .chars()
            .map(|n| n.to_digit(2).unwrap() as u8)
            .collect::<Vec<u8>>();
        let remainder = expected_length % 4;
        let map = match remainder {
            0 => map.to_owned(),
            _ => {
                let bits_to_remove = 4 - remainder as u8;
                let last_4_bits = {
                    if expected_length >= 4 {
                        map[(map.len() - 4)..].to_vec()
                    } else {
                        map.to_vec()
                    }
                };
                let last_4_bits = &trim_binary_number(&last_4_bits, bits_to_remove);
                if expected_length < 4 {
                    last_4_bits.to_vec()
                } else {
                    map[..map.len() - 4]
                        .to_vec()
                        .iter()
                        .chain(last_4_bits)
                        .cloned()
                        .collect::<Vec<_>>()
                }
            }
        };

        Ok(BitMap {
            map: map,
            character,
        })
    }

    pub fn decode(&self) -> Result<String> {
        let as_str = self.map.clone();
        let as_str = as_str
            .into_iter()
            .map(|b| format!("{}", b))
            .collect::<String>()
            .chars()
            .map(|c| match c {
                '1' => self.character,
                _ => '0',
            })
            .collect();
        Ok(as_str)
    }
}

impl Encode for BitMap {
    fn encode(&self) -> Result<Hex> {
        let hex = self.to_hex();
        let character = format!("{:02x}", self.character as u8);
        Ok(character + &hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_bitmap() {
        let s = "all around antarctica";
        let bitmap = BitMap::new(&s, 'a');
        assert_eq!(
            *bitmap.map(),
            vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_create_bitmap_all_characters() {
        let s = "aaaa";
        let bitmap = BitMap::new(&s, 'a');
        assert_eq!(*bitmap.map(), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_create_bitmap_no_characters() {
        let s = "five";
        let bitmap = BitMap::new(&s, 'a');
        assert_eq!(*bitmap.map(), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_bitmap_to_hex() {
        let bitmap = BitMap::new("abbabaaabbbaaaaa", 'a');
        let hex = bitmap.to_hex();
        assert_eq!(hex, String::from("971f"));
    }

    #[test]
    fn test_bitmap_encode() {
        let bitmap = BitMap::new("abbabaaabbbaaaaa", 'a');
        let encoding = bitmap.encode().unwrap();
        assert_eq!(encoding, String::from("61971f"));
    }

    #[test]
    fn test_bitmap_from_str() {
        let s = "61971f";
        let bitmap = BitMap::from_str(&s, 16).unwrap();
        let expected_bitmap = BitMap::new("abbabaaabbbaaaaa", 'a');

        assert_eq!(bitmap, expected_bitmap);
    }

    #[test]
    fn test_bitmap_from_str_variant_size() {
        let s = "61971e";
        let bitmap = BitMap::from_str(&s, 15).unwrap();
        let expected = BitMap::new("abbabaaabbbaaaa", 'a');

        assert_eq!(bitmap, expected);
    }

    #[test]
    fn test_bitmap_from_str_variant_size_2() {
        let s = "63007";
        let bitmap = BitMap::from_str(&s, 11).unwrap();
        let expected = BitMap::new("aaa bbb ccc", 'c');

        assert_eq!(bitmap, expected);
    }

    #[test]
    fn test_bitmap_from_str_variant_size_3() {
        let s = "63003";
        let bitmap = BitMap::from_str(&s, 10).unwrap();
        let expected = BitMap::new("aaa bbb cc", 'c');

        assert_eq!(bitmap, expected);
    }

    #[test]
    fn test_bitmap_from_str_variant_size_4() {
        let s = "63001";
        let bitmap = BitMap::from_str(&s, 9).unwrap();
        let expected = BitMap::new("aaa bbb c", 'c');

        assert_eq!(bitmap, expected);
    }

    #[test]
    fn test_bitmap_decode() {
        let s = "61971e";
        let bitmap = BitMap::from_str(&s, 15).unwrap();
        let expected = String::from("a00a0aaa000aaaa");

        assert_eq!(bitmap.decode().unwrap(), expected);
    }

    #[test]
    #[should_panic(expected = "String too short")]
    fn test_bitmap_from_str_panic_too_short() {
        let s = "61";
        BitMap::from_str(&s, 5).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn test_bitmap_from_str_panic_invalid_digit() {
        let s = "jj0001";
        BitMap::from_str(&s, 10).unwrap();
    }
}
