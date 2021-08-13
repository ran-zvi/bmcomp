pub mod bitmap;
pub mod lib;
pub mod types;
pub mod traits;
pub mod encode;
pub mod decode;

fn main() {

    let mut map = bitmap::BitMap::new("aaa bbb ccc", 'a').to_hex();
    
}
