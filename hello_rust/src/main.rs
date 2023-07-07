use std::mem::size_of_val;

fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}

fn main() {
	println!("====== Unit type ======");
	{
	  println!("{:?}", implicitly_return_unit_type());
	  let ut = implicitly_return_unit_type();
	  println!("type_of(()) is {:?}",type_of(&ut));
	  println!("size of () is {}", size_of_val(&ut));
		fn implicitly_return_unit_type() -> (){
		  ()
		}
		/* Output
		====== Unit type ======
		()
		type_of(()) is "()"
		size of () is 0
	 	*/
	}

  println!("====== Enum ======");
  {
    enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
    }
    //let msg = Message::Move{x: 1, y: 2};
    //Output: 1,2

    //let msg = Message::ChangeColor(10,100,100);
    //Output: 10100100

    let msg = Message::Write("10,100,100".to_string());
    //Output: 10,100,100

    match msg {
      Message::Move{x:a, y:b} => println!("{},{}",a,b),
      Message::Quit => (),
      Message::Write(a) => println!("{}",a),
      Message::ChangeColor(a,b,c) => println!("{}{}{}",a,b,c),
    }
  }
}
