use std::fmt::Display;

use crate::gui::Register;
use crate::{engine::EngineCommand, gui::GUICommand};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tracing::error;
use tracing::instrument;
use tracing::warn;
mod parsing;
use std::io::Write;

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
        Ok(parsing::parse_line_from_gui(buf.as_str()))
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
        Ok(parsing::parse_line_from_engine(buf.as_str()))
    }
}

/// Includes a `'\n'` at the end
#[instrument(level = "trace")]
fn serialize_gui_command(command: GUICommand) -> Box<[u8]> {
    let mut output: Vec<u8> = Vec::with_capacity(30);
    match command {
        GUICommand::UCI => output.extend_from_slice(b"uci"),
        GUICommand::Debug(on) => {
            output.extend_from_slice(b"debug ");
            output.extend_from_slice(if on { b"on" } else { b"off" });
        }
        GUICommand::IsReady => output.extend_from_slice(b"isready"),
        GUICommand::SetOption { name, value } => {
            output.extend_from_slice(b"setoption name ");
            output.extend_from_slice(&name);
            match value {
                crate::gui::OptionType::CheckBox(on) => {
                    output.extend_from_slice(b" value ");
                    output.extend_from_slice(if on { b"true" } else { b"false" });
                }
                crate::gui::OptionType::Integer(i) => {
                    output.extend_from_slice(b" value ");
                    write!(output, "{i}")
                        .expect("Either you've run out of RAM or your OS is broken.");
                }
                crate::gui::OptionType::Dropdown(val) => {
                    output.extend_from_slice(b" value ");
                    output.extend_from_slice(&val);
                }
                crate::gui::OptionType::Button => (),
                crate::gui::OptionType::String(val) => {
                    output.extend_from_slice(b" value ");
                    output.extend_from_slice(&val);
                }
            }
        }
        GUICommand::UCINewGame => output.extend_from_slice(b"ucinewgame"),
        GUICommand::Go(_) => todo!(),
        GUICommand::PonderHit => output.extend_from_slice(b"ponderhit"),
        GUICommand::Position(_) => todo!(),
        GUICommand::Quit => output.extend_from_slice(b"quit"),
        GUICommand::Stop => output.extend_from_slice(b"stop"),
        GUICommand::Register(sub) => {
            output.extend_from_slice(b"register ");
            match sub {
                Some(Register { name, code }) => {
                    output.extend_from_slice(&name);
                    output.push(b' ');
                    output.extend_from_slice(&code);
                }
                None => output.extend_from_slice(b"later"),
            }
        }
        GUICommand::EOF => {
            error!("You should not be manually creating GUICommand::EOF. Use GUICommand::Quit instead.");
            output.push(b'\0');
        }
    }
    output.push(b'\n');
    output.into_boxed_slice()
}

/// Includes a `'\n'` at the end
#[instrument(level = "trace")]
fn serialize_engine_command(command: EngineCommand) -> Box<[u8]> {
    todo!()
}

/// Asynchronously sends a command to the given stream
#[instrument(skip(channel))]
pub async fn send_command_to_engine(
    channel: &mut (impl AsyncWriteExt + std::marker::Unpin),
    command: GUICommand,
) -> tokio::io::Result<()> {
    let string = serialize_gui_command(command);
    channel.write_all(&string).await
}
/// Asynchronously sends a command to the given stream
#[instrument(skip(channel))]
pub async fn send_command_to_gui(
    channel: &mut (impl AsyncWriteExt + std::marker::Unpin),
    command: EngineCommand,
) -> tokio::io::Result<()> {
    let string = serialize_engine_command(command);
    channel.write_all(&string).await
}
