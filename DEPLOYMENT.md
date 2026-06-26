# Deployment Guide

This document provides comprehensive information about deploying the Soroban Cookbook documentation.

## GitHub Pages Deployment

### Overview

The documentation is automatically deployed to GitHub Pages on every push to the `main` branch via GitHub Actions. The workflow is defined in `.github/workflows/deploy.yml`.

### Prerequisites

Before the workflow can successfully deploy, ensure the following repository settings are configured:

#### 1. Enable GitHub Pages

1. Navigate to your repository settings
2. Go to **Settings → Pages**
3. Under "Build and deployment":
   - Set **Source** to "GitHub Actions"
   - This allows the workflow to deploy built artifacts

#### 2. Configure Workflow Permissions

1. Go to **Settings → Actions → General**
2. Under "Workflow permissions":
   - Select "Read and write permissions"
   - Enable "Allow GitHub Actions to create and approve pull requests"
3. This grants the workflow necessary permissions to deploy

#### 3. Set Up Branch Protection (Recommended)

1. Go to **Settings → Branches**
2. Add a rule for the `main` branch:
   - Require status checks to pass before merging
   - Select the "build" job from the deploy workflow
   - This ensures only successful builds are merged

### Workflow Details

**Trigger Events:**

- Push to `main` branch
- Manual trigger via "Run workflow" button in Actions tab

**Build Process:**

1. Checkout code
2. Setup Bun
3. Install dependencies using `bun install --frozen-lockfile`
4. Build documentation with `bun run build`
5. Upload build artifact to GitHub Pages

**Deployment Process:**

1. Configure GitHub Pages environment
2. Deploy artifact to GitHub Pages
3. Deployment URL available in workflow run details

### Monitoring Deployments

1. Go to **Actions** tab in your repository
2. Select the "Deploy to GitHub Pages" workflow
3. View recent runs and their status
4. Click on a run to see detailed logs

### Troubleshooting

#### Workflow Fails with "Pages is not enabled"

**Solution:** Ensure GitHub Pages source is set to "GitHub Actions" in repository settings.

#### Deployment Skipped

**Solution:** Check that:

- You're pushing to the `main` branch
- Workflow permissions are set to "Read and write"
- No branch protection rules are blocking the deployment

#### Build Fails

**Solution:** Check the workflow logs for specific errors:

1. Go to Actions tab
2. Click on the failed run
3. Expand the "Build website" step to see error details

Common issues:

- Missing dependencies: Run `bun install --frozen-lockfile` locally to verify
- TypeScript errors: Run `bun run typecheck` locally
- Build errors: Run `bun run build` locally to reproduce

#### Artifact Upload Fails

**Solution:** Verify that:

- The build directory exists at `documentation/build`
- Build completed successfully (check previous step logs)
- Sufficient storage quota available

### Manual Deployment

To manually trigger a deployment:

1. Go to **Actions** tab
2. Select "Deploy to GitHub Pages" workflow
3. Click "Run workflow"
4. Select the branch (usually `main`)
5. Click "Run workflow"

### Rollback

GitHub Pages automatically serves the latest deployment. To rollback:

1. Revert the problematic commit on `main`
2. Push the revert commit
3. The workflow will automatically deploy the previous version

## Local Development

### Building Locally

```bash
cd documentation
bun install
bun run build
```

The built site will be in `documentation/build/`.

### Serving Locally

```bash
cd documentation
bun run serve
```

Visit `http://localhost:3000` to view the built site.

### Development Server

```bash
cd documentation
bun start
```

This starts a live-reload development server at `http://localhost:3000`.

## Environment Variables

Currently, no environment variables are required for deployment. The workflow uses:

- Bun (specified in workflow)
- Docusaurus build configuration from `documentation/docusaurus.config.ts`

## Performance Considerations

- Build time: ~2-3 minutes (depends on content size)
- Artifact size: ~5-10 MB (typical for Docusaurus sites)
- Deployment time: ~1-2 minutes

## Security

- Workflow uses `actions/checkout@v4` (latest stable)
- Permissions are minimal: `contents: read`, `pages: write`, `id-token: write`
- No secrets required for GitHub Pages deployment
- All code is built from the repository source

## Alert System

Alerting is handled by `.github/workflows/alerts.yml`. The workflow covers three scenarios:

| Trigger | Job | What fires |
|---|---|---|
| CI or CD workflow fails on `main` | `notify-failure` | Slack message with workflow name, branch, commit, actor, and run URL |
| CI or CD workflow recovers on `main` | `notify-recovery` | Slack recovery message |
| Schedule (every 30 min) | `uptime-check` | HTTP probe against the live site; Slack alert if non-2xx/3xx |
| Manual `workflow_dispatch` with `test_alert=true` | `test-alert` | Sends a test Slack message to verify the integration |

### Setup

#### 1. Create a Slack Incoming Webhook

1. Go to [api.slack.com/apps](https://api.slack.com/apps) → **Create New App** → **From scratch**.
2. Name it `Soroban Cookbook Alerts`, pick your workspace.
3. Under **Features** → **Incoming Webhooks**, toggle **Activate Incoming Webhooks** on.
4. Click **Add New Webhook to Workspace**, select the target channel (e.g. `#soroban-alerts`), and click **Allow**.
5. Copy the webhook URL (format: `https://hooks.slack.com/services/…`).

#### 2. Add the secret to GitHub

1. Go to **Settings → Secrets and variables → Actions**.
2. Click **New repository secret**.
3. Name: `SLACK_WEBHOOK_URL` — Value: the URL copied above.

#### 3. (Optional) Override the monitored URL

The uptime probe defaults to `https://soroban-cookbook.dev`. To change it without editing the workflow:

1. Go to **Settings → Secrets and variables → Actions → Variables** tab.
2. Add a variable named `SITE_URL` with the target URL as the value.

#### 4. Verify the integration

1. Go to **Actions** → **Alert System** → **Run workflow**.
2. Set **test_alert** to `true` and click **Run workflow**.
3. A test message should appear in your Slack channel within seconds.

### On-Call Rotation

This project is community-maintained. There is no formal PagerDuty rotation. The Slack channel configured above serves as the incident notification channel. Triage follows this process:

1. **Slack alert fires** → anyone with repository access investigates the linked Actions run.
2. **Build failure** → check the failed job logs; common causes are dependency updates or broken Rust examples.
3. **Site downtime** → check GitHub Pages status at [githubstatus.com](https://www.githubstatus.com) first. If Pages is healthy, check the last deployment run.
4. **Escalation** → open a GitHub issue tagged `incident` and post in [Stellar Discord](https://discord.gg/stellardev) `#soroban-dev`.

If you want to set up a formal PagerDuty integration, replace the `slackapi/slack-github-action` steps with calls to the [PagerDuty Events API v2](https://developer.pagerduty.com/api-reference/YXBpOjI3NDgyNjU-pager-duty-v2-events-api) using a `PAGERDUTY_INTEGRATION_KEY` secret.

### Alert Channels Reference

| Channel | Purpose | Configured via |
|---|---|---|
| Slack `#soroban-alerts` | CI failures, recoveries, downtime | `SLACK_WEBHOOK_URL` secret |
| GitHub Actions email | Default GitHub notification for workflow failures | GitHub account notification settings |

---

## Future Improvements

- [ ] Add build caching to speed up deployments
- [ ] Add performance metrics collection
- [ ] Implement preview deployments for pull requests
- [ ] Add automated lighthouse audits
- [ ] Set up deployment notifications

## Support

For issues or questions:

1. Check the troubleshooting section above
2. Review workflow logs in the Actions tab
3. Open an issue on GitHub with workflow logs attached
4. Join the [Stellar Discord](https://discord.gg/stellardev) for community support
