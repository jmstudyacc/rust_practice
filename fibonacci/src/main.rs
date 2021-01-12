// Computing Fibonacci recursively
// Algorithm: f(n+1) = f(n) + f(n-1)

use std::collections::HashMap;

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

/*fn fib(n: u64) -> u64 {
    // Need base cases
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
    /* The above code block is one expression - hence lacking any semi-colons.
    If the function returns FIB_ZERO, FIB_ONE or the result of the else the function itself
    returns the evaluation e.g. FIB_ZERO may be returned or FIB_ONE or the else statement */
}
*/
/*The above algorithm is poor as it requires constant re-evaluation.
The optimal approach is to store previously computed values and reference.

An option on how to do this is to use a HashMap provided by Rust.
*/

fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,  // this arrow indicates matching
        n => {
            if map.contains_key(&n) {    // if they number passed to the map is there get the value
                *map.get(&n).unwrap()     // if the key does not exist, end the code
                // uses *map to return the value and not the memory location
            } else {
                let val = fib_dyn(n - 1, map) + fib_dyn(n - 2, map);
                map.insert(n, val); // finds the value of the number n & does not recompute
                val
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();

        //for i in 1..101 {
        //    println!("{}: {}", i, fib(i))
        //}
        for i in 1..94 {
            println!("{}: {}", i , fib_dyn(i, &mut map))    // using & denotes that you are referencing a variable
    }
}