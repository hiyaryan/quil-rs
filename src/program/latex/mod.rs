//! LaTeX diagram generation for quil programs.
//! 
//! Provides a feature to generate diagrams using the LaTeX subpackage TikZ/
//! Quantikz for a given quil Program.
//! 
//! - Usage: `Program.to_latex(settings: Settings);`
//! 
//! - Description:
//! [`Quantikz`] is a subpackage in the TikZ package used to generate qubit 
//! circuits. A qubit is represented as a wire separated into multiple columns.
//! Each column contains a symbol of an operation on the qubit. Multiple qubits 
//! can be stacked into rows with interactions between any number of them drawn 
//! as a connecting bar to each involved qubit wire. Commands are used to 
//! control what is rendered on a circuit, e.g. names of qubits, identifying 
//! control/target qubits, gates, etc. View [`Quantikz`] for the documentation 
//! on its usage and full set of commands.
//! 
//! This module should be viewed as a self contained partial implementation of 
//! [`Quantikz`] with all available commands listed as variants in a Command 
//! enum. This feature provides the user variability in how they wish to render 
//! their Program circuits with metadata contained in a Settings struct.
//! 
//! [`Quantikz`]: https://arxiv.org/pdf/1809.03842.pdf

use std::fmt::format;

use crate::Program;

/// Available commands used for building circuits with the same names taken 
/// from the Quantikz documentation for easy reference. LaTeX string denoted 
/// inside `backticks`.
///     Single wire commands: lstick, rstick, qw, meter
///     Multi-wire commands: ctrl, targ, control, (swap, targx)
pub enum Command {
    /// `\lstick{\ket{q_{u32}}}`: Make a qubit "stick out" from the left.
    Lstick(u32),
    /// `\rstick{\ket{q_{u32}}}`: Make a qubit "stick out" from the right.
    Rstick(u32),
    /// `\qw`: Connect the current cell to the previous cell i.e. "do nothing".
    Qw,
    /// `\meter{wire}`: Measure a qubit.
    Meter(u32),    
    /// `\ctrl{wire}`: Make a control qubit--different from Control.
    Ctrl(u32),
    /// `\targ{}`: Make a controlled-not gate.
    Targ,
    /// `\control{}`: Make a controlled-phase gate--different from Ctrl.
    Control,
    /// `\swap{wire}`: Make a swap gate--used with TargX.
    Swap(u32),
    /// `\targX{}`: Make a qubit the target for a swap--used with Swap.
    TargX,
}

impl Command {
    /// Returns the LaTeX String for a given Command variant.
    /// 
    /// # Arguments
    /// `command` - A Command variant.
    /// 
    /// # Examples
    /// ```
    /// use quil_rs::program::latex::Command;
    /// let lstick_ket_0 = Command::get_command(Command::Lstick(0));
    /// ```
    pub fn get_command(command: Self) -> String {
        match command {
            Self::Lstick(wire) => 
                format(format_args!(r#"\lstick{{\ket{{q_{{{wire}}}}}}}"#)),
            Self::Rstick(wire) => 
                format(format_args!(r#"\rstick{{\ket{{q_{{{wire}}}}}}}"#)),
            Self::Qw => r"\qw".to_string(),
            Self::Meter(wire) => format(format_args!(r"\meter{{{wire}}}")),
            Self::Ctrl(wire) => format(format_args!(r#"\ctrl{{{wire}}}"#)),
            Self::Targ => r"\targ{}".to_string(),
            Self::Control => r"\control{}".to_string(),
            Self::Swap(wire) => format(format_args!(r"\swap{{{wire}}}")),
            Self::TargX => r"\targX{}".to_string(),
        }
    }
}

/// Settings contains the metadata that allows the user to customize how the 
/// circuit is rendered or use the default implementation.
#[derive(Debug)]
pub struct Settings {
    /// Convert numerical constants, e.g. pi, to LaTeX form.
    texify_numerical_constants: bool,
    /// Include all qubits implicitly referenced in the Quil program.
    impute_missing_qubits: bool,
    /// Label qubit lines.
    label_qubit_lines: bool,
    /// Write controlled rotations in compact form.
    abbreviate_controlled_rotations: bool,
    /// Extend the length of open wires at the right of the diagram.
    qubit_line_open_wire_length: u32,
    /// Align measurement operations to appear at the end of the diagram.
    right_align_terminal_measurements: bool,
}

impl Default for Settings {
    /// Returns the default Settings.
    fn default() -> Self {
        Self { 
            /// false: π is pi.
            texify_numerical_constants: true, 
            /// true: `CNOT 0 2` would have three qubit lines: 0, 1, 2.
            impute_missing_qubits: false, 
            /// false: remove Lstick/Rstick from latex.
            label_qubit_lines: true, 
            /// true: `RX(pi)` displayed as `X_{\\pi}` instead of `R_X(\\pi)`.
            abbreviate_controlled_rotations: false, 
            /// 0: condenses the size of subdiagrams.
            qubit_line_open_wire_length: 1, 
            /// false: include Meter in the current column.
            right_align_terminal_measurements: true,
        }
    }
}

// TODO: Implement functions to update the settings that allows the user customzie the rendering of the circuit.
impl Settings {

}

#[derive(thiserror::Error, Debug)]
pub enum LatexGenError {
    // TODO: Add variants for each error type using `thiserror` crate to return detailed Result::Err. Example error below.
    #[error("This is an error on {qubit_index}.")]
    SomeError{qubit_index: u32},
}

pub trait Latex {
    fn to_latex(self, settings: Settings) -> Result<String, LatexGenError>;
}

impl Latex for Program {
    fn to_latex(self, settings: Settings) -> Result<String, LatexGenError> {
        // TODO: Generate the Program LaTeX.
        let latex = "";

        Ok(latex.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Settings, Latex};
    use crate::Program;
    use std::str::FromStr;

    /// Take an instruction and return the LaTeX using the to_latex method.
    pub fn get_latex(instructions: &str) -> String {
        let program = Program::from_str(instructions).expect("Program should be returned.");
        program
            .to_latex(Settings::default())
            .expect("LaTeX should generate without error.")
    }

    #[test]
    /// Test functionality of to_latex using default settings.
    fn test_to_latex() {
        let program = Program::from_str("").expect("");
        program.to_latex(Settings::default()).expect("");
    }

    mod gates {
        use crate::program::latex::tests::get_latex;

        #[test]
        fn test_gate_x() {
            insta::assert_snapshot!(get_latex("X 0"));
        }

        #[test]
        fn test_gate_y() {
            insta::assert_snapshot!(get_latex("Y 1"));
        }

        #[test]
        fn test_gate_controlled() {
            insta::assert_snapshot!(get_latex("CONTROLLED H 3 2"));
        }
    }

    /// Test module for command Operators
    mod commands {
        use crate::program::latex::Command;

        #[test]
        fn test_command_left_ket() {
            insta::assert_snapshot!(Command::get_command(Command::Lstick(0)));
        }

        #[test]
        fn test_command_right_ket() {
            insta::assert_snapshot!(Command::get_command(Command::Rstick(0)));
        }

        #[test]
        fn test_command_qw() {
            insta::assert_snapshot!(Command::get_command(Command::Qw));
        }

        #[test]
        fn test_command_measure() {
            insta::assert_snapshot!(Command::get_command(Command::Meter(0)));
        }

        #[test]
        fn test_command_control() {
            insta::assert_snapshot!(Command::get_command(Command::Ctrl(0)));
        }

        #[test]
        fn test_command_cnot_target() {
            insta::assert_snapshot!(Command::get_command(Command::Targ));
        }

        #[test]
        fn test_command_cphase_target() {
            insta::assert_snapshot!(Command::get_command(Command::Control));
        }

        #[test]
        fn test_command_swap() {
            insta::assert_snapshot!(Command::get_command(Command::Swap(0)));
        }

        #[test]
        fn test_command_swap_target() {
            insta::assert_snapshot!(Command::get_command(Command::TargX));
        }
    }
}
