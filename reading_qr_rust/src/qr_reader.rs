use bytes::Bytes;
use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum QrReadError {
    #[error("image IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("image decode error: {0}")]
    Image(#[from] image::ImageError),
    #[error("QR decode error: {0}")]
    Decode(#[from] rqrr::DeQRError),
    #[error("no QR code found in image")]
    NotFound,
}

pub fn read_qr(img_bytes: Bytes) -> Result<String, QrReadError> {
    let gray = image::ImageReader::new(Cursor::new(img_bytes))
        .with_guessed_format()?
        .decode()?
        .to_luma8();

    let mut prepared = rqrr::PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();
    let grid = grids.into_iter().next().ok_or(QrReadError::NotFound)?;
    let (_meta, content) = grid.decode()?;
    Ok(content)
}
