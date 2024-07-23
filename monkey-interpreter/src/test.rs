

pub fn hello() -> &'static str {
    static RESULT: [u8; 3] = [97u8, 98u8, 99u8];
    str::(&RESULT).unwrap()

}