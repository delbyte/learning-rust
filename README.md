# RUST CHEATSHEET

Opening a new project with Cargo 
```
>cargo new [project_name]
>cd [project_name]
```

**Cargo.toml** has your dependencies and build info
### All these commands need to be run while being in the Project's main directory

To compile a project and make an exe file
```
>cargo build
```
**Cargo stores your main.rs's executable in target/debug/, and expects main.rs to be in src.** 

#### **To compile and run the program in one line**
```
>cargo run 
```
**Very useful and this is what most devs use instead of compiling over and over.**

Use `cargo check` instead of  `cargo build/cargo run` while you're still writing the program to skip the step of building an exe and just see if your code compiles.

Use `cargo build --release` when all of your code is finished for the project. Compiles slower but applies optimizations that make the code run faster.

### Code stuff

Use libraries in your program 
```
use std::io;
```
Main function
```
fn main(){

}
```

Print statement 
```
println!("xyz");
```

**Declaring a variable**
Immutable(default)-
```
let apples = 5;
```
Mutable-
```
let mut apples = 5;
```

**Loops** 
```
loop(){
}
``` 
Loops Infinite Times












