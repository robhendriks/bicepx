use std::io;

use anyhow::{Context, Result};
use serde::Serialize;

pub fn write_json<T>(json: T, pretty: bool) -> Result<()>
where
    T: Serialize,
{
    let json_fn = match pretty {
        true => serde_json::to_writer_pretty,
        false => serde_json::to_writer,
    };

    json_fn(io::stdout(), &json).with_context(|| "Failed to output JSON to console")
}
