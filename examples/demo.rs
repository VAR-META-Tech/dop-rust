use dop::TsLib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lib = TsLib::new_from_path("ts-lib/dist/index.js").await?;
    let result = lib.greet("Duc").await?;
    println!("Result from TS2: {}", result);
    Ok(())
}
