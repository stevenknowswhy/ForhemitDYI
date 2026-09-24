//! Writes the contract JSON Schemas to a directory.
//!
//! Usage: `cargo run -p forhemit-contracts --bin export-schemas -- <out-dir>`

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: export-schemas <out-dir>")?;
    std::fs::create_dir_all(&out_dir)?;

    for (name, schema) in forhemit_contracts::schema::export_per_type() {
        let path = out_dir.join(format!("{name}.schema.json"));
        std::fs::write(&path, schema + "\n")?;
        println!("wrote {}", path.display());
    }

    let index_path = out_dir.join("index.json");
    std::fs::write(
        &index_path,
        forhemit_contracts::schema::export_index() + "\n",
    )?;
    println!("wrote {}", index_path.display());

    Ok(())
}
