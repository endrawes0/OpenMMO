# AGENTS.md  
Guidelines and Workflow Requirements for Coding Agents

This document defines how automated coding agents MUST operate within this repository.  
All contributions, regardless of agent, MUST follow these rules unless explicitly instructed otherwise.

---

## 1. General Principles

1. **Spec is Source of Truth**  
   Agents MUST adhere strictly to the architectural specification in `open_mmorpg_mvp_specification.md`.  
   - If any task appears ambiguous, agents MUST request clarification rather than assume.

2. **No Unapproved Dependencies**  
   Agents MUST NOT introduce new external libraries unless explicitly permitted by the spec.  
   - For Rust: permitted stacks include Tokio, Axum, SQLx, Prost, tracing.  
   - For Godot: ONLY built-in Godot packages and officially supported addons.

3. **Reproducibility First**  
   All build steps, code, and assets MUST be deterministic and runnable via documented commands without local modification.

---

## 2. Branching and PR Workflow

1. **Feature Branches Required**  
   Agents MUST create a new branch for each task using the naming pattern: feature/<short-description>

2. **Mandatory Pull Requests**  
Agents MUST open a PR targeting `master` for every change.  
Agents MUST NOT push directly to `master`.

3. **PR Requirements**  
Every PR MUST include:
- A brief description of the work
- The related spec section or task ID
- Validation steps (how reviewers can run or test it)
- A diff that touches ONLY files required for the specific task

4. **Atomicity**  
Agents MUST keep PRs small, self-contained, and focused on a single logical unit of progress.

---

## 3. Coding Standards

### 3.1 Rust (Server)

Agents MUST follow:

- `rustfmt` and `clippy` with default or repository-provided configs
- Tokio async runtime conventions
- SQLx compile-time checked queries
- Protobuf definitions via `.proto` → Prost generation

Agents MUST ensure:

- No blocking calls in async contexts  
- Error handling uses `thiserror` or idiomatic Rust error enums  
- Logging uses `tracing` with structured fields

### 3.2 Godot (Client)

Agents MUST follow:

- Godot 4.x project structure  
- Engine-agnostic game logic stored outside Godot scripts  
- GDScript or C# ONLY for UI binding, scene setup, and engine-specific code  
- No business logic or protocol logic inside Godot scripts

### 3.3 Protobuf

All `.proto` files MUST:

- Use snake_case for fields  
- Use versioned package names  
- Maintain backward compatibility  
- Be updated ONLY in PRs labeled `[proto-change]`

---

## 4. Testing Requirements

### 4.1 Server Testing

Agents MUST write:

- Unit tests for core simulation logic (combat, movement validation, ability cooldown logic)
- Unit tests for database access through SQLx query macros when feasible

Integration tests MAY be added but are not required in MVP.

### 4.2 Client Testing

Automated testing on the client is optional for MVP.  
Agents MUST ensure the game is runnable after any client change.

---

## 5. Documentation Requirements

Every agent-submitted PR MUST update or create documentation in:

- `docs/`  
- `SPEC.md` (if spec changes are approved)  
- Comments in code where non-obvious logic exists

Agents MUST NOT leave undocumented architectural changes.

---

## 6. Security and Safety Rules

1. **No Plaintext Secrets**  
Agents MAY NOT commit passwords, tokens, or hardcoded secrets.

2. **Password Handling**  
Agents MUST use Argon2 or bcrypt per spec.

3. **Input Validation**  
Agents MUST treat all client input as untrusted:
- Movement must be validated  
- Ability usage must be checked  
- Inventory operations must be server-authoritative

4. **No Sensitive Data in Logs**
Logs MUST NOT contain passwords, session tokens, or PII beyond usernames.

5. **Security Vulnerabilities**
Agents MUST NOT add security vulnerabilities to audit ignore lists without explicit user confirmation. All security decisions regarding known vulnerabilities must be approved by a human maintainer.

---

## 7. Asset Workflow Rules

Agents MUST:

- Place all assets under `assets/` with clear licensing metadata files
- Use only CC0, CC BY, or explicitly spec-approved assets
- NEVER import assets without attribution when required

---

## 8. When Agents Must Ask for Confirmation

Agents MUST request approval before:

- Introducing new dependencies  
- Altering protocol message definitions  
- Modifying database schema outside scheduled migrations  
- Adding new engine-specific abstractions  
- Changing tick rate or network update frequency  
- Changing repository structure  
- Implementing speculative features not present in the spec

If unsure, agents MUST ASK.

---

## 9. Task Execution Hierarchy

Agents MUST follow the project phases in order:

1. Phase 0 – Infrastructure setup  
2. Phase 1 – Networking + protocol  
3. Phase 2 – Core simulation  
4. Phase 3 – Persistence + accounts  
5. Phase 4 – Core MMO systems  
6. Phase 5 – Second zone + content  
7. Phase 6 – Admin tools + packaging

Agents MUST NOT skip ahead unless explicitly authorized.

---

## 10. Definition of Done (DoD)

A task is complete ONLY when:

- Code compiles without warnings  
- Tests (if any) pass  
- Behavior matches the spec exactly  
- Documentation is updated  
- PR is opened following all rules  
- No unrelated changes are included

---

## 11. Final Rule: Agents Must Work Like Skilled Teammates

Agents MUST:

- Write clear, maintainable code  
- Avoid unnecessary abstraction  
- Follow the architecture, not invent parallel systems  
- Communicate when assumptions are unclear  
- Treat the spec as a living contract  

If requirements conflict, agents MUST escalate by asking for human clarification.

---

This AGENTS.md governs all automated contributions and ensures the project remains consistent, maintainable, and aligned with the long-term MMORPG framework vision.
