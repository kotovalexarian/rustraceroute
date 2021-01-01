pub fn checksum(data: &[u8]) -> u16 {
    let mut data: Vec<u8> = data.to_vec();
    data.push(0);

    let mut sum: u32 = 0;

    for index in 0..(data.len() - 1) {
        if index % 2 == 0 {
            let current = data[index]     as u32;
            let next    = data[index + 1] as u32;

            sum = sum.wrapping_add((current << 8) + next);
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
    }

    (!sum & 0xFFFF) as u16
}

#[cfg(test)]
#[test]
fn tests() {
    assert_eq!(255,   checksum(b"\xFF"));
    assert_eq!(4095,  checksum(b"\xF0"));
    assert_eq!(61695, checksum(b"\x0F"));

    assert_eq!(65535, checksum(b""));
    assert_eq!(65535, checksum(b"\x00"));
    assert_eq!(65535, checksum(b"\x00\x00"));
    assert_eq!(65535, checksum(b"\x00\x00\x00"));

    assert_eq!(40703, checksum(b"a"));
    assert_eq!(40703, checksum(b"a\x00"));
    assert_eq!(40703, checksum(b"a\x00\x00"));
    assert_eq!(40703, checksum(b"a\x00\x00\x00"));
    assert_eq!(65438, checksum(b"\x00a"));
    assert_eq!(40703, checksum(b"\x00\x00a"));
    assert_eq!(65438, checksum(b"\x00\x00\x00a"));
    assert_eq!(40703, checksum(b"\x00\x00\x00\x00a"));

    assert_eq!(40605, checksum(b"ab"));
    assert_eq!(15261, checksum(b"abc"));
    assert_eq!(15161, checksum(b"abcd"));
    assert_eq!(54840, checksum(b"abcde"));
    assert_eq!(54738, checksum(b"abcdef"));
    assert_eq!(28370, checksum(b"abcdefg"));
    assert_eq!(10632, checksum(b"qwe"));
    assert_eq!(46236, checksum(b"qwerty"));
    assert_eq!(51387, checksum(b"foobar"));

    assert_eq!(16320, checksum(&(0u8..=255u8).collect::<Vec<u8>>()));
}
