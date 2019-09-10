fn main() {
    println!("Hello, world!\n");
    
    fibonacci(50)
}

pub fn fibonacci(n: usize) {
	//Prints first n fibonacci terms
	let mut n_0 = 0;
	let mut n_1 = 1;

	let mut n_2:usize; 

	println!("{}", n_0);
	println!("{}", n_1);

	for _x in 2..n {
		n_2 = n_0 + n_1;
		println!("{}", n_2);

		n_0 = n_1;
		n_1 = n_2;
	}

} 