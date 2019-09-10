fn main() {
    println!("\nHello, world!");
    let mut var :i8 = 30;
    println!("This is my variable var with value {}.", var);
    println!("It is an i8 i.e. Signed 8 bit integer");
    var = -128;
    println!("now it's value is {}; which is the lowest value an unsigned 8bit integer can take.\n", var);

    println!("Creating a function for factorial");
    //Anything higher than 20 gives an overflow error
    for i in 1..21 {
    	println!("factorial of {} is {}", i, factorial(i));
    }
}

pub fn factorial(n: u64) -> u64 {
	let mut i = 1;

	for x in 1..n+1 {
		i = i*x;
	}
	i
}