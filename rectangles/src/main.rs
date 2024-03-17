#[derive(Debug)] // 구조체에 디버깅 정보를 출력 적용
struct Rectangle {
    width: u32,
    height: u32,
}

// 구조체에 메서드를 붙이기
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect1);
    println!("area = {}", rect1.area());

    println!("rect1 -> rect2 {}", rect1.can_hold(&rect2));
    println!("rect1 -> rect3 {}", rect1.can_hold(&rect3));
}