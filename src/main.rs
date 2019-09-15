/* 
  When executes 'cargo run', rustc emits the below error message if uncomment
  error[E0584]: file for module `main` found at both main.rs and main/mod.rs
*/
// mod main;
mod test;

fn main() {
    println!("Hello, world!");
}
