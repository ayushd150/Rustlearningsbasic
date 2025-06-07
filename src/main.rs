// // // fn main() {
// // //     println!("Hello, world!");
// // // }

// // fn main(){
// //     let x: i32 = -5; //both positive and negative
// //     let y: u32 = 1000; // only positive
// //     let z: f32 = 12.2222; //both positive and negative and have decimal places
// //     print!("x:{}",x);
// //     print!("y:{}",y);
// //     print!("z:{}",z);
// // }//mutate-change value, 3 types - number, boolean, string 

// fn main(){
//     let greeting: String = String::from("hi bro");
//     println!("{}", greeting);
//     let char1: Option<char> =  greeting.chars().nth(90);
//     println!("{}", char1.unwrap());
// }

fn main (){
    let a: i32 = 10;
    let b: i32 = 20;
    let sum: i32 = do_sum(a,b);
    println!("sum is {}", sum);
}
fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}