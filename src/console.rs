use std::io;

use anyhow::Result;
use serde::Serialize;

pub fn write_json<T>(json: T, pretty: bool) -> Result<()>
where
    T: Serialize,
{
    let json_fn = if pretty {
        serde_json::to_writer_pretty
    } else {
        serde_json::to_writer
    };

    json_fn(io::stdout(), &json)?;

    Ok(())
}
