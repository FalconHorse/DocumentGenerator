use std::io::{Cursor, Seek, SeekFrom, Write};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use zip::write::FileOptions;
use crate::generator::FileMap;

pub async fn zip_and_return(file_map: FileMap) -> zip::result::ZipResult<Vec<u8>> {
    let mut file = Vec::new();
    {
    let mut zip = zip::ZipWriter::new(Cursor::new(&mut file));
    zip.add_directory("images/", Default::default())?;
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);

    for (file_name, data) in file_map {
        let file_name = format!("images/{}.pdf", file_name);
        zip.start_file(file_name, options)?;
        zip.write_all(&data)?;
    }
    zip.finish()?;
    }

    let mut out = Vec::new();

    let mut file_cursor = Cursor::new(file);
    AsyncSeekExt::seek(&mut file_cursor, SeekFrom::Start(0)).await.expect("TODO: panic message");
    file_cursor.read_to_end(&mut out).await.expect("TODO: panic message");

    Ok(out)
}