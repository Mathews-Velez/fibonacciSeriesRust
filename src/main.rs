use std::io::{self};

fn main() {
    println!("Enter the of sequences you would like to generate: ");
    //
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        let x:i32 = line.trim().parse().expect("Failed to parse line");
        let x_usize: usize = x as usize;    
        fib_sequence(x_usize);
}
fn fib_sequence(x:usize) {
    //initialize the vector with a size of [x] to store the numbers
    let mut nums = vec![0,1];
    //create interator var to give us a way to index 
    let mut i:usize = 0;
    while  i < x {
    // if block for index > 1
    if i > 1 {
        let sum:i32 ={ nums.get(i-1).unwrap() + nums.get(i-2).unwrap()};
        nums.push(sum);
        println!("{}",nums[i]);
    } 
    i+=1;
   } 
}
