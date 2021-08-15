//ฟังก์ชัน add รับอากิวเมนต์ประเภท i32 (signed integer of 32-bit length)
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

//หากคุณต้องการส่งคืนนิพจน์ คุณสามารถลบ return และเครื่องหมาย ; ที่ส่วนท้ายได้
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

//NOTE: ชื่อฟังก์ชันจะเป็น snake_case เสมอ

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
}
