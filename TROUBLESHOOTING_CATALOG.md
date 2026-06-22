# Troubleshooting Catalog

A comprehensive guide to resolving common setup, build, test, and deployment errors for the Soroban Cookbook documentation site.

## 📋 Table of Contents

- [Setup Issues](#setup-issues)
- [Build Issues](#build-issues)
- [Development Issues](#development-issues)
- [Deployment Issues](#deployment-issues)
- [Runtime Issues](#runtime-issues)
- [Performance Issues](#performance-issues)

---

## 🔧 Setup Issues

### 1. Node.js Version Incompatible

**Error:** `Error: The engine "node" appears to be incompatible with this module`

**Root Cause:** Node.js version doesn't meet the minimum requirement (>=20.0)

**Fix Steps:**
```bash
# Check current version
node --version

# Install correct version using nvm
nvm install 20
nvm use 20

# Or download from nodejs.org
```

**Prevention:** Add `.nvmrc` file with `20` to enforce version consistency.

---

### 2. Bun Installation Failed

**Error:** `bun: command not found` or `Bun installation failed`

**Root Cause:** Bun package manager not installed or not in PATH

**Fix Steps:**
```bash
# Install Bun
curl -fsSL https://bun.sh/install | bash

# Restart terminal or reload PATH
source ~/.bashrc  # or ~/.zshrc

# Verify installation
bun --version
```

**Prevention:** Include Bun setup in project README and CI/CD workflows.

---

### 3. Dependencies Installation Failed

**Error:** `bun install failed` or `npm ERR! peer dep missing`

**Root Cause:** Corrupted lockfile, network issues, or incompatible dependencies

**Fix Steps:**
```bash
# Clear cache and reinstall
cd documentation
rm -rf node_modules bun.lock
bun install --frozen-lockfile

# If still failing, try fresh install
rm -rf node_modules bun.lock package-lock.json
bun install
```

**Prevention:** Always commit `bun.lock` file and use `--frozen-lockfile` in CI.

---

### 4. Git Submodules Missing

**Error:** `fatal: not a git repository` or submodule-related errors

**Root Cause:** Project uses submodules not properly initialized

**Fix Steps:**
```bash
git submodule update --init --recursive
git submodule sync
```

**Prevention:** Document submodule requirements in setup guide.

---

### 5. Environment Variables Missing

**Error:** `process.env.VARIABLE is undefined`

**Root Cause:** Required environment variables not set

**Fix Steps:**
```bash
# Create .env file
cp .env.example .env
# Edit .env with required values

# Or set in shell
export VARIABLE_NAME=value
```

**Prevention:** Provide `.env.example` file and document required variables.

---

## 🏗️ Build Issues

### 6. TypeScript Compilation Errors

**Error:** `TS2322: Type 'string' is not assignable to type 'number'`

**Root Cause:** Type mismatches, missing type definitions, or incorrect imports

**Fix Steps:**
```bash
# Run typecheck to see detailed errors
cd documentation
bun run typecheck

# Fix specific type issues
# Common fixes:
# - Add proper type annotations
# - Import correct types
# - Use type assertions carefully
```

**Prevention:** Enable strict TypeScript mode and run typecheck in CI.

---

### 7. ESLint Linting Failures

**Error:** `ESLint found too many warnings (max: 0)`

**Root Cause:** Code style violations, unused variables, or syntax issues

**Fix Steps:**
```bash
# Auto-fix where possible
cd documentation
bun run lint:fix

# Manual fixes for remaining issues
bun run lint
```

**Prevention:** Configure editor to run ESLint on save and set up pre-commit hooks.

---

### 8. Prettier Formatting Issues

**Error:** `Prettier check failed` or `Code style issues found`

**Root Cause:** Code not following project's formatting standards

**Fix Steps:**
```bash
# Auto-format all files
cd documentation
bun run format

# Check specific files
bun run format:check
```

**Prevention:** Set up editor integrations and pre-commit hooks for Prettier.

---

### 9. Docusaurus Build Failures

**Error:** `Build failed` or `Cannot read property 'map' of undefined`

**Root Cause:** Invalid frontmatter, broken links, or plugin misconfiguration

**Fix Steps:**
```bash
# Clear build cache
cd documentation
bun run clear

# Rebuild
bun run build

# Check for specific errors in build output
# Common issues:
# - Missing required frontmatter fields
# - Invalid sidebar configuration
# - Broken internal links
```

**Prevention:** Validate frontmatter and links in CI, use Docusaurus's built-in link checking.

---

### 10. Asset Import Errors

**Error:** `Module not found: Can't resolve './image.png'`

**Root Cause:** Incorrect asset paths or missing files in `static/` directory

**Fix Steps:**
```bash
# Verify file exists in correct location
ls documentation/static/

# Fix import paths
# Use absolute paths from static/ directory
# Example: import image from '/image.png'
```

**Prevention:** Use Docusaurus's static folder structure and document asset organization.

---

## 💻 Development Issues

### 11. Port Already in Use

**Error:** `Port 3000 is already in use`

**Root Cause:** Previous dev server still running or another process using port

**Fix Steps:**
```bash
# Kill process on port 3000
lsof -ti:3000 | xargs kill -9  # macOS/Linux
# or
netstat -ano | findstr :3000  # Windows

# Or use different port
cd documentation
bun start -- --port 3001
```

**Prevention:** Use `.env` file to configure default port and document port usage.

---

### 12. Hot Reload Not Working

**Error:** Changes not reflecting in browser during development

**Root Cause:** File watcher issues, incorrect file extensions, or configuration problems

**Fix Steps:**
```bash
# Restart dev server
cd documentation
bun run clear
bun start

# Check file permissions
# Ensure files are writable by the dev process
```

**Prevention:** Use proper file extensions (.mdx for React components) and avoid large node_modules in watch paths.

---

### 13. MDX Compilation Errors

**Error:** `MDX compilation failed` or `Unexpected JSX token`

**Root Cause:** Invalid MDX syntax, missing imports, or component issues

**Fix Steps:**
```bash
# Check MDX syntax
# Ensure proper JSX closing tags
# Import required components at top of file
# Use proper frontmatter

# Example fix:
import React from 'react';
import Component from './Component';
```

**Prevention:** Use MDX linting tools and validate syntax in CI.

---

### 14. CSS/Style Loading Issues

**Error:** Styles not applying or CSS modules not working

**Root Cause:** Incorrect CSS imports, missing Tailwind configuration, or path issues

**Fix Steps:**
```bash
# Check CSS imports in custom CSS
# Verify Tailwind configuration
# Ensure proper CSS module usage

# Restart dev server after CSS changes
```

**Prevention:** Document CSS architecture and use consistent styling patterns.

---

### 15. Search Index Not Building

**Error:** Search functionality not working or index outdated

**Root Cause:** Search plugin misconfiguration or build issues

**Fix Steps:**
```bash
# Rebuild search index
cd documentation
bun run build

# Check search plugin configuration
# Verify docusaurus.config.ts search settings
```

**Prevention:** Configure search plugins properly and test search functionality regularly.

---

## 🚢 Deployment Issues

### 16. GitHub Pages Deployment Failed

**Error:** `Pages is not enabled for this repository` or missing deployment workflow

**Root Cause:** GitHub Pages not configured, deployment workflow missing, or permissions insufficient

**Fix Steps:**
```bash
# In GitHub repository settings:
# 1. Go to Settings → Pages
# 2. Set Source to "GitHub Actions"
# 3. Go to Settings → Actions → General
# 4. Set "Workflow permissions" to "Read and write permissions"

# Create deploy.yml in .github/workflows/ if missing
# Reference DEPLOYMENT.md for complete setup
```

**Prevention:** Document deployment setup requirements and validate in CI.

---

### 17. Workflow Permissions Error

**Error:** `Permission denied` or `Insufficient permissions`

**Root Cause:** GitHub Actions workflow lacks necessary permissions

**Fix Steps:**
```yaml
# In workflow file, add proper permissions:
permissions:
  contents: read
  pages: write
  id-token: write
  checks: write
  pull-requests: write
```

**Prevention:** Use minimal required permissions and document permission requirements.

---

### 18. Build Artifact Missing

**Error:** `No build artifact found` or `index.html not found`

**Root Cause:** Build process failed or artifact upload issues

**Fix Steps:**
```bash
# Verify build locally
cd documentation
bun run build
ls build/  # Should contain index.html

# Check CI build logs for specific errors
# Fix build issues and redeploy
```

**Prevention:** Add build verification steps in CI and validate artifact creation.

---

### 19. Cache Issues in CI/CD

**Error:** Stale cache causing build failures or outdated dependencies

**Root Cause:** Cache key conflicts or corrupted cache

**Fix Steps:**
```yaml
# In GitHub Actions, clear cache:
- name: Clear cache
  run: |
    echo "Clearing cache..."
    rm -rf ~/.bun/install/cache
```

**Prevention:** Use proper cache keys and implement cache invalidation strategy.

---

### 20. Environment-Specific Build Failures

**Error:** Build works locally but fails in CI/CD

**Root Cause:** Environment differences, missing dependencies, or platform-specific issues

**Fix Steps:**
```bash
# Match CI environment locally
# Use same Node.js version
# Use same dependency versions
# Check for platform-specific code

# Debug with:
docker run -it ubuntu:latest  # Match CI environment
```

**Prevention:** Use container-based development and document environment requirements.

---

## 🌐 Runtime Issues

### 21. 404 Errors on Production

**Error:** Pages returning 404 after deployment

**Root Cause:** Incorrect base URL configuration or routing issues

**Fix Steps:**
```typescript
// In docusaurus.config.ts, set correct baseUrl:
const config = {
  baseUrl: '/',  // Current setting for custom domain
  // For GitHub Pages: '/repository-name/'
};
```

**Prevention:** Test deployment thoroughly and configure base URL correctly for each environment.

---

### 22. JavaScript Console Errors

**Error:** Various JS errors in browser console

**Root Cause:** Client-side code issues, missing dependencies, or runtime errors

**Fix Steps:**
```bash
# Check browser console for specific errors
# Fix component issues
# Verify all imports are correct
# Test in different browsers
```

**Prevention:** Use TypeScript for type safety and implement error boundaries.

---

### 23. Slow Page Load Times

**Error:** Pages taking too long to load

**Root Cause:** Large assets, unoptimized images, or inefficient code

**Fix Steps:**
```bash
# Analyze bundle size
cd documentation
bun run build
# Check build/ directory size

# Optimize images
# Use lazy loading
# Implement code splitting
```

**Prevention:** Monitor bundle size, optimize assets, and use performance budgets.

---

### 24. Search Not Working on Production

**Error:** Search functionality broken after deployment

**Root Cause:** Search index not built or incorrect configuration

**Fix Steps:**
```bash
# Rebuild and redeploy
cd documentation
bun run build

# Check search plugin configuration
# Verify search files are deployed
```

**Prevention:** Test search functionality in staging before production deployment.

---

### 25. Responsive Design Issues

**Error:** Layout broken on mobile devices

**Root Cause:** CSS not responsive or viewport configuration issues

**Fix Steps:**
```css
/* Ensure proper viewport meta tag */
/* Use responsive design patterns */
/* Test on different screen sizes */
```

**Prevention:** Use responsive design from start and test on multiple devices.

---

## ⚡ Performance Issues

### 26. Large Bundle Size

**Error:** JavaScript bundle too large (>1MB)

**Root Cause:** Unused dependencies, large libraries, or unoptimized code

**Fix Steps:**
```bash
# Analyze bundle
cd documentation
bun run build

# Use bundle analyzer
npm install --save-dev webpack-bundle-analyzer
# Configure in docusaurus.config.ts
```

**Prevention:** Regular bundle analysis, dependency audits, and code splitting.

---

### 27. Memory Leaks in Development

**Error:** Node.js process memory increasing continuously

**Root Cause:** Memory leaks in custom code or plugins

**Fix Steps:**
```bash
# Monitor memory usage
# Profile with Chrome DevTools
# Fix event listener cleanup
# Check for circular references
```

**Prevention:** Proper cleanup in React components and regular performance monitoring.

---

### 28. Slow Build Times

**Error:** Build process taking too long (>5 minutes)

**Root Cause:** Inefficient build configuration or large asset processing

**Fix Steps:**
```bash
# Optimize build configuration
# Use build caching
# Parallelize build steps
# Optimize images and assets
```

**Prevention:** Implement build caching and optimize build configuration.

---

### 29. CI/CD Pipeline Timeouts

**Error:** GitHub Actions timing out after 30 minutes

**Root Cause:** Inefficient workflows or resource-intensive operations

**Fix Steps:**
```yaml
# Optimize workflow steps
# Use caching effectively
# Parallelize independent jobs
# Break into smaller workflows
```

**Prevention:** Regular workflow optimization and monitoring of execution times.

---

### 30. SEO and Accessibility Issues

**Error:** Poor SEO scores or accessibility violations

**Root Cause:** Missing meta tags, poor structure, or accessibility issues

**Fix Steps:**
```bash
# Run lighthouse audit
# Fix meta tags and descriptions
# Improve semantic HTML
# Add alt text to images
```

**Prevention:** Regular accessibility audits and SEO optimization.

---

## 🔍 Quick Reference Commands

### Local Development
```bash
cd documentation
bun install --frozen-lockfile
bun start                    # Start dev server
bun run build               # Build for production
bun run serve               # Serve production build
bun run typecheck           # Check TypeScript
bun run lint                # Lint code
bun run format              # Format code
```

### Troubleshooting Commands
```bash
# Clear all caches
bun run clear
rm -rf node_modules bun.lock
bun install --frozen-lockfile

# Full CI simulation
bun run format:check && bun run lint && bun run typecheck && bun run build

# Check specific issues
bun run typecheck    # TypeScript errors
bun run lint         # Linting issues
bun run build        # Build problems
```

### Deployment Verification
```bash
# Verify build artifact
ls documentation/build/
file documentation/build/index.html

# Test production build locally
bun run build && bun run serve
```

---

## 📞 Getting Help

If you encounter issues not covered in this catalog:

1. **Check GitHub Issues**: Search existing issues in the repository
2. **Review Logs**: Check CI/CD workflow logs for detailed error messages
3. **Community Support**: Join the [Stellar Discord](https://discord.gg/stellardev)
4. **Documentation**: Refer to [Docusaurus Docs](https://docusaurus.io/docs)
5. **Create Issue**: Open a new issue with detailed error logs and reproduction steps

---

## 🔄 Contributing to This Catalog

To add new issues or improve existing solutions:

1. Reproduce the issue locally
2. Document clear, reproducible steps
3. Verify solutions work across different environments
4. Update this catalog with prevention tips
5. Test solutions in CI/CD pipeline

---

**Last Updated**: 2026-04-26  
**Version**: 1.0.0  
**Maintainers**: Soroban Cookbook Team
