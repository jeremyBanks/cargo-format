fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    cargo_format::init();
    Ok(cargo_format::main()?)
}
