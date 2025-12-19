# Branch Protection Rules for OpenMMO

This document outlines the branch protection rules that should be configured for the repository.

## Master Branch Protection

### Required Status Checks
All of the following must pass before merging to master:

- **CI - Full Pipeline** (definition-of-done)
  - Rust Server
  - Godot Client  
  - Database Migrations
  - Code Quality Gates

### Required Pull Request Reviews
- **Required approving reviews**: 1
- **Dismiss stale PR approvals when new commits are pushed**: Yes
- **Require review from Code Owners**: Yes
- **Restrict reviews from collaborators**: No
- **Limit to users with dismiss stale review approvals**: Yes

### Enforce Admins
- **Include administrators**: Yes (admins must also follow these rules)

### Restrictions
- **Restrict pushes that create matching branches**: Yes
- **Allow force pushes**: No
- **Allow deletions**: No

## Additional Rules

### Pull Request Requirements
- PRs must follow the template in `.github/pull_request_template.md`
- PRs must be from feature branches (pattern: `feature/*`)
- PRs must include required sections: Summary, Validation
- PRs must pass all automated checks

### Code Quality Gates
- Code must compile without warnings
- All tests must pass
- Code must follow formatting standards (rustfmt)
- Code must pass linting (clippy)
- No secrets or sensitive data in commits
- Documentation must be updated for functional changes

### Security Requirements
- Security audit must pass
- No known vulnerabilities in dependencies
- Database migrations must be tested
- Input validation must be maintained

## Implementation

These rules can be configured in GitHub repository settings:

1. Go to Settings > Branches
2. Add branch protection rule for `master`
3. Configure the settings as outlined above
4. Save the rule

Alternatively, use GitHub API or GitHub CLI to automate the configuration.

## Automation

The workflows in `.github/workflows/` enforce these rules automatically:

- `ci-full.yml`: Runs all quality gates
- `pr-validation.yml`: Validates PR requirements
- `ci-rust.yml`: Rust-specific checks
- `ci-godot.yml`: Godot-specific checks
- `ci-database.yml`: Database migration checks

## Compliance

These rules ensure compliance with AGENTS.md requirements:

- ✅ Enforce coding standards (rustfmt, clippy)
- ✅ Ensure code compiles without warnings
- ✅ Tests pass if any exist
- ✅ Documentation updates required
- ✅ No unrelated changes in PRs
- ✅ Feature branch naming convention
- ✅ Pull request template requirements
- ✅ Security and quality gates
