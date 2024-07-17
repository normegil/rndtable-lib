use rndtable::loaders::filesystem::FilesystemLoader;

#[test]
fn filesystem_read_toml() {
    
    let loader = FilesystemLoader{ format: SupportedFormat::Toml, base_path: todo!() };
}