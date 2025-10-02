# INPROD_23: Text Extraction Robustness

## SEVERITY: MEDIUM

## OBJECTIVE
Implement robust text extraction and markdown highlighting instead of simple character-by-character parsing.

## LOCATION
- `packages/sweetmcp/plugins/fetch/src/lib.rs`

## CURRENT STATE
- Line 500: `// Simple text extraction - in a real implementation this would be more robust`
- Line 550: `// Simple markdown highlighting (in a real implementation this would be more sophisticated)`
- Text extraction uses basic tag detection with in_tag flag
- Markdown highlighting just returns content unchanged

## SUBTASK 1: Implement Robust HTML Text Extraction
- Locate fetch/src/lib.rs:500
- Use proper HTML parsing library (html5ever, scraper, etc.)
- Extract text while preserving meaningful whitespace
- Handle nested tags, entities, and special characters
- Strip scripts, styles, and other non-content elements

## SUBTASK 2: Implement Markdown Syntax Highlighting
- Locate fetch/src/lib.rs:550
- Parse markdown syntax elements (headers, bold, italic, code, etc.)
- Apply ANSI color codes or other highlighting
- Handle code blocks with language-specific highlighting if possible
- Preserve markdown structure while adding visual enhancement

## SUBTASK 3: Handle Edge Cases
- Handle malformed HTML gracefully
- Process incomplete markdown correctly
- Handle mixed content (HTML in markdown, etc.)
- Support various character encodings

## DEFINITION OF DONE
- [ ] HTML text extraction uses proper parser
- [ ] Markdown highlighting is implemented
- [ ] Edge cases are handled
- [ ] Simple character-by-character parsing is replaced
- [ ] Stub comments removed

## RESEARCH NOTES
- Review available HTML parsing libraries in dependencies
- Check for markdown parsing crates (pulldown-cmark, etc.)
- Examine ContentFormat enum and usage
- Look for existing syntax highlighting utilities

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation
