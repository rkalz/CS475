use std::io;

mod rawfile;
mod point;
use rawfile::RawFile;

fn main() -> io::Result<()> {
    let file = RawFile::new("brain_scan_256_256_unsigned_char.raw", 256, 256)?;
    file.build_and_write_isolines(81.5, "brain_scan_iso_81_5.obj")?;

    Ok(())
}
