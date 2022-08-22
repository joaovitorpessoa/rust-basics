pub fn execute() {
  // Like process.argv
  let args: Vec<String> = std::env::args().collect();
  println!("CLI args: {:?}", args);
}