use std::io::{self, Write};

fn main() -> io::Result<()> {
    let input_from_user = get_input("Type something here:")?;
    println!("You typed: '{}'", input_from_user);
    Ok(())
}

fn get_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{} ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    buffer.pop();
    Ok(buffer)
}