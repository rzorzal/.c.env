// Literal parsing - strings, templates, numbers, booleans

use crate::lexing;
use crate::grama::gramma_rules::{Expr, TemplatePart};
use crate::grama::error::{ParseError, ParseResult};

pub(super) fn parse_string_template(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // String template parsing
  // This is a simplified implementation - in a real parser you'd need to handle
  // the complex parsing of the template with expressions inside

  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty template", 0));
  }

  // For now, we'll just handle a basic template
  if let lexing::TokenType::StringTemplate(content) = &tokens[0].token_type {
    // Here we would parse the content to find expressions inside the template
    // For now, we'll just create a simple template with text parts

    let parts = vec![
      TemplatePart::Text(content.clone())
    ];

    return Ok(Expr::Template(parts));
  }

  Err(ParseError::unexpected_token(
    "string template",
    &format!("{:?}", tokens[0].token_type),
    tokens[0].start
  ))
}
