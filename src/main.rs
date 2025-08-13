fn main() {
    // println!("Hello, world!");

    // numbers with size in bits 8,16,32,64,128 float only have f32 and f64
    // let x : i32 = 32;
    // let y : i32 = 45;
    // let z: f32 = 100.001;

    // println!("x: {}, y: :{}, z: {}", x, y, z);

    // let mut a = 10;
    // for i in 0..100 {
    //     a = a + 100;
    // }

    // println!("a: {}", a);

    // let is_male: bool = true;
    // let is_above_18: bool = true;

    // if is_male {
    //     println!("You are a male!")
    // } else {
    //     println!("you are a boy!")
    // }

    // if is_male && is_above_18 {
    //     println!("You are eligible to vote!")
    // }

    // let str2: &str ="Hello, word!";
    // let str = String::from("Hello, Rust!");
    // println!("{}", str);

    // getting the character at index 2 throws error if index is out of bounds
    // println!("{}", str.chars().nth(2).unwrap());

    // Get the lenght of the string
    // println!("Length of str: {}", str.len());

    // conditional statements
    // let is_even = true;

    // if is_even {
    //     println!("The number is even!");
    
    // } else if !is_even {
    //     println!("The number is odd!")
    // }

    // Iterate over string characters
    // let sentence = String::from("Hello Jahir!");
    // let res = get_first_word(sentence);
    // println!("First character: {}", res);

    // function in rust

    let a: i32 = 10;
    let b: i32 = 20;
    let add = is_add(a,b);
    println!("The sum: {}", add)
}

fn is_add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn get_first_word (str: String) -> String {
    let mut ans: String = String::from("");
    for char in str.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}