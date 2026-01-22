use anyhow::Result;

pub fn whoami() -> Result<()> {
    let current = std::env::var("CURRENT")?;
    if current.is_empty() {
        eprintln!("✘ Currently, no list is active");
    } else {
        println!("This is {current}. Ready for duty!");
    }
    Ok(())
}
