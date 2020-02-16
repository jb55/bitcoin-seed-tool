

static CHECKSUM_CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
static INPUT_CHARSET: &[u8] = b"0123456789()[],'/*abcdefgh@:$%{}IJKLMNOPQRSTUVWXYZ&+-.;<=>?!^_|~ijklmnopqrstuvwxyzABCDEFGH`#\"\\ ";

fn charset_find(ch: u8) -> Option<usize> {
    for (i, item) in INPUT_CHARSET.iter().enumerate() {
        if *item == ch {
            return Some(i);
        }
    }
    None
}

fn polymod(c: u64, val: i32) -> u64
{
    let c0 = (c >> 35) as u8;
    let mut c = ((c & 0x7ffffffff) << 5) ^ (val as u64);
    if c0 & 1  > 0 { c ^= 0xf5dee51989 };
    if c0 & 2  > 0 { c ^= 0xa9fdca3312 };
    if c0 & 4  > 0 { c ^= 0x1bab10e32d };
    if c0 & 8  > 0 { c ^= 0x3706b1677a };
    if c0 & 16 > 0 { c ^= 0x644d626ffd };
    c
}


pub fn descriptor_checksum(descriptor: &str) -> Option<String>
{
    let mut checksum = String::with_capacity(8);

    let mut c : u64 = 1;
    let mut cls = 0;
    let mut clscount = 0;

    for ch in descriptor.bytes() {
        let pos = charset_find(ch)? as i32;

        // Emit a symbol for the position inside the group, for every character.
        c = polymod(c, pos & 31);

        // Accumulate the group numbers
        cls = cls * 3 + (pos >> 5);

        clscount += 1;
        if clscount == 3 {
            c = polymod(c, cls);
            cls = 0;
            clscount = 0;
        }
    }

    if clscount > 0 {
        c = polymod(c, cls);
    }

    // Shift further to determine the checksum.
    for _ in 0..8 {
        c = polymod(c, 0);
    }

    // Prevent appending zeroes from not affecting the checksum.
    c ^= 1;

    for j in 0..8 {
        let byte = CHECKSUM_CHARSET[((c as usize) >> (5 * (7 - j))) & 31];
        checksum.push(byte as char);
    }

    Some(checksum)
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_descriptor_checksum() {
        let desc = "combo(xpub661MyMwAqRbcEzDwwjzJhBnXBZpv9hdqY2JYRUtwzEZU6grFRhWkXx9c3HJ4EKR1Nvdwf5U3VoekstoKSKjfcJYhRrhMYeEzZzu2h7uZAQX/1/*)";
        let desc_check = "j38hxvh0";
        let checksum = descriptor_checksum(desc).unwrap();
        assert!(checksum == desc_check);
    }

}
