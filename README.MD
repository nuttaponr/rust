## https://tourofrust.com/00_th.html

## Installing  

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  

เมื่อติดตั้งเสร็จเรียบร้อยแล้ว ลองเช็ค version ของ rustc กับ cargo กันหน่อย   

`rustc --version`  
`cargo --version`

* rustc เป็น compiler ใช้ compile code
* cargo เป็น package manager ใข้ติดตั้ง lib

ต่อมาเราจะมาสร้าง Project กันจากคำสั่ง  

`cargo new --bin myproject` สำหรับ binary project  
`cargo new --lib myproject` สำหรับ library project

เมื่อ run คำสั่งแล้วเราจะได้หน้าตา ประมาณนี้

```
├── Cargo.toml
└── src
    └── main.rs
```
* Cargo.toml เป็น config ของโปรเจกต์โดยสามารถระบุรายละเอียดโปรเจกต์, dependencies ที่ใช้, ระบุ target ที่ต้องการเมื่อสั่ง build และ run  
ref https://doc.rust-lang.org/cargo/reference/manifest.html
* main.rs คือส่วนที่เราจะเขียน Code ตรงนี้

ต่อมาลอง build/run project กันดีกว่า  

`cargo build` โดย default จะเป็น Debug  
`cargo build --release`  ถ้าอยาก build เป็น release  
`cargo run` ก็สามารถ run project ได้เลยนะ

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── debug
        └── myproject
```
หลังจากเรา build เสร็จ
* Cargo.lock เอาไว้ lock version ของ lib หรือ dependencies คล้าย ๆ ตัว package-lock.json ของพวก node
* ./target/debug/myproject โดยปกติแล้วมันจะได้ binary ชื่อเดียวกับ projrct นั้นแหละ สามารถสั่ง run ได้เลย 
* rust มี document มาให้อยู่แล้วนะโดยการพิมพ์  `rust doc`