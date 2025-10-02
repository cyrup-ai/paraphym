# INPROD_5: Template System Implementation

## SEVERITY: HIGH

## OBJECTIVE
Implement full template parsing, compilation, and evaluation. The entire template system is currently non-functional with simplified stubs.

## LOCATION
- `packages/candle/src/domain/chat/templates/parser.rs`
- `packages/candle/src/domain/chat/templates/compiler.rs`

## CURRENT STATE
- parser.rs:179: Conditional parsing just returns variable
- parser.rs:189: Loop parsing just returns variable
- parser.rs:223: Expression parsing returns as variable
- compiler.rs:52: Compilation creates simple text AST without actual parsing

## SUBTASK 1: Implement Conditional Parsing
- Locate parser.rs:179 `parse_conditional` method
- Parse if/else/endif structures properly
- Build proper Conditional AST nodes with condition evaluation
- Support comparison operators (==, !=, <, >, etc.)

## SUBTASK 2: Implement Loop Parsing
- Locate parser.rs:189 `parse_loop` method
- Parse for/in/endfor structures properly
- Build proper Loop AST nodes with iteration logic
- Handle loop variables and collections

## SUBTASK 3: Implement Expression Parsing
- Locate parser.rs:223 `parse_expression` method
- Parse mathematical and logical expressions
- Support operators: +, -, *, /, ==, !=, &&, ||
- Build proper Expression AST nodes

## SUBTASK 4: Implement Full Template Compilation
- Locate compiler.rs:52 `compile` method
- Actually parse the template content into a proper AST
- Don't just wrap content in TemplateAst::Text
- Use the parsing methods from parser.rs

## DEFINITION OF DONE
- [ ] Conditionals are parsed and evaluated correctly
- [ ] Loops are parsed and iterated correctly
- [ ] Expressions are parsed and calculated correctly
- [ ] Compilation produces proper AST from template content
- [ ] All stub comments removed
- [ ] Templates can be rendered with variables, conditions, and loops

## RESEARCH NOTES
- Review TemplateAst enum and all variant structures
- Examine variable resolution in resolve_variables_sync
- Check for existing operator precedence or evaluation logic
- Review ChatTemplate structure and expected content format

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation
