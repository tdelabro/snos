pub mod config;
pub mod error;
pub mod hints;
pub mod io;
pub mod sharp;
pub mod state;
pub mod utils;

use std::fs;

use cairo_vm::cairo_run::{cairo_run, CairoRunConfig};
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use error::SnOsError;

pub struct SnOsRunner {
    layout: String,
    os_path: String,
}

impl SnOsRunner {
    pub fn with_layout(layout: &str) -> Self {
        Self { layout: layout.to_string(), os_path: Self::default().os_path }
    }

    pub fn with_os_path(os_path: &str) -> Self {
        Self { layout: Self::default().layout, os_path: os_path.to_string() }
    }

    pub fn run(&self) -> Result<CairoPie, SnOsError> {
        let mut sn_hint_processor = hints::sn_hint_processor();

        // Load the Starknet OS
        let starknet_os = fs::read(&self.os_path).map_err(|e| SnOsError::CatchAll(format!("{e}")))?;

        let (runner, vm) = cairo_run(
            &starknet_os,
            &CairoRunConfig {
                layout: self.layout.as_str(),
                relocate_mem: true,
                trace_enabled: true,
                ..Default::default()
            },
            &mut sn_hint_processor,
        )
        .map_err(SnOsError::Runner)?;

        let pie = runner.get_cairo_pie(&vm).map_err(|e| SnOsError::PieParsing(format!("{e}")))?;

        // TODO: also return program output
        Ok(pie)
    }
}

impl Default for SnOsRunner {
    fn default() -> Self {
        Self { layout: config::DEFAULT_LAYOUT.to_string(), os_path: config::DEFAULT_COMPILED_OS.to_string() }
    }
}
