use std::env::args

fn main() {
  let args: Vec<String> = env::args.collect();
  // the first element in args is the path of our binary
  let sfirstnum = &args[1];
  let operator = &args[2];
  let sthirdnum = &args[3];
  
  let firstnum: i32 = sfirstnum.parse().expect("INVALID FIRST NUMBER");
  let thirdnum: i32 = sthirdnum.parse().expect("INVALID SECOND NUMBER");
  if operator == '+' {
    println!("{sfirstnum}+{sthirdnum}={}", firstnum+thirdnum);
    }
  elif operator == '-' {
    println!("{sfirstnum}-{sthirdnum}={}", firstnum-thirdnum);
    