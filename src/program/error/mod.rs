// Copyright 2021 Rigetti Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod leftover;
mod result;
mod syntax;

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::instruction::Instruction;
use crate::parser::{LexError, ParseError};
pub use leftover::LeftoverError;
pub use result::{disallow_leftover, map_parsed, recover, convert_leftover};
pub use syntax::SyntaxError;

#[derive(Debug, PartialEq)]
pub enum ProgramError<T> {
    InvalidCalibration {
        instruction: Instruction,
        message: String,
    },
    RecursiveCalibration(Instruction),
    Syntax(SyntaxError),
    Leftover(LeftoverError<T>),
}

impl<T> From<LexError> for ProgramError<T>
where
    T: fmt::Debug,
{
    fn from(e: LexError) -> Self {
        Self::Syntax(SyntaxError::from(e))
    }
}

impl<T> From<ParseError> for ProgramError<T> {
    fn from(e: ParseError) -> Self {
        Self::Syntax(SyntaxError::from(e))
    }
}

impl<T> From<LeftoverError<T>> for ProgramError<T> {
    fn from(err: LeftoverError<T>) -> Self {
        Self::Leftover(err)
    }
}

impl<T> ProgramError<T> {
    pub fn map_parsed<T2>(self, map: impl Fn(T) -> T2) -> ProgramError<T2> {
        match self {
            Self::InvalidCalibration { instruction, message } => ProgramError::InvalidCalibration { instruction, message },
            Self::RecursiveCalibration(inst) => ProgramError::RecursiveCalibration(inst),
            Self::Syntax(err) => ProgramError::Syntax(err),
            Self::Leftover(err) => ProgramError::Leftover(err.map_parsed(map)),
        }
    }
}

impl<T> fmt::Display for ProgramError<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCalibration {
                instruction,
                message,
            } => write!(f, "invalid calibration `{}`: {}", instruction, message),
            Self::RecursiveCalibration(instruction) => {
                write!(f, "instruction {} expands into itself", instruction)
            }
            Self::Syntax(err) => fmt::Display::fmt(err, f),
            Self::Leftover(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl<T> Error for ProgramError<T>
where
    T: fmt::Debug + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidCalibration { .. } => None,
            Self::RecursiveCalibration(_) => None,
            Self::Syntax(err) => Some(err),
            Self::Leftover(err) => Some(err),
        }
    }
}
