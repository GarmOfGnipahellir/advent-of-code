use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

pub fn read_string<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}