// // // // // // // fn main() {
// // // // // // //     println!("Hello, world!");
// // // // // // // }

// // // // // // fn main(){
// // // // // //     let x: i32 = -5; //both positive and negative
// // // // // //     let y: u32 = 1000; // only positive
// // // // // //     let z: f32 = 12.2222; //both positive and negative and have decimal places
// // // // // //     print!("x:{}",x);
// // // // // //     print!("y:{}",y);
// // // // // //     print!("z:{}",z);
// // // // // // }//mutate-change value, 3 types - number, boolean, string 

// // // // // fn main(){
// // // // //     let greeting: String = String::from("hi bro");
// // // // //     println!("{}", greeting);
// // // // //     let char1: Option<char> =  greeting.chars().nth(90);
// // // // //     println!("{}", char1.unwrap());
// // // // // }

// // // // fn main (){
// // // //     let a: i32 = 10;
// // // //     let b: i32 = 20;
// // // //     let sum: i32 = do_sum(a,b);
// // // //     println!("sum is {}", sum);
// // // // }
// // // // fn do_sum(a: i32, b: i32) -> i32 {
// // // //     return a + b;
// // // // }


// // // //stack-fixed time data at compile time
// // // //heap-store all things that can change in size at runtime like strings
// // // fn main() {
// // //     stack_fn();   // Call the function that uses stack memory
// // //     heap_fn();    // Call the function that uses heap memory
// // //     update_string();  // Call the function that changes size of variable at runtime
// // // }

// // // fn stack_fn() {
// // //     // Declare a few integers on the stack
// // //     let a = 10;
// // //     let b = 20;
// // //     let c = a + b;
// // //     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// // // }

// // // fn heap_fn() {
// // //     // Create a string, which is allocated on the heap
// // //     let s1 = String::from("Hello");
// // //     let s2 = String::from("World");
// // //     let combined = format!("{} {}", s1, s2);
// // //     println!("Heap function: Combined string is '{}'", combined);
// // // }

// // // fn update_string() {
// // //     // Start with a base string on the heap
// // //     let mut s = String::from("Initial string");
// // //     println!("Before update: {}", s);

// // //     // Append some text to the string
// // //     s.push_str(" and some additional text");
// // //     println!("After update: {}", s);
// // // }

// // // //every heap variable will have single stack owner to avoid dangling pointers and double free errors

// // fn main(){
// //     let s1: String = String::from("hi");
// //     print!("{}", s1);
// //     let s2: String = s1;
// //     print!("{}",s2); //s1 throws error
// // }

// fn main(){
//     let mut s1: String = String::from("hi");
//     s1.push_str("ayush");
//     print!("{}", s1);
// }

fn main(){
    let mut s1: String =  String::from("hi");
    update_str(&mut s1);
    print!("{}", s1)
    
}
fn update_str(s: &mut String){
    s.push_str("ayush");
}