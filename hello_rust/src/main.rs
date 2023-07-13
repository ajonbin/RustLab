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
    #[derive(Debug)]
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
    //println!("{:?}",msg)
    //                ^^^ value borrowed here after partial move
  }

  println!("===== Loop =====");
  {
   	{
   		let names: [String; 2] = [String::from("hello"), String::from("world")];

     	//move occurs (when looping) because `names` has type `[String; 2]`, which does not implement the `Copy` trait
	   	for name in names {
	    	println!("{}",type_of(&name));
				//Output: alloc::string::String
	    	println!("{}", name);
	    }
	    //println!("{:?}", names)
	    //								 ^^^^^ value borrowed here after move
    }
    {
   		let names: [String; 2] = [String::from("hello"), String::from("world")];
     	//Borrow in Loop
	   	for name in &names {
	    	println!("{}",type_of(&name));
				//Output: &alloc::string::String
	    	println!("{}", name);
	    }
	    println!("{:?}", names)
    }
  }

  println!("==== if let ====");
  {
    let cfg_max = Some(100u32);
    match cfg_max {
      Some(max) => println!("Max is {}", max),
      // Must match all values, kind of verbose
      _ => ()
    }

    // Use if let to unwarp a Option value
    if let Some(another_max) = cfg_max {
      println!("Another max is {}", another_max);
    }

    if let Some(thrid_max) = Option::<u32>::None {
      panic!("Should not be matched");
    }
  }

  println!("==== Match ====");
  {
    enum Direction {
      EAST,
      SOUTH,
      WEST,
      NORTH,
    }

    let direct1 = Direction::SOUTH;
    match direct1 {
      Direction::SOUTH | Direction::NORTH => println!("South or North"),
      _ => println!("Not care"),
    }
    //Output: South or North

    let direct2 = Direction::EAST;
    match direct2 {
      Direction::SOUTH | Direction::NORTH => println!("South or North"),
      _ => println!("Not care"),
    }
    //Output: Not care
  }

  println!("==== matches! ====");
  {
    enum Direction {
      EAST,
      SOUTH,
      WEST,
      NORTH,
    }

    let directs: Vec<Direction> = vec![Direction::EAST, Direction::WEST, Direction::NORTH];
    let direct1: Direction = Direction::EAST;

    /* ERROR
    if direct1 == Direction::EAST{
      println!("direct1 is east");
    }
    */
    //error[E0369]: binary operation `==` cannot be applied to type `main::Direction`

    if matches!(direct1, Direction::EAST) {
      println!("direct1 matches! with east");
    }
  }

  println!("==== self vs &self ====");
  {
    #[derive(Debug)]
    struct Bird;

    impl Bird{
      fn fly(self) -> (){
        println!("The Bird is moved here(by self), not available after call");
      }

      fn stay(&self) -> (){
        println!("The Bird is borrowed here(by &self), will be still available after call");
      }
    }

    let b1 = Bird;
    let b2 = Bird;
    b1.fly();
    //  ----- `b1` moved due to this method call
    b2.stay();

    //println!("{:?}", b1);
    //                 ^^ value borrowed here after move
    println!("{:?}", b2);
  }
}
