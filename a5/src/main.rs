use std::io;

mod rawfile;
mod point;
use rawfile::RawFile;

fn main() -> io::Result<()> {
    let mut file = RawFile::new("brain_scan_256_256_unsigned_char.raw", 256, 256)?;
    file.build_and_write_isolines(81.5, "brain_scan_iso_81_5.obj")?;

    file = RawFile::new("neuron_1024_1024_unsigned_char.raw", 1024, 1024)?;
    file.build_and_write_isolines(128.0, "neuron_iso_128.obj")?;

    Ok(())
}
