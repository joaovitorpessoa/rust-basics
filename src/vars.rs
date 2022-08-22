pub fn execute() {
  let name = "My name";
  let mut age =  21;

  println!("My name is {} and I am {} years old", name, age);
  println!("~Birthday");
  age += 1;
  println!("My name is {} and I am {} years old", name, age);

  const SOMETHING_CONSTANT: i32 = 10;
  println!("Print of a constant {}", SOMETHING_CONSTANT);

  let (other_name, other_age) = ("Name", 29);
  println!("other_name: {}\tother_age: {}", other_name, other_age);
}