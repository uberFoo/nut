use std::env;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    println!("{}", Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()));
    Ok(())
}
