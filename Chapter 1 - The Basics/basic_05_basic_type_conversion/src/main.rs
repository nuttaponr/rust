fn main() {
    let a = 13u8;
    let b = 7u32;
    //NOTE: Rust มีวิธีแปลงค่าตัวเลขที่ง่ายมากๆด้วยการใช้คีย์เวิร์ด as
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}
