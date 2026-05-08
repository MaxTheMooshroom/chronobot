mod bot;
mod chrono;
mod cli;
mod env;

use cli::Cli;

// (variable name, requirement reason)
// const REQUIRED_ENV_VARS: [(&str, &str); 1] = [
//     ("DISCORD_AUTH_TOKEN", "Connecting to discord")
// ];

fn main() -> anyhow::Result<()> {
    let _ = Cli::parse();

    if env::load().is_err() {
        println!("Failed to read `.env` file.");
        println!("This is required to connect to discord.");
        println!("\nAborting...");
        return Ok(());
    }

    let read_guard = env::read()?;
    let len: usize = read_guard.iter().fold(0, |len, (k, v)| {
        len + k.len() + " = ".len() + v.len() + '\n'.len_utf8()
    });
    let s = read_guard.iter().fold(
        String::with_capacity(len),
        |mut s: String, (k, v)| {
            s.push_str(k);
            s.push_str(" = ");
            s.push_str(v);
            s.push('\n');
            s
        }
    );

    assert!(s.len() == len);

    print!("{}", s);

    for (key, value) in chrono::dice::ROLL_TABLES.iter() {
        println!("{}: {:#?}", key, value);
    }

    println!("{:?}", chrono::dice::roll_dice(&vec![]));

    Ok(())
}
