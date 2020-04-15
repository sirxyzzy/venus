extern crate venus;

fn main() {
    println!("I am an example of using Venus, my name is Pong!");
    let context = venus::Context::new();
    context.run_event_loop()
}