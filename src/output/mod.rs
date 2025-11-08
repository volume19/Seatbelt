//! Output system for Seatbelt
//!
//! This module provides the output abstraction layer with support for
//! multiple output formats (text, JSON) and destinations (console, file).

pub mod sink;
pub mod writer;
pub mod console_writer;
pub mod file_writer;
pub mod text_sink;
pub mod json_file_sink;

pub use sink::OutputSink;
pub use writer::TextWriter;
pub use console_writer::ConsoleWriter;
pub use file_writer::FileWriter;
pub use text_sink::TextOutputSink;
pub use json_file_sink::JsonFileOutputSink;
