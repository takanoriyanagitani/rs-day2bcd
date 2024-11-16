static INVALID: u8 = 0x20;

static DAY_TO_BCD: [u8; 32] = [
    INVALID, // INVALID VALUE
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, // 1, 2, 3, ... , 9
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, // 10, 11, 12, ... , 19
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, // 20, 21, 22, ... , 29
    0x30, 0x31, // 30, 31
];

pub fn day_to_binary_coded_decimal(day: u8) -> u8 {
    let u: u8 = day & 0x1f;
    DAY_TO_BCD[u as usize]
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn day2bcd(day: u8) -> u8 {
    day_to_binary_coded_decimal(day)
}

#[cfg(test)]
mod test {
    mod day_to_binary_coded_decimal {
        macro_rules! create_test {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let got: u8 = crate::day_to_binary_coded_decimal($input);
                    assert_eq!(got, $expected);
                }
            };
        }

        create_test!(day01, 0x01, 0x01);
        create_test!(day02, 0x02, 0x02);
        create_test!(day03, 0x03, 0x03);
        create_test!(day04, 0x04, 0x04);
        create_test!(day05, 0x05, 0x05);
        create_test!(day06, 0x06, 0x06);
        create_test!(day07, 0x07, 0x07);
        create_test!(day08, 0x08, 0x08);
        create_test!(day09, 0x09, 0x09);

        create_test!(day10, 10, 0x10);
        create_test!(day11, 11, 0x11);
        create_test!(day12, 12, 0x12);
        create_test!(day13, 13, 0x13);
        create_test!(day14, 14, 0x14);
        create_test!(day15, 15, 0x15);
        create_test!(day16, 16, 0x16);
        create_test!(day17, 17, 0x17);
        create_test!(day18, 18, 0x18);
        create_test!(day19, 19, 0x19);

        create_test!(day20, 20, 0x20);
        create_test!(day21, 21, 0x21);
        create_test!(day22, 22, 0x22);
        create_test!(day23, 23, 0x23);
        create_test!(day24, 24, 0x24);
        create_test!(day25, 25, 0x25);
        create_test!(day26, 26, 0x26);
        create_test!(day27, 27, 0x27);
        create_test!(day28, 28, 0x28);
        create_test!(day29, 29, 0x29);

        create_test!(day30, 30, 0x30);
        create_test!(day31, 31, 0x31);
    }
}
