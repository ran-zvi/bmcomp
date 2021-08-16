# bmcomp
_requires rust nightly_

A failed attempt at creating a bitmap based compression algorithm, instead, the program encodes a file following the following steps:

1. create a bitmap for every value in the file
2. pack each 4 bits into a hex digit
3. convert the character of the bitmap into a hex number
4. convert the length of the file into a hex number
5. convert the hex number length into a 2 digit hex number
6. concat them like so:
`data_length_length + data_length + bitmaps`

Usage:

`bmcomp -i <input-file-path> -o <output-file-path> <subcommand: (decode, encode)>`

Example:

`bmcomp -i ./input-file -o ./output-file encode`

---

# Algorithm walkthrough

Input: `aaabbbccc`

### 1. Bitmaps
```rust
'a' -> 111000000
'b' -> 000111000
'c' -> 000000111
```

### 2. Hex Packing
```rust
'a' -> e00
'b' -> 1c0
'c' -> 021
```

### 3. Convert character to Hex

```rust
'a' -> 61 e00
'b' -> 62 1c0
'c' -> 63 021
```

### 4 + 5 Convert length
```rust
'aaabbbccc'.len() = 9 -> 0x9
'9'.len() = 1 -> 0x1
``` 

### 6 Concat and save output

```rust
'1' + '9' + '61e00' + '621c0' + '63021' -> '1961e00621c063021'
```

## Summary

As you can see, it's actually longer than the original input, the only scenario for which this program would compress the input is the following:

`input > (chars in input) * (input / 4) + 2 + (input length as hex)`

For example, assume an input comprised of the letters `'a' 'b' 'c'` which is 99 characters long.

```
99 > (3) * (99 /4) + 2 + (0x2)
99 > 74 + 4
99 > 76
```
