use memmap2::Mmap;
use safetensors::tensor::SafeTensors;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let file = File::open(".kb/models/model.safetensors")?;
    let buffer = unsafe { Mmap::map(&file)? };
    let safetensors = SafeTensors::deserialize(&buffer)?;

    for name in safetensors.names() {
        println!("{}", name);
    }
    Ok(())
}
