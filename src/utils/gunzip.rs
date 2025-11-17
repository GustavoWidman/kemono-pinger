use std::io::Read;

use eyre::Result;
use log::{error, warn};

pub async fn gunzip(response: reqwest::Response) -> Result<String> {
    let bytes = response.bytes().await?;

    if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
        let mut decoder = flate2::read::GzDecoder::new(&bytes[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed).map_err(|e| {
            error!("{:?}", bytes);
            e
        })?;

        Ok(decompressed)
    } else {
        let out = String::from_utf8_lossy(&bytes[..]).into();
        warn!(
            "response came back that was not gzipped, this is worth checking out:\n{}",
            out
        );
        Ok(out)
    }
}
