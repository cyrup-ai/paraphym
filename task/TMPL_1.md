# TMPL_1: Implement Nested Function Parsing (New Parser)

## OBJECTIVE
Implement parenthesis-aware argument parsing to handle nested function calls in template expressions.

## LOCATION
`packages/candle/src/domain/chat/templates/parser.rs:655`

## SUBTASK 1: Implement parse_function_args with depth tracking
- Add parenthesis-aware parsing with depth tracking
- Track quote context to ignore parentheses in strings
- Handle nested function calls: `func(a, other(b, c), d)`

## SUBTASK 2: Implement the parsing logic
```rust
fn parse_function_args(&self, args_str: &str, depth: usize) -> Result<Vec<Expr>> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut paren_depth = 0;
    let mut in_string = false;
    
    for ch in args_str.chars() {
        match ch {
            '"' => in_string = !in_string,
            '(' if !in_string => paren_depth += 1,
            ')' if !in_string => paren_depth -= 1,
            ',' if !in_string && paren_depth == 0 => {
                args.push(self.parse_expression(current_arg.trim(), depth + 1)?);
                current_arg.clear();
                continue;
            }
            _ => {}
        }
        current_arg.push(ch);
    }
    
    if !current_arg.trim().is_empty() {
        args.push(self.parse_expression(current_arg.trim(), depth + 1)?);
    }
    
    Ok(args)
}
```

## SUBTASK 3: Replace simple comma-split
- Remove the "for now" comment
- Replace existing comma-split logic with new parse_function_args
- Ensure proper error handling

## DEFINITION OF DONE
- Nested function calls parse correctly
- No "for now" comments remain
- String literals with commas handled correctly
- Code compiles without warnings

## RESEARCH NOTES
- Parser combinator patterns
- Recursive descent parsing
- Existing Expr types in parser_new.rs

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on src modification
