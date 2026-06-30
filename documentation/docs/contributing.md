---
sidebar_position: 10
title: Contributing Guide
description: Learn how to contribute to the Soroban Cookbook project.
---

# Contributing to Soroban Cookbook

First off, thank you for considering contributing to the Soroban Cookbook! It's people like you that make this a great resource for the Stellar community.

This guide will help you get started with the contribution process, from setting up your development environment to submitting your first pull request.

---

## 🚀 Getting Started

### Project Overview

Soroban Cookbook is a comprehensive documentation platform for Stellar smart contract development. We aim to provides interactive guides, patterns, and tutorials that are easy to follow and production-ready.

### Types of Contributions

We welcome several types of contributions:

- **Documentation:** Improving explanations, fixing typos, or adding new guides.
- **Smart Contract Examples:** Adding new reusable patterns or contract examples.
- **UI/UX Improvements:** Enhancing the website's look, feel, and accessibility.
- **Bug Fixes:** Identifying and resolving issues in code or documentation.

---

## 🛠️ Setup Instructions

To contribute code or documentation changes, you'll need to set up the project locally.

### Prerequisites

- **Rust & Soroban CLI:** [Install Rust](https://www.rust-lang.org/tools/install) and the [Soroban CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup#install-the-soroban-cli).
- **Bun:** We use [Bun](https://bun.sh/) as our primary JavaScript runtime and package manager.
- **Git:** For version control.

### Local Development Setup

1. **Fork and Clone**

   ```bash
   git clone https://github.com/Soroban-Cookbook/Soroban_Cookbook_online.git
   cd Soroban_Cookbook_online
   ```

2. **Install Dependencies**

   ```bash
   cd documentation
   bun install  # (alternative: npm install)
   ```

3. **Run Development Server**
   ```bash
   bun start    # (alternative: npm start)
   ```
   The site will be available at `http://localhost:3000`.

---

## 🌿 Branching & PR Conventions

### Branch Naming

Keep your branches scoped to a single concern. Use the following naming convention:

- `feat/...` for new features or patterns.
- `fix/...` for bug fixes.
- `docs/...` for documentation-only changes.
- `chore/...` for maintenance tasks.

### Commit Messages

We follow a lightweight conventional commit style:

- `feat: add liquidity pool pattern`
- `fix: correct typo in storage docs`
- `docs: update contributing guide`

### Pull Requests

A high-quality PR includes:

- **Clear Title:** Concise summary of the change.
- **Detailed Description:** What changed and why.
- **Issue Link:** Reference any related issues (e.g., `Closes #123`).
- **Media:** Screenshots or recordings for any UI changes.

---

## 📑 Contribution Workflows

### A. Documentation Contributions

Docs are written in **MDX** and located in `documentation/docs/`.

- **Cross-linking:** Always use relative paths for internal links (e.g., `[Setup](./setup.md)`).
- **Formatting:** Follow the existing structure and use standard Markdown.
- **Metadata:** Ensure every page has proper frontmatter (title, description).

### B. Example / Code Contributions

Contract examples should be minimal, focused, and well-documented.

- **Storage:** Use `examples/` for standalone Rust projects (if applicable).
- **Inline Docs:** Explain complex logic within the Rust code snippets using comments.
- **Best Practices:** Follow [Soroban Best Practices](https://developers.stellar.org/docs/smart-contracts/best-practices/security-checklist).

### C. Fixes & Improvements

- **Scope:** Keep PRs small. Avoid unrelated refactors in the same PR.
- **Validation:** Ensure your fix doesn't break existing functionality.

---

## 🧪 Local Validation Steps

Before submitting a PR, you **must** run the following checks in the `documentation/` directory:

```bash
# Using bun (recommended)
bun run typecheck
bun run lint
bun run format:check
bun run build

# Using npm (fallback)
npm run typecheck
npm run lint
npm run format:check
npm run build
```

For Rust code examples, ensure they compile and pass tests:

```bash
cargo check
cargo test
cargo fmt --all -- --check
```

---

## ✅ Pre-PR Checklist

Before you hit "Submit", make sure you've checked these off:

- [ ] My code compiles successfully without warnings.
- [ ] Documentation renders correctly in the local dev server.
- [ ] I have verified all links are functional.
- [ ] My changes are scoped and minimal.
- [ ] I have followed the project's styling and coding conventions.
- [ ] No linting or formatting errors remain.

---

## 🔍 Review Expectations

### What Reviewers Look For

- **Correctness:** Does the code/doc work as intended?
- **Clarity:** Is the explanation easy to understand for a beginner?
- **Consistency:** Does it follow the existing patterns and design tokens?
- **Utility:** Does this add value to the cookbook?

### Iteration Process

Expect some feedback! We might ask for clarifications or small adjustments. This is part of maintaining high standards for the community.

---

## 📞 Getting Help

If you're stuck, feel free to:

- Open a [GitHub Discussion](https://github.com/Soroban-Cookbook/Soroban_Cookbook_online/discussions).
- Join the [Stellar Dev Discord](https://discord.gg/stellardev) and ask in the `#soroban` channel.

---

**Thank you for helping us build the best Soroban resource! 🚀**
