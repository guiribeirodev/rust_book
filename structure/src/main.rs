// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // println!("Hello, world!");
    // let user1 = User {
    //     username: "teste".to_string(),
    //     sign_in_count: 1,
    //     email: "stes".to_string(),
    //     active:true,
    // };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // println!("{}", user1.email);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("rect1 is {rect1:?}");
    // println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
