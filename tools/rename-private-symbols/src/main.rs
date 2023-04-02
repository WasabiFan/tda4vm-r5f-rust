use clap::Parser;
use object::{Object, ObjectSymbol, archive};
use tempfile::TempPath;

use std::{process::Command, path::{Path, PathBuf}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source files
    #[arg(long)]
    source_archive: Vec<PathBuf>,

    /// Output directory
    #[arg(long)]
    out_dir: PathBuf,
}

// fn partial_link_archive(archive_path: &str, out_dir: impl AsRef<Path>) -> PathBuf {
//     let out_path = out_dir.as_ref().join(Path::new(archive_path).with_extension(".o").file_name().unwrap());
fn partial_link_archive(archive_path_in: impl AsRef<Path>, object_path_out: impl AsRef<Path>) {
    let result = Command::new("arm-none-eabi-ld").args(
        [
            "-r",
            "-o", object_path_out.as_ref().to_str().unwrap(),
            "--whole-archive",
            archive_path_in.as_ref().to_str().unwrap(),
        ]).status().unwrap();
    assert!(result.success(), "relocatable link failed with status: {}", result);
}

// fn list_private_symbols<'r, R: object::ReadRef<'r>>(object: &object::File<'r, R>) {
//     object.symbols().filter(|s| s.is_undefined())
// }

fn rename_object_symbols(object_path_in: impl AsRef<Path>, object_path_out: impl AsRef<Path>) {
    // let file = std::fs::File::open(object_path_in).unwrap();
    let data = std::fs::read(&object_path_in).unwrap();
    let file = object::File::parse(&*data).unwrap();
    let unlinked_symbols:Vec<_> = file.symbols().filter(|s| s.is_undefined()).collect();
    println!("Localizing object symbols:");
    println!("\tInput: {}", object_path_in.as_ref().to_str().unwrap());
    println!("\tOutput: {}", object_path_out.as_ref().to_str().unwrap());
    println!("\tUnresolved symbols: {:?}", unlinked_symbols);
    

    // "--wildcard",
    // Localize all symbols by default
    // "--localize-symbol", "*",
    // Globalize unresolved (undefined) symbols only
    // "--globalize-symbol",
    // ,
    let result = Command::new("arm-none-eabi-objcopy")
        .args(unlinked_symbols.iter().flat_map(|s| ["--keep-global-symbol", s.name().unwrap()]))
        .arg(object_path_in.as_ref().to_str().unwrap())
        .arg(object_path_out.as_ref().to_str().unwrap())
        .status().unwrap();
    assert!(result.success(), "objcopy failed with status: {}", result);
}

fn main() {
    let args = Args::parse();

    let _ = std::fs::create_dir(&args.out_dir);

    for archive_path in &args.source_archive {
        let final_obj_path = args.out_dir.join(archive_path.with_extension("o").file_name().unwrap());
        let partially_linked_obj_path = final_obj_path.with_file_name(final_obj_path.with_extension("").file_name().unwrap().to_str().unwrap().to_owned() + "_partial.o");

        partial_link_archive(archive_path, &partially_linked_obj_path);
        rename_object_symbols(&partially_linked_obj_path, &final_obj_path);
    }
}
