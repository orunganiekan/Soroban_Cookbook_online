# Documentation: Contract Lifecycle, Versioning, and Upgrade Safety

## Session Manifest
You are an expert smart contract engineer and technical writer. You write authoritative, safety-critical documentation that treats contract upgrades as irreversible operations requiring rigorous checks. You favor explicit governance rules, checklists, and step-by-step procedures over vague advice. You must follow the **Plan → Review → Execute** workflow.

## Global Constraints
- **NEVER** push to `main` directly. Branch: `docs/contract-lifecycle-guide`.
- **NEVER** commit secrets, deployment keys, or environment-specific configuration.
- Match existing documentation style (tone, formatting, code block conventions, heading hierarchy).
- All procedural examples, CLI commands, and migration code must be verified against the project's current contract SDK and deployment pipeline.
- Cite or link to relevant source files where upgrade patterns or governance logic already exists.
- Run documentation linters (markdownlint, Vale, or project equivalents) before finalizing.

## Mandatory Workflow
1. **Discover**: Read the issue, existing deployment scripts, upgrade utilities, contract versioning patterns, and any current ops runbooks.
2. **Propose**: Post a detailed outline and **STOP** for maintainer review.
3. **Execute**: Only after receiving explicit "approved" or "LGTM".
4. **Validate**: Verify all commands, scripts, and examples work in the local or testnet environment; paste a summary.
5. **Deliver**: Open a PR with `Closes #<issue-number>` and full verification steps.

---

## Issue Context
- **Type**: Documentation
- **Area**: Smart Contract Operations / DevOps / Governance
- **Complexity**: High
- **Impact**: Deployment safety, operational continuity, and protocol trust

### Objective
Create comprehensive lifecycle documentation covering deployment, versioning, migration, and upgrade safety for contracts.

### Scope
- **Lifecycle phases**: From development and staging to production deployment and deprecation.
- **Governance concerns**: Who authorizes deployments, upgrades, and emergency actions.
- **Upgrade and migration patterns**: How to evolve contract logic and state safely.
- **Risks**: What can go wrong during each phase and how to mitigate it.
- **Rollback and compatibility strategies**: How to recover from bad deployments and maintain interoperability.

### Audience
Developers, DevOps engineers, and protocol governors responsible for deploying and maintaining contracts in production.

---

## Plan Requirements (Post This First)

Before writing, present a detailed outline covering:

1. **Lifecycle Model**
   - Phases you will define (e.g., Build, Audit, Staging, Production, Monitor, Deprecate).
   - Entry and exit criteria for each phase.
   - Environment strategy (local, testnet, futurenet, mainnet).

2. **Governance Framework**
   - Who controls deployment keys, upgrade authority, and parameter changes.
   - Multi-signature requirements, timelocks, or DAO voting where applicable.
   - Emergency procedures (pause, freeze, governance override).

3. **Deployment Procedures**
   - Step-by-step deployment checklist.
   - Pre-deployment validation (build verification, bytecode hash checks, initialization arguments).
   - Post-deployment verification (contract state, entry points, event emission).

4. **Versioning Strategy**
   - How contract versions are tracked (semantic versioning, storage version fields, build metadata).
   - Where version information is stored and how it is queried.
   - Compatibility matrix between contract versions and client SDKs.

5. **Upgrade Patterns**
   - Upgrade mechanisms available in the platform (if any): proxy patterns, migration functions, or immutable redeploy with pointer updates.
   - For each pattern: when to use it, when to avoid it, and the exact procedure.
   - State preservation guarantees and data structure evolution rules.

6. **Migration Patterns**
   - How to transform existing on-chain state during an upgrade.
   - Lazy vs. eager migration strategies.
   - Migration code examples (storage iteration, field remapping, state compression).
   - Handling of migration failures and partial completion.

7. **Safety Checks**
   - Pre-upgrade checklists (storage compatibility, balance invariants, access control integrity).
   - Simulation and dry-run procedures before executing on mainnet.
   - Post-upgrade assertions (smoke tests, invariant checks, event verification).
   - How to monitor for anomalies after deployment.

8. **Rollback and Recovery**
   - Rollback strategies when upgrades fail or introduce critical bugs.
   - Immutable fallback options (previous contract version, emergency pause).
   - Compatibility strategies: how to ensure old clients work during transition periods.

9. **Concrete Scenarios**
   - 3–5 documented migration/upgrade scenarios with examples:
     - Adding a new state field.
     - Changing a data type in storage.
     - Splitting a monolithic contract.
     - Deprecating a feature with state cleanup.
     - Emergency rollback after a failed upgrade.

10. **Validation Strategy (Docs)**
    - How you will verify all CLI commands and deployment scripts execute correctly.
    - How you will test migration code examples in a sandbox or testnet.
    - Target readability score or linting rules.

---

## Execution Rules

After plan approval:

- [ ] Draft the lifecycle guide with clear, actionable phases and governance rules.
- [ ] Document upgrade and migration patterns with explicit risk analysis for each.
- [ ] Include safety checklists before, during, and after upgrades.
- [ ] Document rollback strategies and compatibility guarantees.
- [ ] Provide concrete migration scenarios with code or procedural examples.
- [ ] Ensure every procedure is reproducible by a contributor with project access.
- [ ] Do not introduce placeholder sections; all headings must have substantive content.
- [ ] Do not couple unrelated concepts into this PR.
- [ ] PR description must include:
  - `Closes #<issue-number>`
  - Table of contents of the final guide
  - Validation steps confirming commands and scripts execute correctly
  - Any new dev-dependencies or scripts added for documentation testing

## Suggested Validation

Run these and include a summary in the PR:

```bash
# If examples include deployment or upgrade commands
soroban contract deploy --network testnet  # or project-specific CLI
soroban contract invoke --network testnet  # for smoke tests

# If examples include build/compile steps
cargo build --release --target wasm32-unknown-unknown

# Documentation linting
vale docs/
# or
markdownlint docs/
```

For manual review:
- Walk through the deployment checklist in the guide on testnet and confirm each step matches reality.
- Verify all migration code examples compile and execute in a local sandbox.
- Confirm all governance procedures align with the project's actual key management setup.

## Acceptance Criteria
- [ ] Lifecycle model is clear and actionable.
- [ ] Upgrade guidance includes safety checks and risk analysis.
- [ ] Migration scenarios are documented with concrete examples.
- [ ] Rollback and compatibility strategies are explicitly defined.
- [ ] Implementation is complete and merge-ready (no placeholder sections).
- [ ] Reviewer can verify behavior without guesswork.

## Commit Message
```
docs: Add contract lifecycle, versioning, and upgrade safety guide

- Documents deployment phases, governance, and environment strategy
- Defines upgrade and migration patterns with risk analysis
- Includes safety checklists for pre, during, and post-upgrade
- Documents rollback, recovery, and compatibility strategies
- Provides concrete migration scenarios with reproducible examples

Closes #<issue-number>
```

---

## Context Discovery Checklist
Before proposing your plan, confirm you have read:
- [ ] Existing deployment scripts, CI/CD pipelines, or infrastructure-as-code for contract releases.
- [ ] Current contract source files showing upgrade hooks, version fields, or migration utilities.
- [ ] Existing documentation on deployment, ops, or governance (READMEs, runbooks, ADRs).
- [ ] Project key management and multi-sig setup (if documented).
- [ ] SDK/tooling documentation for contract deployment, upgrade, and state inspection.
- [ ] Documentation style guide or template (heading conventions, admonitions, code block labels).
- [ ] Any past incident postmortems or upgrade retrospectives that should inform the guide.