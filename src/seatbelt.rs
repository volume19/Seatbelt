//! Main Seatbelt orchestrator
//!
//! This module ties together CLI parsing, runtime creation, and command execution.

use crate::cli::{CliArgs, SeatbeltConfig};
use crate::error::Result;
use crate::output::{ConsoleWriter, FileWriter, JsonFileOutputSink, TextOutputSink};
use crate::runtime::{Runtime, RuntimeConfig};
use std::time::Instant;

const VERSION: &str = "1.2.2";

/// Main Seatbelt application
pub struct Seatbelt {
    config: SeatbeltConfig,
}

impl Seatbelt {
    /// Create a new Seatbelt instance from command-line arguments
    pub fn from_args() -> Self {
        let args = CliArgs::parse_args();
        let config = SeatbeltConfig::from(args);
        Self { config }
    }

    /// Print the Seatbelt logo
    fn print_logo() {
        println!("\n");
        println!("                        %&&@@@&&                                                                                  ");
        println!("                        &&&&&&&%%%,                       #&&@@@@@@%%%%%%###############%                         ");
        println!("                        &%&   %&%%                        &////(((&%%%%%#%################//((((###%%%%%%%%%%%%%%%");
        println!("%%%%%%%%%%%######%%%#%%####%  &%%**#                      @////(((&%%%%%%######################(((((((((((((((((((");
        println!("#%#%%%%%%%#######%#%%#######  %&%,,,,,,,,,,,,,,,,         @////(((&%%%%%#%#####################(((((((((((((((((((");
        println!("#%#%%%%%%#####%%#%#%%#######  %%%,,,,,,  ,,.   ,,         @////(((&%%%%%%%######################(#(((#(#((((((((((");
        println!("#####%%%####################  &%%......  ...   ..         @////(((&%%%%%%%###############%######((#(#(####((((((((");
        println!("#######%##########%#########  %%%......  ...   ..         @////(((&%%%%%#########################(#(#######((#####");
        println!("###%##%%####################  &%%...............          @////(((&%%%%%%%%##############%#######(#########((#####");
        println!("#####%######################  %%%..                       @////(((&%%%%%%%################                        ");
        println!("                        &%&   %%%%%      Seatbelt         %////(((&%%%%%%%%#############*                         ");
        println!(
            "                        &%%&&&%%%%%        v{}         ,(((&%%%%%%%%%%%%%%%%%,                                 ",
            VERSION
        );
        println!("                         #%%%%##,                                                                                 \n");
    }

    /// Print usage information
    fn print_usage(runtime: &Runtime) {
        println!("Available commands (+ means remote usage is supported):\n");

        for command in runtime.commands() {
            let prefix = if command.supports_remote() { "+" } else { " " };
            println!("  {} {:<22} - {}", prefix, command.name(), command.description());
        }

        println!("\n\nSeatbelt has the following command groups: All, User, System, Slack, Chromium, Remote, Misc");
        println!("\n    You can invoke command groups with         \"seatbelt <group>\"\n");
        println!("\nExamples:");
        println!("  'seatbelt <Command> [Command2] ...' will run one or more specified checks only");
        println!("  'seatbelt <Command> --full' will return complete results for a command without any filtering.");
        println!("  'seatbelt --group=all' will run ALL enumeration checks, can be combined with \"--full\".");
        println!("  'seatbelt --group=system --output-file=output.txt' will run system checks and output to a file.");
        println!("  'seatbelt --group=user -q --output-file=output.json' will run in quiet mode with user checks and output to JSON.");
    }

    /// Run the Seatbelt application
    pub fn run(&self) -> Result<()> {
        let start_time = Instant::now();

        // Create output sink based on configuration
        let output_sink = self.create_output_sink()?;

        // Create runtime configuration
        let runtime_config = RuntimeConfig {
            filter_results: self.config.filter_results,
            delay_ms: self.config.delay_ms,
            randomize_order: self.config.randomize_order,
            remote: None, // TODO: Handle remote config
        };

        // Create runtime and register commands
        let mut runtime = Runtime::new(output_sink, runtime_config);
        self.register_commands(&mut runtime);

        // Print logo if not in quiet mode
        if !self.config.quiet_mode {
            Self::print_logo();
        }

        // If no commands or groups specified, show usage
        if self.config.commands.is_empty() && self.config.command_groups.is_empty() {
            Self::print_usage(&runtime);
            return Ok(());
        }

        // Execute command groups
        for group in &self.config.command_groups {
            runtime.execute_group(group)?;
        }

        // Execute individual commands
        if !self.config.commands.is_empty() {
            runtime.execute_commands(&self.config.commands)?;
        }

        // Print completion time if not in quiet mode
        if !self.config.quiet_mode {
            let elapsed = start_time.elapsed();
            runtime.write_verbose(&format!(
                "\n[*] Completed collection in {:.2} seconds\n",
                elapsed.as_secs_f64()
            ));
        }

        Ok(())
    }

    /// Create the output sink based on configuration
    fn create_output_sink(&self) -> Result<Box<dyn crate::output::OutputSink>> {

        if let Some(ref path) = self.config.output_file {
            if path.ends_with(".json") {
                // JSON file output
                Ok(Box::new(JsonFileOutputSink::new(
                    path,
                    self.config.filter_results,
                )?))
            } else {
                // Text file output
                let writer = FileWriter::new(path)?;
                Ok(Box::new(TextOutputSink::new(
                    writer,
                    self.config.filter_results,
                )))
            }
        } else {
            // Console output
            let writer = ConsoleWriter::new();
            Ok(Box::new(TextOutputSink::new(
                writer,
                self.config.filter_results,
            )))
        }
    }

    /// Register all available commands
    fn register_commands(&self, runtime: &mut Runtime) {
        // Register Windows commands
        runtime.register_command(Box::new(crate::commands::windows::DnsCacheCommand::new()));

        log::debug!("Registered {} commands", runtime.commands().len());
    }
}
