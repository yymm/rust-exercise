extern crate exercise;

use exercise::fibonacci;
use exercise::fizzbuzz;

fn main() {
    println!("{}", fibonacci::fib_repeat(5));
    //println!("{}", fibonacci::fib_memo(10));
    for i in 1..100 {
        println!("{}", fizzbuzz::fizzbuzz_match(i));
    }
}
