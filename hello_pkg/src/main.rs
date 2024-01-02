fn main() {
    println!("Hello, world!");
		let result = simplemathlib::add::add(1,2);
    println!("1+2={}",result);
		let another_result = simplemathlib::multiple(1,2);
    println!("1x2={}",another_result);
}
