fn main() {
    //operation ที่เราคุ้นเคยก็ยังมีให้ใช้เหมือนเดิม: ==, !=, <, >, <=, >=, !, ||, &&.
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}
