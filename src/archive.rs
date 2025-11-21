use anyhow::Result;
use std::fs::File;
use std::path::Path; 
use walkdir::WalkDir;
use tar::Builder;
use zstd::stream::write::Encoder as ZstdEncoder;

/// Compresses a directory into a Zstd-compressed Tar byte vector.
/// `ignore_path`: The specific output file path to exclude (prevents infinite recursion).
pub fn create_archive(path: &Path, ignore_path: &Path) -> Result<Vec<u8>> {
    // 1. Create the inner writer (Buffer)
    let memory_buffer = Vec::new();
    
    // 2. Create Zstd Encoder (takes ownership of buffer)
    let zstd_encoder = ZstdEncoder::new(memory_buffer, 0)?; 

    // 3. Create Tar Builder (takes ownership of zstd_encoder)
    let mut tar_builder = Builder::new(zstd_encoder);

    // 4. Walk and add files
    let parent_dir = path.parent().unwrap_or(Path::new("/"));
    
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path_on_disk = entry.path();

        // --- SAFETY CHECK ---
        // 1. Explicitly ignore the file we are currently writing to
        if path_on_disk == ignore_path {
            continue;
        }

        // 2. General Safety: Ignore any other .shadow files found in the folder
        if let Some(ext) = path_on_disk.extension() {
            if ext == "shadow" {
                continue;
            }
        }
        // --------------------

        // Calculate relative path inside the tarball
        let path_in_archive = path_on_disk.strip_prefix(parent_dir)?;

        if path_on_disk.is_dir() {
            tar_builder.append_dir(path_in_archive, path_on_disk)?;
        } else {
            let mut file = File::open(path_on_disk)?;
            tar_builder.append_file(path_in_archive, &mut file)?;
        }
    }

    // 5. Finish Tar and retrieve the ZstdEncoder back
    let zstd_encoder = tar_builder.into_inner()?;
    
    // 6. Finish Zstd and retrieve the Vec<u8> back
    let compressed_data = zstd_encoder.finish()?;

    Ok(compressed_data)
}

/// Decompresses and unpacks bytes to a destination.
pub fn unpack_archive(data: &[u8], dest: &Path) -> Result<()> {
    use std::io::Cursor;

    // 1. Wrap data in Cursor
    let cursor = Cursor::new(data);

    // 2. Zstd Decoder
    let zstd_decoder = zstd::stream::read::Decoder::new(cursor)?;

    // 3. Tar Archive
    let mut archive = tar::Archive::new(zstd_decoder);

    // 4. Unpack
    archive.unpack(dest)?;

    Ok(())
}
