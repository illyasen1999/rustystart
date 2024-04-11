// struct - a custom data type that lets you package together and name multiple related values that make up a meaningful group
// each instance of this struct owns all of its data and that data is to be valid as long as the struct is valid
struct User { // creating the Structure of User
    // fields - attributes of the structure
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Structs that store a ref to data owned by something else
// this is not possible yet without using Lifetimes
// struct UserRef {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
 // useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
struct Person;

#[derive(Debug)] // this is a derived trait for debugging
struct Rectangle {
    width: u32,
    height: u32,
}

// defining a method of Rectangle using implementation(Impl)
impl Rectangle { // Rectangle now has a method of rectangle_area
    fn rectangle_area(&self) -> u32 {
        return self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        let res = self.width > other.width && self.height > other.height;
        return res;
    }

    // Associated Functions
    // are not methods as they dont have the "&self" as their first parameter and often called using the double colon(::) syntax
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    // we can have a method the same name as the fields
    // when we give a method the same name as a field we want it to only return the value in the field and do nothing else, these are like "getters" in other programming langugages so that you can make the field private and the method public
     fn width(&self) -> bool { //  we are just checking if the user has provided a value on the width
        return self.width > 0;
     }
}

// Multiple impl Blocks - a struct can have multiple impl blocks and its still valid use
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         let res = self.width > other.width && self.height > other.height;
//         return res;
//     }
// }

// Enum
// gives you a way of saying a value is one of a possible set of values
enum IpAddrKind {
    V4(String),
    // V4(u8, u8, u8, u8), // other advantages of enums is that it can have different types and amounts of associated data ex. ipv4 always has 4 components that has values between 0-255
    V6(String),
}

// struct IpAddr { // this can aslo be represented just by adding data directly to each enum
//     kind: IpAddrKind,
//     address: String,
// }

// like structs, enums can also have impl methods attached to them
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("reply: Hi\n");
    }

    fn calling(message: String) -> String {
        // running some code
        return message;
    }
}


pub fn structures(){
    println!("Topic: Using Structs to Structure Related Data");

    // instance of a struct
    let mut user1 = User {
        // key,value pairs
        active: true,
        username: String::from("username123"),
        email: String::from("user_123@email.com"),
        sign_in_count: 5,
    };

    println!("User1 info \nactive: {} \nusername: {} \nemail: {} \nsign-in count: {}\n", &user1.active, &user1.username, &user1.email, &user1.sign_in_count);

    user1.email = String::from("new_user123@email.com");
    println!("New user1 email: {}", &user1.email);

    let new_user = create_user(String::from("new_user@email.com"), String::from("New User"));

    println!("New User email: {}\n",&new_user.email);

    // Creating Instance from other Instances w/ Struct Update Syntax
    // data from user1 has been moved to user2
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("user2@email.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // a much shorter way of doing it
    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1 // grabbing all the info of User1 and only updating a few fields
    };

    println!("User2 info \nactive: {} \nusername: {} \nemail: {} \nsign-in count: {}\n", &user2.active, &user2.username, &user2.email, &user2.sign_in_count);

    let color_black = Color(0, 0, 0);
    let point_points = Point(0, 0, 0);

    // Tuple Structs values can be accessed by index
    println!("Color Black - h: {}, s: {}, l: {}\n", color_black.0, color_black.1, color_black.2);

    println!("Point points - a: {}, b: {}, c: {}\n", point_points.0, point_points.1, point_points.2);

    let _illyasen = Person;
    // let name = illyasen.name = "Illyasen";

    // basic way
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area is {} square pixels\n", calc_area(width1, height1));

    // w/ tuples
    // let rect1 = (30, 50);
    // println!("The area is {} square pixels with tuples\n", calc_area(rect1));

    // w/ structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {} square pixels with structs\n", calc_area(&rect1));

    // Derived Traits
    // using the Debug derived trait values of the Rectangle struct can be seen with the {:?} or {:#?} print formatting
    println!("rect1: {:#?}\n", &rect1);

    // Sub-topic: Method Syntax
    // methods are defined within the context of a struct (or an enum or a trait object
    // methods can be accessed with the dot(.) notation
    println!("The area is {} square pixels with methods in structs\n", &rect1.rectangle_area());

    // with parentheses, Rust knows we mean the method width
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}a\n", rect1.width);
    }

    // Methods with More Parameters
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    
    println!("Can rect1 hold rect2?: {}\n", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}\n", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Square: {:?}\n", &sq);

    // creating instances of the IpAddrKind enum
    // let ip_ver_four = IpAddrKind::V4;
    // let ip_ver_six = IpAddrKind::V6;

    // let home_ip = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback_ip = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let _home_ip = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback_ip = IpAddrKind::V6(String::from("::1"));

    // sample of the enum having multiple values
    // let _home = IpAddr::V4(127, 0, 0, 1);

    let _m = Message::Write(String::from("Hello"));
    _m.call();

    let my_message = &Message::calling(String::from("Test message"));

    println!("My message: {}\n", &my_message);

    // Option enum - an enum type that either returns Some(T) or a None, to remove the null type that is common in other languages
    /* structure of the Option enum type
        enum Option<T> {
            None,
            Some(T),
        }
    */
    let x: Option<u32> = Some(32);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    let fetch_data: Option<String> = Some(String::from("Some url string"));
    println!("URL: {:?}", &fetch_data);

    // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
}

// creating a new instance of User using a function
fn create_user(email: String, username: String) -> User {
    return User {
        active: true,
        username, // no need to do like username: username
        email,
        sign_in_count: 5,
    }
}

// Program using Structs
// basic way - using 2 variables 
// fn calc_area(width: u32, height: u32) -> u32 {
//     return width * height
// }

// w/ tuples - using 1 variable but has no named elements
// fn calc_area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1
// }

// w/ structs - more meaningful requirements
// we want to borrow the struct rather than take ownership of it/
fn calc_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height
}

// fn route(ip_kind: IpAddrKind) {
//     //
// }