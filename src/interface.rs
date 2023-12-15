use crate::{engine::EngineCommand, gui::GUICommand};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tracing::instrument;
use tracing::warn;

/// Assumes the partner is being nice and sending one command per line
#[instrument(level = "trace")]
fn parse_line_from_gui(line: &str) -> GUICommand {
    todo!()
}

/// Assumes the partner is being nice and sending one command per line
#[instrument(level = "trace")]
fn parse_line_from_engine(line: &str) -> EngineCommand {
    todo!()
}

/// Asynchronously gets the next `GUICommand` from the given stream.
#[instrument(skip_all)]
pub async fn next_gui_command(
    channel: &mut (impl AsyncBufReadExt + std::marker::Unpin),
) -> tokio::io::Result<GUICommand> {
    let mut buf = String::with_capacity(30);
    let len = channel.read_line(&mut buf).await?;
    // len == 0 implies that the stream has hit EOF according to the `channel.read_line` documentation
    if len == 0 {
        warn!("Stream unexpectedly reached EOF!");
        Ok(GUICommand::EOF)
    } else {
        Ok(parse_line_from_gui(buf.as_str()))
    }
}

#[instrument(skip_all)]
pub async fn next_engine_command(
    channel: &mut (impl AsyncBufReadExt + std::marker::Unpin),
) -> tokio::io::Result<EngineCommand> {
    let mut buf = String::with_capacity(30);
    let len = channel.read_line(&mut buf).await?;
    // len == 0 implies that the stream has hit EOF according to the `channel.read_line` documentation
    if len == 0 {
        warn!("Stream unexpectedly reached EOF!");
        Ok(EngineCommand::EOF)
    } else {
        Ok(parse_line_from_engine(buf.as_str()))
    }
}

/// Includes a `'\n'` at the end
#[instrument(level = "trace")]
fn serialize_gui_command(command: GUICommand) -> String {
    todo!()
}

/// Includes a `'\n'` at the end
#[instrument(level = "trace")]
fn serialize_engine_command(command: EngineCommand) -> String {
    todo!()
}

/// Asynchronously sends a command to the given stream
#[instrument(skip(channel))]
pub async fn send_command_to_engine(
    channel: &mut (impl AsyncWriteExt + std::marker::Unpin),
    command: GUICommand,
) -> tokio::io::Result<()> {
    let string = serialize_gui_command(command);
    channel.write_all(string.as_bytes()).await
}
/// Asynchronously sends a command to the given stream
#[instrument(skip(channel))]
pub async fn send_command_to_gui(
    channel: &mut (impl AsyncWriteExt + std::marker::Unpin),
    command: EngineCommand,
) -> tokio::io::Result<()> {
    let string = serialize_engine_command(command);
    channel.write_all(string.as_bytes()).await
}
