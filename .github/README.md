# GitHub Actions CI/CD Setup for OpenMMO

This directory contains the complete GitHub Actions CI/CD pipeline for the OpenMMO project, designed to enforce the quality standards defined in `AGENTS.md`.

## Workflows Overview

### Main CI Pipeline
- **`ci-full.yml`**: Complete pipeline that runs all checks and enforces Definition of Done criteria
- **`ci-rust.yml`**: Rust server-specific checks (build, test, lint, security audit)
- **`ci-godot.yml`**: Godot client validation (project structure, syntax checks)
- **`ci-database.yml`**: Database migration testing and validation

### PR and Release Management
- **`pr-validation.yml`**: Enforces PR requirements from AGENTS.md
- **`release.yml`**: Automated release creation with Docker builds
- **`dependency-updates.yml`**: Automated dependency updates (optional)

## Quality Gates

### Definition of Done Enforcement
The pipeline enforces all DoD criteria from AGENTS.md:
- ✅ Code compiles without warnings
- ✅ Tests pass if any exist
- ✅ Follows coding standards (rustfmt, clippy)
- ✅ Documentation updates required for functional changes
- ✅ No unrelated changes in PRs
- ✅ Security audit passes
- ✅ No secrets committed

### Automated Checks
- **Rust Server**: Formatting, linting, building, testing, security audit
- **Godot Client**: Headless project validation, required scenes/presets, and core module smoke tests
- **Database**: Migration testing, rollback capability, schema validation
- **Code Quality**: Secret detection, file organization, documentation updates

## PR Requirements

### Branch Naming
- Must follow pattern: `feature/<short-description>`
- Direct pushes to master are blocked

### PR Template
All PRs must include:
- Summary of changes
- Related spec section
- Validation steps
- Type of change
- Testing checklist
- AGENTS.md compliance checklist

### Validation Steps
The pipeline automatically validates:
- PR description completeness
- Branch naming convention
- Unrestricted file changes
- Commit message quality
- Documentation updates
- Secret detection
- File size limits

## Security Features

### Automated Security Checks
- Dependency vulnerability scanning
- Secret detection in commits
- Database migration security validation
- Input validation enforcement

### Access Control
- Branch protection rules
- Required status checks
- PR review requirements
- Admin enforcement

## Release Process

### Automated Releases
- Triggered by git tags (`v*`)
- Creates GitHub releases with assets
- Builds and publishes Docker images
- Generates checksums for verification

### Release Assets
- Server binary
- Database migrations
- Documentation
- Client exports (if configured)
- SHA256 checksums

## Configuration Files

### Templates
- `pull_request_template.md`: Required PR template
- `bug_report.md`: Bug report issue template
- `feature_request.md`: Feature request template
- `spec_change.md`: Specification change request template

### Documentation
- `BRANCH_PROTECTION.md`: Branch protection rules
- This README: CI/CD setup documentation

## Setup Instructions

### 1. Enable Branch Protection
Configure master branch protection as outlined in `BRANCH_PROTECTION.md`.

### 2. Required Secrets
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions
- No additional secrets required for basic CI/CD

### 3. Optional Integrations
- Docker registry for container publishing
- Additional security scanning tools
- Custom notification systems

## Troubleshooting

### Common Issues
1. **Rust formatting fails**: Run `cargo fmt` locally and commit changes
2. **Clippy warnings**: Fix warnings locally with `cargo clippy --fix`
3. **Godot validation fails**: Check project structure and scene files
4. **Database tests fail**: Ensure migrations are properly formatted

### Debugging
- Check workflow logs in GitHub Actions tab
- Run workflows manually with `workflow_dispatch`
- Use `act` for local GitHub Actions testing

## Maintenance

### Regular Updates
- Review and update action versions
- Update Rust toolchain versions
- Update Godot version in workflows
- Review security scanning rules

### Monitoring
- Monitor workflow success rates
- Check for flaky tests
- Review security scan results
- Update dependency update schedules

## Compliance

This CI/CD setup ensures compliance with:
- AGENTS.md guidelines
- Definition of Done criteria
- Security best practices
- Code quality standards
- Documentation requirements

All automated contributions must pass these checks before merging, ensuring consistent, high-quality code that follows the project's architectural standards.
