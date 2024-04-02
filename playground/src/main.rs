fn main() {
    // immutable and mutable references
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2} are {s}");

    let r3 = &mut s;
    println!("{r3} is {r3}");
    //println!("{r3} is {s}"); // compiler error: cannot borrow `s` as mutable

    // Slice example
    let s = String::from("hello world");

    let hello = first_word(&s);

    let world = second_word(&s);

    println!("{hello} {world}");

    // Structs
    let mut user1 = User {
        active: true,
        username: String::from("myusername123"),
        email: String::from("myemail123@example.com"),
        sign_in_count: 1,
    };
    println!("User created: {user1}");
    // change the sign_in_count
    user1.sign_in_count += 1;
    println!("User modified: {user1}");
    let user2 = build_user(
        String::from("myemail456@example.com"),
        String::from("myusername456"),
    );
    let user3 = User {
        email: String::from("myemail789@example.com"),
        username: String::from("myusername789"),
        active: user1.active,
        ..user2
    };
    println!("Other users: {user2}\n{user3}");
    println!("User1: {user1}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{\n  active: {},\n  username: {},\n  email: {},\n  sign_in_count: {}\n}}",
            self.active, self.username, self.email, self.sign_in_count
        )
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
