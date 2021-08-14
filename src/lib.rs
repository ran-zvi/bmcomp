pub fn to_binary(c: char) -> &'static str {
    match c.to_uppercase().next().unwrap() {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

pub fn trim_binary_number(bin: &Vec<u8>, n_bits_to_trim: u8) -> Vec<u8> {
    let beginning_length = bin.len();

    let bin = trim_binary_number_left(&bin, n_bits_to_trim);
    if bin.len() < beginning_length {
        bin
    }
    else {
        trim_binary_number_right(&bin, n_bits_to_trim)
    }
}

fn trim_binary_number_left(bin: &Vec<u8>, n_bits_to_trim: u8) -> Vec<u8> {
    let bin_copy = bin.clone();
    let mut temp  = vec![];
    let mut previous = 0;
    let mut i = 0;
    let mut bin_iter = bin_copy.iter();
    while i < bin_copy.len() {
        let value = *bin_iter.next().unwrap();
        if value == 0 && previous == 0 && i < n_bits_to_trim as usize {
            i+=1;
            continue;
        }
        temp.push(value);
        previous = value;
        i+=1;
    }
    temp
}

fn trim_binary_number_right(bin: &Vec<u8>, n_bits_to_trim: u8) -> Vec<u8> {
    let mut bin_copy = bin.clone();
    bin_copy.reverse();
    bin_copy = trim_binary_number_left(&bin_copy, n_bits_to_trim);
    bin_copy.reverse();
    bin_copy
}

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;
    
    macro_rules! trim_test {
        ($($name:ident:  $value:expr, $func:ident),*) => {
            $(
                #[test]
                fn $name() {
                    let (input, bits, expected) = $value;
                    assert_eq!($func(input, bits), expected);
                }
            )*
        };
    }

    trim_test!{
        test_trim_left_1: (&vec![1,1,1,1], 0, vec![1,1,1,1]), trim_binary_number_left,
        test_trim_left_2: (&vec![0,1,1,1], 1, vec![1,1,1]), trim_binary_number_left,
        test_trim_left_3: (&vec![0,0,1,1], 2, vec![1,1]), trim_binary_number_left,
        test_trim_left_4: (&vec![0,0,0,1], 3, vec![1]), trim_binary_number_left,
        test_trim_left_5: (&vec![0,0,0,0], 4, vec![]), trim_binary_number_left,
        test_trim_left_6: (&vec![1,0,0,1], 1, vec![1,0,0,1]), trim_binary_number_left,
        test_trim_left_7: (&vec![0,1,0,0], 3, vec![1,0,0]), trim_binary_number_left,

        test_trim_right_1: (&vec![1,1,1,1], 0, vec![1,1,1,1]), trim_binary_number_right,
        test_trim_right_2: (&vec![1,1,1,0], 1, vec![1,1,1]), trim_binary_number_right,
        test_trim_right_3: (&vec![1,1,0,0], 2, vec![1,1]), trim_binary_number_right,
        test_trim_right_4: (&vec![1,0,0,0], 3, vec![1]), trim_binary_number_right,
        test_trim_right_5: (&vec![0,0,0,0], 4, vec![]), trim_binary_number_right,
        test_trim_right_6: (&vec![1,0,0,1], 1, vec![1,0,0,1]), trim_binary_number_right,
        test_trim_right_7: (&vec![0,1,0,0], 3, vec![0,1]), trim_binary_number_right,

        test_trim_1: (&vec![0,1,1,0], 1, vec![1,1,0]), trim_binary_number,
        test_trim_2: (&vec![1,1,1,0], 1, vec![1,1,1]), trim_binary_number
    }
}
