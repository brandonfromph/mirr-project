# MCP Server API Specification

This document will contain sample requests, request/response schemas, and
examples for each MCP method once Phase 1 is underway.

## Authentication

All requests must include an `Authorization` header with a valid API token.
Tokens bear permissions that determine which endpoints are accessible.

## Common Schemas

(Placeholders for JSON Schema objects; eventually extracted from AJV source files.)

```json
{
  "$id": "https://nasa-rust-project/mcp/schemas/base.json",
  "type": "object",
  "properties": {
    "method": { "type": "string" },
    "params": { "type": "object" }
  },
  "required": ["method", "params"]
}
```

## Methods

### `read_file`

**Request example**

```json
{
  "method": "read_file",
  "params": { "path": "src/lib.rs" }
}
```

**Response schema**

```json
{
  "type": "object",
  "properties": {
    "content": { "type": "string" }
  },
  "required": ["content"]
}
```

*(Additional methods and details to be filled in as implementation progresses.)*
