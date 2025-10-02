# INPROD_26: Command Output Serialization

## SEVERITY: MEDIUM

## OBJECTIVE
Include command data field in JSON output serialization instead of skipping it.

## LOCATION
- `packages/candle/src/domain/chat/commands/response.rs`

## CURRENT STATE
- Line 144: `// Note: CommandOutput doesn't have a data field, so we'll skip this for now`
- Data field is not serialized to JSON
- Output may be incomplete
- Serialization is not fully implemented

## SUBTASK 1: Add Data Field to CommandOutput
- Check if CommandOutput structure needs data field added
- If missing, add appropriate data field
- If present but not accessible, fix accessibility
- Determine proper type for data field

## SUBTASK 2: Serialize Data Field to JSON
- Locate response.rs:144
- Add data field to json_output
- Serialize command data appropriately
- Remove skip comment

## SUBTASK 3: Handle Different Data Types
- Support various output data formats
- Serialize structured data as JSON
- Handle binary or large output appropriately
- Include metadata about data type/format

## DEFINITION OF DONE
- [ ] Data field is included in CommandOutput if needed
- [ ] Data is serialized to JSON output
- [ ] Different data types are handled
- [ ] Skip comment is removed
- [ ] Serialization is complete

## RESEARCH NOTES
- Review CommandOutput structure definition
- Check what data should be included
- Examine JSON serialization patterns
- Look for data type handling in responses

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation
