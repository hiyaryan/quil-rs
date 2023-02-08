use std::fmt::format;

use crate::Program;
use latex::{print, Document, DocumentClass, Element, Section, Align};

#[derive(Debug)]
/// Settings to control the layout and rendering of circuits.
pub struct DiagramSettings {
    /// Convert numerical constants, such as pi, to LaTeX form.
    texify_numerical_constants: bool,

    /// Include qubits with indices between those explicitly referenced in the Quil program.
    /// For example, if true, the diagram for `CNOT 0 2` would have three qubit lines: 0, 1, 2.
    impute_missing_qubits: bool,

    /// Label qubit lines.
    label_qubit_lines: bool,

    /// Write controlled rotations in a compact form.
    /// For example,  `RX(pi)` as `X_{\\pi}`, instead of the longer `R_X(\\pi)`
    abbreviate_controlled_rotations: bool,

    /// The length by which qubit lines should be extended with open wires at the right of the diagram.
    /// The default of 1 is the natural choice. The main reason for including this option
    /// is that it may be appropriate for this to be 0 in subdiagrams.
    qubit_line_open_wire_length: u32,

    /// Align measurement operations which appear at the end of the program.
    right_align_terminal_measurements: bool,
}

impl Default for DiagramSettings {
    fn default() -> Self {
        Self {
            texify_numerical_constants: true,
            impute_missing_qubits: false,
            label_qubit_lines: true,
            abbreviate_controlled_rotations: false,
            qubit_line_open_wire_length: 1,
            right_align_terminal_measurements: true,
        }
    }
}

pub enum TikzOperator {
    TikzLeftKet(u32),
    TikzControl(i32),
    TikzCnotTarget,
    TikzCphaseTarget,
    TikzSwap(i32),
    TikzSwapTarget,
    TikzNop,
    TikzMeasure,
}

impl TikzOperator {
    fn get_tikz_operator(tikz_operator: Self) -> String {
        match tikz_operator {
            Self::TikzLeftKet(qubit) => format(format_args!(r#"\lstick{{\ket{{q_{{{qubit}}}}}}}"#)), // \lstick{\ket{q_{qubit}}}
            Self::TikzControl(offset) => format(format_args!(r#"\ctrl{{{offset}}}"#)), // \ctrl{offset}
            Self::TikzCnotTarget => r"\targ{}".to_string(), // \targ{}
            Self::TikzCphaseTarget => r"\control{}".to_string(), // \control{}
            Self::TikzSwap(offset) => format(format_args!(r"\swap{{{offset}}}")), // \swap{offset}
            Self::TikzSwapTarget => r"\targX{}".to_string(), // \targX{}
            Self::TikzNop => r"\qw".to_string(),              // \qw
            Self::TikzMeasure => r"\meter{}".to_string(),     // \meter{}
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LatexError {
    // TODO: Add variants for each error type using `thiserror` crate to return detailed Result::Err.
    #[error("This is an error on {qubit_index}.")]
    SomeError { qubit_index: u32 },
}

pub trait Latex {
    fn to_latex(self, diagram_settings: DiagramSettings) -> Result<String, LatexError>;
    fn build_1q_unitary(self);
}

impl Latex for Program {
    fn to_latex(self, diagram_settings: DiagramSettings) -> Result<String, LatexError> {
        // Using the latex crate to build the LaTeX document object..
        // Set the header
        let mut doc = Document::new(DocumentClass::Other("standalone".to_string()));
        doc.push(Element::Input("".to_string()));

        // doc.preamble.use_package("tikz");

        // Packages 
        // r"\documentclass[convert={density=300,outext=.png}]{standalone}",
        // r"\usepackage[margin=1in]{geometry}",
        // r"\usepackage{tikz}",
        // r"\usetikzlibrary{quantikz}",

        // Initialization
        // r"\begin{document}", r"\begin{tikzcd}"


        // Build the body
        
        
        // Set the footer
        // "\\end{tikzcd}\n\\end{document}"

        println!("{}", print(&doc).unwrap());

        // TODO: Generate the Program LaTeX.
        let latex = "";
        Ok(latex.to_string())
    }


    // TODO: Implement build_1q_unitary function that builds single qubit X and Y gates.
    fn build_1q_unitary(self) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagramSettings, Latex};
    use crate::Program;
    use std::str::FromStr;

    /// Take an instruction and return the LaTeX using the to_latex method.
    pub fn get_latex(instructions: &str) -> String {
        let program = Program::from_str(instructions).expect("Program should be returned.");
        program
            .to_latex(DiagramSettings::default())
            .expect("LaTeX should generate without error.")
    }

    #[test]
    /// Test functionality of to_latex using default settings.
    fn test_to_latex() {
        let program = Program::from_str("").expect("");
        program.to_latex(DiagramSettings::default()).expect("");
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

    mod tikz_operators {
        use crate::program::latex::TikzOperator;

        #[test]
        fn test_tikz_left_ket() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzLeftKet(0)));
        }

        #[test]
        fn test_tikz_control() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzControl(2)));
        }

        #[test]
        fn test_tikz_cnot_target() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzCnotTarget));
        }

        #[test]
        fn test_tikz_cphase_target() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzCphaseTarget));
        }

        #[test]
        fn test_tikz_swap() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzSwap(4)));
        }

        #[test]
        fn test_tikz_swap_target() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzSwapTarget));
        }

        #[test]
        fn test_tikz_nop() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzNop));
        }

        #[test]
        fn test_tikz_measure() {
            insta::assert_snapshot!(TikzOperator::get_tikz_operator(TikzOperator::TikzMeasure));
        }
    }
}
