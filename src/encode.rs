use crate::types::{Hex, Result};
use crate::traits::{Encode, Decode};
use crate::bitmap::{BitMap};
use std::collections::{BTreeMap};

type BitMapMapping = BTreeMap<char, BitMap>;

#[derive(Debug)]
pub struct Encoder { 
    data: String,
    bitmaps: BitMapMapping
}

impl Encoder {
    pub fn new(data: String) -> Self {
        let mut encoder = Encoder {
            data,
            bitmaps: BTreeMap::new()
        };
        encoder.initialize_bitmaps().unwrap();
        encoder
    }

    pub fn bitmaps(&self) -> &BitMapMapping {
        &self.bitmaps
    }

    fn initialize_bitmaps(&mut self) -> Result<()> {
        for c in self.data.chars() {
            self.bitmaps.entry(c).or_insert(BitMap::new(&self.data, c));
        }
        Ok(())
    }
}

impl Encode for Encoder {
    fn encode(&self) -> Result<Hex> {
        let mut encoded_bitmaps = String::new();
        for bitmap in self.bitmaps.values() {
            &encoded_bitmaps.push_str(&bitmap.encode()?);
        }
        let data_length = format!("{:x}", self.data.len());
        let data_length_length = format!("{:02x}", data_length.len());

        Ok(data_length_length + &data_length + &encoded_bitmaps)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_raw_data() -> String {
        String::from("aaa bbb ccc")
    }

    fn get_bitmaps() -> BitMapMapping {
        let data = get_raw_data();
        let mut hashmap = BTreeMap::new();

        hashmap.insert('a', BitMap::new(&data, 'a'));
        hashmap.insert('b', BitMap::new(&data, 'b'));
        hashmap.insert('c', BitMap::new(&data, 'c'));
        hashmap.insert(' ', BitMap::new(&data, ' '));

        hashmap
    }

    #[test]
    fn test_create_encoder() {
        let data = get_raw_data();
        let encoder = Encoder::new(String::from(&data));

        let bitmaps = get_bitmaps();

        for c in encoder.bitmaps().keys() {
            assert!(bitmaps.contains_key(&c));
            assert_eq!(encoder.bitmaps()[c] , bitmaps[c]);
        }
        
    }

    #[test]
    fn test_encode_data() {
        let data = get_raw_data();
        let encoder = Encoder::new(String::from(&data));

        let encoded_bitmap = encoder.encode().unwrap();
        let expected_encoding = "01b2011061e00620e063007";
        assert_eq!(encoded_bitmap, String::from(expected_encoding));

    }
}
