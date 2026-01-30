// Test module for grammar parser
// Organized into separate files by feature area

#[cfg(test)]
mod helpers;

#[cfg(test)]
mod error_handling;

#[cfg(test)]
mod lexer;

#[cfg(test)]
mod precedence;

#[cfg(test)]
mod statements;

#[cfg(test)]
mod assignments;

#[cfg(test)]
mod imports;

#[cfg(test)]
mod evaluator_tests;
