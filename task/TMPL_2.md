# TMPL_2: Implement Nested Function Parsing (Legacy Parser)

## OBJECTIVE
Apply the same nested function parsing fix to the legacy parser, then evaluate deprecation strategy.

## LOCATION
`packages/candle/src/domain/chat/templates/parser.rs:650`

## SUBTASK 1: Apply TMPL_1 solution
- Implement the same parse_function_args solution from TMPL_1
- Handle parenthesis-aware parsing with depth tracking
- Track quote context for string literals

## SUBTASK 2: Add deprecation warnings
- Add doc comments indicating parser.rs is legacy
- Point users to parser_new.rs
- Add compile-time deprecation attribute if appropriate

## SUBTASK 3: Document migration path
- Add comments explaining migration to parser_new.rs
- Note: Do not create separate migration documentation files
- Keep migration notes as code comments only

## DEFINITION OF DONE
- Nested function calls parse correctly in legacy parser
- No "for now" comments remain
- Deprecation warnings added
- Code compiles without warnings

## RESEARCH NOTES
- TMPL_1 implementation (must be completed first)
- Rust deprecation attributes: #[deprecated]
- Existing parser.rs structure

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Do NOT create documentation files
- Focus solely on src modification
