# Multi-Model Decision Protocol

The Orchestrator uses **multi-model decision analysis** to resolve non-trivial technical choices. This is the autonomous decision-making process — distinct from the interactive brainstorming skill.

## How It Works

The Orchestrator launches ALL available Researcher variants **in parallel** with the same question. Each returns an independent recommendation. The Orchestrator synthesizes results and presents the agreement/disagreement breakdown to the user.

## When to Use (Auto-Trigger Rules)

Trigger the decision protocol when there is an **unresolved non-trivial technical decision** after requirements are understood:
- Architecture or infrastructure decisions with multiple viable approaches
- Data model, schema, or storage strategy choices
- Technology or library selection
- Trade-offs where the "right" answer isn't obvious
- When a sub-agent returns a recommendation that has alternatives

**Do NOT use for:** Requirements discovery, user intent clarification, or feature scoping — those belong to the brainstorming skill.

## Key Rules

- Always launch in **parallel**, minimum 4 variants
- Use exact case-sensitive agent names — never rename or alias
- Never make a non-trivial technical decision without multi-model analysis
- **Produce an ADR** after every decision resolution
- `remember` the decision for future recall

