use crate::encode::Encoder;
use crate::decode::Decoder;
use crate::traits::{Encode, Decode};

#[cfg(test)]
mod tests {
    use super::*;

    fn lorem_ipsum() -> String {
        "lorem ipsum dolor sit amet \
        , consectetur adipiscing elit, \
        sed do eiusmod tempor incididunt \
        ut labore et dolore magna aliqua. \
        Ut enim ad minim veniam, quis \
        nostrud exercitation ullamco \
        laboris nisi ut aliquip ex ea  \
        commodo consequat. Duis aute irure \
        dolor in reprehenderit in \
        voluptate velit esse cillum  \
        dolore eu fugiat nulla pariatur. \
        Excepteur sint occaecat cupidatat \
        non proident, sunt in culpa qui \
        officia deserunt mollit anim \
        id est laborum.".to_owned()
    }

    #[test]
    fn test_integration() {
        let input_data = lorem_ipsum();
            
        let encoder = Encoder::new(input_data.clone());
        let encoded_data = encoder.encode().unwrap();
        let mut  decoder = Decoder::new(&encoded_data).unwrap();
        let decoded_data = decoder.decode().unwrap();
        
        assert_eq!(decoded_data, input_data);
    }
}
