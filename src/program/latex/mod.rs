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

use std::collections::HashMap;
use std::fmt::{format, Display};

use crate::Program;
use crate::instruction;
use crate::instruction::Qubit;

/// Available commands used for building circuits with the same names taken 
/// from the Quantikz documentation for easy reference. LaTeX string denoted 
/// inside `backticks`.
///     Single wire commands: lstick, rstick, qw, meter
///     Multi-wire commands: ctrl, targ, control, (swap, targx)
pub enum Command {
    /// `\lstick{\ket{q_{u32}}}`: Make a qubit "stick out" from the left.
    Lstick(String),
    /// `\rstick{\ket{q_{u32}}}`: Make a qubit "stick out" from the right.
    Rstick(String),
    /// ` \gate{name}`: Make a gate on the wire.
    Gate(String),
    /// `\qw`: Connect the current cell to the previous cell i.e. "do nothing".
    Qw,
    /// `\\`: Start a new row 
    Nr,
    /// `\meter{wire}`: Measure a qubit.
    Meter(String),    
    /// `\ctrl{wire}`: Make a control qubit--different from Control.
    Ctrl(String),
    /// `\targ{}`: Make a controlled-not gate.
    Targ,
    /// `\control{}`: Make a controlled-phase gate--different from Ctrl.
    Control,
    /// `\swap{wire}`: Make a swap gate--used with TargX.
    Swap(String),
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
    /// let ket_0 = "0".to_string();
    /// let lstick_ket_0 = Command::get_command(Command::Lstick(ket_0));
    /// ```
    pub fn get_command(command: Self) -> String {
        match command {
            Self::Lstick(wire) => 
                format(format_args!(r#"\lstick{{\ket{{q_{{{wire}}}}}}}"#)),
            Self::Rstick(wire) => 
                format(format_args!(r#"\rstick{{\ket{{q_{{{wire}}}}}}}"#)),
            Self::Gate(name) => 
                format(format_args!(r#"\gate{{{name}}}"#)),
            Self::Qw => r"\qw".to_string(),
            Self::Nr => r"\\".to_string(),
            Self::Meter(wire) => 
                format(format_args!(r#"\meter{{{wire}}}"#)),
            Self::Ctrl(wire) => 
                format(format_args!(r#"\ctrl{{{wire}}}"#)),
            Self::Targ => r"\targ{}".to_string(),
            Self::Control => r"\control{}".to_string(),
            Self::Swap(wire) => 
                format(format_args!(r#"\swap{{{wire}}}"#)),
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
    /// Display Fixed qubits in increasing numerical order.
    preserve_fixed_order: bool,
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
            /// false: preserve positional order 
            preserve_fixed_order: true,
        }
    }
}

// TODO: Implement functions to update the settings that allows the user customzie the rendering of the circuit.
impl Settings {
    fn label_qubit_lines(&self, key: String) -> String {
        Command::get_command(Command::Lstick(key.to_string()))
    }

    /// Sorts the order of the qubits as they are rendered on the Diagram.
    /// 
    /// # Arguments
    /// `mut self` - expose order field to sort
    fn preserve_fixed_order(&self, order: &mut Vec<String>) {
        order.sort();

        // TODO: Calculate target qubit for ctrl{targ}
    }
}

/// The structure of a LaTeX document. Typically a LaTeX document contains 
/// metadata defining the setup and packages used in a document within a header 
/// and footer while the body contains content and controls its presentation. 
struct Document {
    header: String,
    body: String,
    footer: String,
}

// TODO: Move TikZ/Quantikz into a separate struct. Keep Document abstract enough to represent any variant of LaTeX Documents.
impl Default for Document {
    fn default() -> Self {
        Self { 
            header:
r"\documentclass[convert={density=300,outext=.png}]{standalone}
\usepackage[margin=1in]{geometry}
\usepackage{tikz}
\usetikzlibrary{quantikz}
\begin{document}
\begin{tikzcd}".to_string(), 
            body: "".to_string(), 
            footer:
r"\end{tikzcd}
\end{document}".to_string(),
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.header, self.body, self.footer)
    }
}

/// A Diagram represents a collection of circuits. It encodes the relationships 
/// between the circuits and their positions or the row that it fills. A row is 
/// one of the Circuits in the HashMap. At this view over the circuits, Diagram 
/// can form a relationship between circuits based on information about the 
/// column and row. For example, one row, say qubit 0, at some column can hold 
/// information that it is the control. If another row, say qubit 1, at this 
/// same exact column says that it is the target, then it can inform qubit 0 
/// that it is controlling qubit 1. This information is then placed into the 
/// circuit as the diagram forms the equivalent LaTeX form for each qubit.
#[derive(Debug)]
struct Diagram {
    /// Settings
    settings: Settings,
    /// preserves the order of wires through indexing the circuit keys
    order: Vec<String>,
    /// n-1 columns for all wires
    column: u32,
    /// a HashMap of wires with the name of the wire as the key
    circuit: HashMap<String, Box<Wire>>,
}

impl Diagram {
    /// Returns a string indicating whether the qubit at row x column on the 
    /// wire is a control or target qubit. Using order, a qubit whose index = 0 
    /// is a control whereas index > 0, without modifiers, is a target.
    /// 
    /// # Arguments
    /// `&usize position` - the index of the qubit in &self.order
    fn set_ctrl_targ(&mut self, qubits: &Vec<Qubit>) {
        // cargo test --package quil-rs --lib --all-features -- program::latex::tests::test_to_latex --exact --nocapture

        // for each qubit in a single line of instructions
        for i in 0..qubits.len() {
            match self.circuit.get_mut(&qubits[i].to_string()) {
                Some(wire) => {
                    // the first qubit is the control
                    if i == 0 {
                        wire.ctrl.insert(self.column.clone(), true);
                        wire.targ.insert(self.column.clone(), false);
                    // all other qubits are the target
                    } else {
                        wire.ctrl.insert(self.column.clone(), false);
                        wire.targ.insert(self.column.clone(), true);
                    }

                    println!("{:?}", wire);
                },
                None => (),
            }
        }

        // if *position == 0 {
        //     // the target qubit lies on the next wire 1
        //     Command::get_command(Command::Ctrl(String::from("1")))
        // } else {
        //     Command::get_command(Command::Targ)
        // }

        
    }

    /// Takes a new or existing wire and adds or updates it using the name
    /// (String) as the key. If a wire exists with the same name, then the 
    /// contents of the new wire are added to it by updating the next column 
    /// using the Quantikz command associated with its attributes (e.g. gate, 
    /// do_nothing, etc).
    /// 
    /// # Arguments
    /// `&mut self` - exposes HashMap<String, Box<Circuit>>
    /// `wire` - the wire to be pushed or updated to in circuits
    fn push_wire(&mut self, wire: Wire) {
        // find wire in circuit collection
        match self.circuit.get_mut(&wire.name) {
            // wire found, push to existing wire
            Some(wire_in_circuit) => {
                // indicate a new item to be added by incrementing column
                self.column += 1;

                if let Some(gate) = wire.gates.get(&0) {
                    // add gates to wire in circuit
                    wire_in_circuit.gates.insert(self.column, gate.to_string());
                    }
            },
            // no wire found, preserve insertion order and insert new wire
            None => {
                self.order.push(wire.name.clone());
                self.circuit.insert(wire.name.clone(), Box::new(wire.clone()));
            },
        }

        // println!("{:?}", wire);
    }
}

impl Display for Diagram {
    /// Converts the Diagram Circuit to LaTeX string. Returns a Result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // add a newline between the first line and the header
        let mut body = String::from('\n');

        for i in 0..self.order.len() {
            // a single line of LaTeX representing a wire from the circuit   
            let mut line = String::from("");

            // are labels on in settings?
            if self.settings.label_qubit_lines {
                // add label to left side of wire
                line.push_str(&self.settings.label_qubit_lines(self.order[i].clone()));
            }



            // TODO: Change the way you identify what wire is a control and what is a target at column x row. Below should happen outside fof this format block. Maybe have Diagram add a flag to the Wire to indicate that at this point the Wire is a control or target. You should be able to sort the order and it not change the instruction positioning. Order should only be keeping track of the hashmap order or wires but with this in this block it seems to be keeping track of the positioning of the qubit in the instruction which is creating some confusion.

            // convert each attribute other than the default to string.
            if let Some(wire) = self.circuit.get(&self.order[i]) {
                for c in 0..=self.column {
                    if let Some(gate) = wire.gates.get(&c) {
                        // println!("GATE: {gate}");

                        line.push_str(" & ");

                        // determine if target or control
                        // if gate == "CNOT" {
                        //     line.push_str(&self.set_ctrl_targ(&i));
                        // } else {
                        line.push_str(&Command::get_command(Command::Gate(gate.to_string())));
                        // }
                    }
                }
            }

            // chain an empty column qw to the end of the line
            line.push_str(" & ");
            line.push_str(&Command::get_command(Command::Qw));

            // if this is the last key iteration, omit Nr from end of line
            if i < self.circuit.len() - 1 {
                // indicate a new row
                line.push(' ');
                line.push_str(&Command::get_command(Command::Nr));
            }

            // add a newline between each new line or the footer
            line.push('\n');
            body.push_str(&line);
        }

        write!(f, "{}", body)
    }
}

/// A circuit represents a single wire. A wire chains columns together of 
/// various Quantikz elements (using `&`). Encoded in each column is an index 
/// which determines the placement of the element on the circuit. Each column 
/// can hold only one element, therefore, each encoded index is unique between 
/// all of the attributes. Using this property, a String can be generated. 
#[derive(Debug, Clone)]
struct Wire {
    /// a wire, ket(qubit) placed using the Lstick or Rstick commands
    name: String,
    /// gate element(s) placed at column_u32 on wire using the Gate command
    gates: HashMap<u32, String>,
    /// defines at which column the qubit is a control
    ctrl: HashMap<u32, bool>,
    /// defines at which column the qubit is a target
    targ: HashMap<u32, bool>,
    /// a column_u32 that contains nothing placed using the Qw command  
    do_nothing: Vec<u32>,
}

impl Default for Wire {
    fn default() -> Self {
        Self { 
            name: String::from(""), 
            gates: HashMap::new(),
            ctrl: HashMap::new(),
            targ: HashMap::new(),
            do_nothing: vec![],
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LatexGenError {
    // TODO: Add variants for each error type using `thiserror` crate to return detailed Result::Err. Example error below.
    #[error("Tried to pop gate from new circuit and append to wire={wire} but found None.")]
    NoGateInInst{wire: String},
}

pub trait Latex {
    /// Returns a Result containing a quil Program as a LaTeX string.
    /// 
    /// # Arguments
    /// `settings` - Customizes the rendering of a circuit.
    fn to_latex(self, settings: Settings) -> Result<String, LatexGenError>;
}

impl Latex for Program {
    fn to_latex(self, settings: Settings) -> Result<String, LatexGenError> {
        // get a reference to the current program
        let instructions = Program::to_instructions(&self, false);

        // instruction
        // X 0, Y 1, 

        // store circuit strings
        let mut diagram = Diagram {settings, order: vec![], column: 0, circuit: HashMap::new()};

        for instruction in instructions {
            match instruction {
                // parse gate instructions into a new circuit
                instruction::Instruction::Gate(gate) => {
                    // println!("GATE: {:?}", gate.name);

                    // for each qubit in a single gate instruction
                    // TODO: Change to FIXED
                    for qubit in &gate.qubits {
                        // create a new wire
                        let mut wire = Wire::default();

                        // set name of wire for any qubit variant as String
                        // TODO: Change to int
                        wire.name = qubit.to_string().clone();

                        // add the gate to the next column on the wire
                        wire.gates.insert(diagram.column, gate.name.clone());

                        // push wire to diagram circuit
                        diagram.push_wire(wire);
                    }

                    if gate.name == "CNOT" {
                        // determine ctrl and targ qubits at the current column
                        diagram.set_ctrl_targ(&gate.qubits);
                    }
                },
                // do nothing for all other instructions
                _ => (),
            }
        }

        let body = diagram.to_string();
        let document = Document {body: body, ..Default::default()};
        println!("{}", document.to_string());

        Ok(document.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Settings, Latex};
    use crate::Program;
    use std::str::FromStr;

    /// Helper function takes instructions and return the LaTeX using the 
    /// Latex::to_latex method.
    pub fn get_latex(instructions: &str, settings: Settings) -> String {
        let program = Program::from_str(instructions).expect("program `{instructions}` should be returned");
        program
            .to_latex(settings)
            .expect("LaTeX should generate for program `{instructions}`")
    }

    #[test]
    /// Test functionality of to_latex using default settings.
    fn test_to_latex() {
        let program = Program::from_str("CNOT 0 1\nCNOT 2 3\nCNOT 1 0").expect("");
        program.to_latex(Settings::default()).expect("");
    }

    mod document {
        use crate::program::latex::{Document, tests::get_latex, Settings};

        #[test]
        fn test_template() {
            insta::assert_snapshot!(get_latex("", Settings::default()));
        }

        #[test]
        fn test_header() {
            let document = Document::default();
            insta::assert_snapshot!(document.header);
        }

        #[test]
        fn test_body_default() {
            let document = Document::default();
            insta::assert_snapshot!(document.body);
        }

        #[test]
        fn test_footer() {
            let document = Document::default();
            insta::assert_snapshot!(document.footer);
        }
    }

    mod gates {
        use crate::program::latex::{tests::get_latex, Settings};

        #[test]
        fn test_gate_x() {
            insta::assert_snapshot!(get_latex("X 0", Settings::default()));
        }

        #[test]
        fn test_gate_y() {
            insta::assert_snapshot!(get_latex("Y 1", Settings::default()));
        }

        #[test]
        fn test_gates_x_and_y_single_qubit() {
            insta::assert_snapshot!(get_latex("X 0\nY 0", Settings::default()));
        }

        #[test]
        fn test_gates_x_and_y_two_qubits() {
            insta::assert_snapshot!(get_latex("X 0\nY 1", Settings::default()));
        }

        #[test]
        fn test_gates_cnot_ctrl_0_targ_1() {
            insta::assert_snapshot!(get_latex("CNOT 0 1", Settings::default()));
        }

        #[test]
        fn test_gates_cnot_ctrl_1_targ_0() {
            let settings = Settings {preserve_fixed_order: false, ..Default::default()};
            insta::assert_snapshot!(get_latex("CNOT 1 0", settings));
        }

        #[test]
        fn test_gates_cnot_ctrl_1_targ_0_fixed_order() {
            insta::assert_snapshot!(get_latex("CNOT 1 0", Settings::default()));
        }

        // #[test]
        // fn test_gate_controlled() {
        //     insta::assert_snapshot!(get_latex("CONTROLLED H 3 2"));
        // }
    }

    /// Test module for command Operators
    mod commands {
        use crate::program::latex::Command;

        #[test]
        fn test_command_left_ket() {
            insta::assert_snapshot!(Command::get_command(Command::Lstick("0".to_string())));
        }

        #[test]
        fn test_command_right_ket() {
            insta::assert_snapshot!(Command::get_command(Command::Rstick("0".to_string())));
        }

        #[test]
        fn test_command_gate() {
            insta::assert_snapshot!(Command::get_command(Command::Gate("X".to_string())));
        }

        #[test]
        fn test_command_qw() {
            insta::assert_snapshot!(Command::get_command(Command::Qw));
        }

        #[test]
        fn test_command_nr() {
            insta::assert_snapshot!(Command::get_command(Command::Nr));
        }

        #[test]
        fn test_command_measure() {
            insta::assert_snapshot!(Command::get_command(Command::Meter("0".to_string())));
        }

        #[test]
        fn test_command_control() {
            insta::assert_snapshot!(Command::get_command(Command::Ctrl("0".to_string())));
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
            insta::assert_snapshot!(Command::get_command(Command::Swap("0".to_string())));
        }

        #[test]
        fn test_command_swap_target() {
            insta::assert_snapshot!(Command::get_command(Command::TargX));
        }
    }
}
