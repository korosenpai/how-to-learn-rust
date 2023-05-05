// generate the nth fibonacci number

fn main() {

    println!("{}", fibonacci(9))
    
}

fn fibonacci(n: u32) -> u32{
    if n == 1 || n == 0 { return  n }
    fibonacci(n - 1) + fibonacci(n - 2)
}