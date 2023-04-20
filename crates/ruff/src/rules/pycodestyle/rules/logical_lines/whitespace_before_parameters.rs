use crate::checkers::logical_lines::LogicalLinesContext;
use crate::rules::pycodestyle::rules::logical_lines::LogicalLine;
use ruff_diagnostics::{AlwaysAutofixableViolation, Diagnostic, Edit};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::token_kind::TokenKind;
use ruff_text_size::{TextRange, TextSize};
use rustpython_parser::Tok;

#[violation]
pub struct WhitespaceBeforeParameters {
    pub bracket: TokenKind,
}

impl WhitespaceBeforeParameters {
    fn bracket_text(&self) -> char {
        match self.bracket {
            TokenKind::Lpar => '(',
            TokenKind::Lsqb => '[',
            _ => unreachable!(),
        }
    }
}

impl AlwaysAutofixableViolation for WhitespaceBeforeParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        let bracket = self.bracket_text();
        format!("Whitespace before '{bracket}'")
    }

    fn autofix_title(&self) -> String {
        let bracket = self.bracket_text();
        format!("Removed whitespace before '{bracket}'")
    }
}

/// E211
pub(crate) fn whitespace_before_parameters(
    line: &LogicalLine,
    autofix: bool,
    context: &mut LogicalLinesContext,
) {
    let previous = line.first_token().unwrap();

    let mut pre_pre_kind: Option<&Tok> = None;
    let mut prev_token = previous.token();
    let mut prev_end = previous.end();

    for token in line.tokens() {
        let kind = token.token();

        if matches!(kind, Tok::Lpar | Tok::Lsqb)
            && matches!(
                prev_token,
                Tok::Name { .. } | Tok::Rpar | Tok::Rsqb | Tok::Rbrace
            )
            && !matches!(pre_pre_kind, Some(Tok::Class))
            && token.start() != prev_end
        {
            let start = prev_end;
            let end = token.end() - TextSize::from(1);
            let kind: WhitespaceBeforeParameters = WhitespaceBeforeParameters {
                bracket: TokenKind::from_token(kind),
            };

            let mut diagnostic = Diagnostic::new(kind, TextRange::new(start, end));

            if autofix {
                diagnostic.set_fix(Edit::deletion(start, end));
            }
            context.push_diagnostic(diagnostic);
        }
        pre_pre_kind = Some(prev_token);
        prev_token = kind;
        prev_end = token.end();
    }
}
