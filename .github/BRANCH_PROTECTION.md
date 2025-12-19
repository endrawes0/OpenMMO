# Branch Protection Rules for OpenMMO

This document reflects the branch protection settings currently configured on GitHub and highlights known gaps with the intended standards in `AGENTS.md`.

## Master Branch Protection (current state)

### Required Status Checks
Branch protection currently requires the following contexts to report before merge:

- `Rust Server`
- `Godot Client`
- `Database Migrations`
- `Code Quality Gates`
- `Definition of Done Check`
- `Validate PR Requirements`

### Required Pull Request Reviews
- Required approving reviews: **0**
- Dismiss stale approvals: **Disabled**
- Require code owner reviews: **No**
- Require approval from last pusher: **No**

### Admin Enforcement
- Include administrators: **No**

### Other Settings
- Require status checks to pass before merging: **Yes** (with strict = true)
- Require linear history: **No**
- Require conversation resolution: **No**
- Allow force pushes: **No**
- Allow deletions: **No**
- Block branch creations: **No**
- Allow fork syncing: **No**

## Additional Notes
- The required status check names now match the workflow job contexts emitted by the CI runs, resolving the pending-checks issue in #25.
- If you want to move toward the stricter policy described in `AGENTS.md`, update the GitHub branch protection rule to require at least one approval (and code owners/admin enforcement).
