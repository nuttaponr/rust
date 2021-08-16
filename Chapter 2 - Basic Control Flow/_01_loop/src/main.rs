fn main() {
    let mut x = 0;
    loop {
        //NOTE: ลูปแบบไม่มีจุดจบ ใช้ loop นะไม่ใช่ for
        x += 1;
        if x == 42 {
            break; //break จะพาคุณหนีออกจากลูปเมื่อคุณต้องการ
        }
    }
    println!("{}", x);
}
