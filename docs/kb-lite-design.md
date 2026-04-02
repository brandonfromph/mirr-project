---
title: KB-Lite Design
status: Active
---

# KB-Lite Design

KB-lite is a repository-local capability used for proposal and governance workflows.

## Stack
- Data plane: .kb-data/knowledge.lance and .kb-data/graph.db
- Governance plane: scripts/validate_proposals.py and scripts/repo_metrics.py
- Interface plane: mcp_server read and search surfaces

## Scope
- Validate presence and basic health of KB-lite storage
- Surface metrics for proposal and campaign observability
- Keep integration local-first and repository-scoped

## Non-Goals
- No new daemon requirement
- No platform migration in Proposal 096
- No policy gate beyond documented proposal acceptance commands
