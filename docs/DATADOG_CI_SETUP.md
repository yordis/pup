# Datadog CI Products - Quick Setup Guide

This guide walks through setting up Datadog CI products for the pup repository.

## Prerequisites

- GitHub repository admin access
- Datadog account with API access
- Datadog API key and Application key

## Step 1: Get Datadog Credentials

### 1.1 API Key

1. Log in to [Datadog](https://app.datadoghq.com/)
2. Navigate to **Organization Settings** → **API Keys**
3. Create a new API key or copy existing one
4. Save the key securely (format: `abc123def456...`)

### 1.2 Application Key

1. In Datadog, go to **Organization Settings** → **Application Keys**
2. Create a new application key
3. Name it: `pup-ci-integration`
4. Save the key securely (format: `xyz789abc456...`)

### 1.3 Datadog Site

Determine your Datadog site based on your URL:

| URL | DD_SITE Value |
|-----|---------------|
| https://app.datadoghq.com | `datadoghq.com` |
| https://us3.datadoghq.com | `us3.datadoghq.com` |
| https://us5.datadoghq.com | `us5.datadoghq.com` |
| https://app.datadoghq.eu | `datadoghq.eu` |
| https://app.ddog-gov.com | `ddog-gov.com` |
| https://ap1.datadoghq.com | `ap1.datadoghq.com` |

## Step 2: Add GitHub Secrets

### 2.1 Navigate to Repository Settings

1. Go to your GitHub repository
2. Click **Settings** (top right)
3. In the left sidebar, expand **Secrets and variables**
4. Click **Actions**

### 2.2 Add Required Secrets

Click **New repository secret** and add each of these:

#### Secret 1: DD_API_KEY
- **Name:** `DD_API_KEY`
- **Value:** Your Datadog API key from Step 1.1
- Click **Add secret**

#### Secret 2: DD_APP_KEY
- **Name:** `DD_APP_KEY`
- **Value:** Your Datadog Application key from Step 1.2
- Click **Add secret**

#### Secret 3: DD_SITE (Optional)
- **Name:** `DD_SITE`
- **Value:** Your Datadog site from Step 1.3 (defaults to `datadoghq.com`)
- Click **Add secret**

### 2.3 Verify Secrets

Your secrets should look like this:

```
DD_API_KEY      ****************  Updated X minutes ago
DD_APP_KEY      ****************  Updated X minutes ago
DD_SITE         datadoghq.com     Updated X minutes ago
```

## Step 3: Enable GitHub Actions

### 3.1 Enable Workflows

1. Go to **Actions** tab in your repository
2. Click **I understand my workflows, go ahead and enable them**
3. Verify workflows are enabled

### 3.2 Trigger a Test Run

Create a test pull request to verify everything works:

```bash
# Create a test branch
git checkout -b test/datadog-ci-integration

# Make a trivial change
echo "# Test" >> test.md

# Commit and push
git add test.md
git commit -m "test: verify Datadog CI integration"
git push origin test/datadog-ci-integration

# Create PR
gh pr create --title "Test: Verify Datadog CI Integration" \
  --body "Testing Datadog CI products integration"
```

## Step 4: Verify Integration

### 4.1 Check GitHub Actions

1. Go to **Actions** tab
2. Find your PR workflow run
3. Check all jobs pass:
   - ✅ Test and Coverage
   - ✅ Lint
   - ✅ Build
   - ✅ Datadog Static Analysis

### 4.2 Check Test Visibility

1. Go to [Datadog Test Visibility](https://app.datadoghq.com/ci/test-runs)
2. Filter by service: `pup`
3. Verify you see test runs from your PR

Expected view:
```
Service: pup
Tests: 163 tests
Duration: ~30s
Status: Passed
```

### 4.3 Check Code Coverage

1. Go to [Datadog Code Coverage](https://app.datadoghq.com/ci/coverage)
2. Filter by service: `pup`
3. Verify coverage reports appear

Expected view:
```
Service: pup
Coverage: ~75-80%
Files: ~40 files
```

### 4.4 Check CI Pipelines

1. Go to [Datadog CI Pipelines](https://app.datadoghq.com/ci/pipelines)
2. Find `DataDog/pup` repository
3. Verify pipeline runs appear

Expected view:
```
Pipeline: DataDog/pup
Branch: test/datadog-ci-integration
Status: Passed
Duration: ~2-3 minutes
```

### 4.5 Check SAST Results

1. Go to [Datadog Security](https://app.datadoghq.com/security/appsec/findings)
2. Filter by service: `pup`
3. Review any findings

Expected: Few or no findings (clean codebase)

## Step 5: Configure Notifications (Optional)

### 5.1 Create Monitor for Test Failures

```yaml
# Datadog Monitor Configuration
name: Pup CI Test Failures
type: ci-test
query: |
  ci-tests(service:pup status:fail).rollup(count).last(5m) > 0
message: |
  CI tests are failing in pup repository.
  Check: https://app.datadoghq.com/ci/test-runs?service=pup
```

### 5.2 Slack Integration

1. Go to **Integrations** → **Slack**
2. Configure Slack channel: `#pup-ci-alerts`
3. Add monitors to post to channel

### 5.3 GitHub Status Checks (Optional)

For PR quality gates:

1. Go to repository **Settings** → **Branches**
2. Add branch protection rule for `main`
3. Enable: "Require status checks to pass"
4. Select Datadog checks:
   - `datadog-ci/sast`
   - `test-visibility/coverage`

## Troubleshooting

### Tests Not Appearing in Datadog

**Problem:** Tests run but don't show in Datadog

**Solution:**
1. Check GitHub Actions logs for errors
2. Verify `DD_API_KEY` is set correctly
3. Check Datadog site matches your account
4. Look for orchestrion errors in test output

**Debug:**
```bash
# Check if secrets are accessible (they won't show values)
gh secret list

# View workflow logs
gh run view <run-id> --log
```

### SAST Job Failing

**Problem:** SAST job fails with authentication error

**Solution:**
1. Verify both `DD_API_KEY` and `DD_APP_KEY` are set
2. Check Application Key has correct permissions
3. Verify datadog-ci CLI installed correctly

**Debug:**
```bash
# Test API connectivity locally
curl -H "DD-API-KEY: $DD_API_KEY" \
  https://api.datadoghq.com/api/v1/validate

# Test with datadog-ci locally
export DD_API_KEY="your-key"
export DD_APP_KEY="your-app-key"
datadog-ci sast scan --dry-run
```

### Coverage Not Uploading

**Problem:** Coverage report generated but not in Datadog

**Solution:**
1. Check `datadog-ci` installed successfully
2. Verify coverage.out file exists
3. Check DATADOG_API_KEY environment variable

**Debug:**
```bash
# Check coverage file
ls -la coverage.out

# Manual upload test
datadog-ci coverage upload --format=go-cover coverage.out
```

## Maintenance

### Rotating API Keys

When rotating keys:

1. Create new key in Datadog
2. Update GitHub secret
3. Trigger test workflow
4. Delete old key after verification

### Monitoring Costs

Datadog CI products are metered:

- **Test Visibility:** Per test execution
- **Code Coverage:** Included with Test Visibility
- **SAST:** Per analyzed commit
- **CI Pipeline:** Per pipeline run

**Monitor usage:**
1. Go to **Plan & Usage** in Datadog
2. Check **CI Visibility** section
3. Set up usage alerts

**Optimization:**
- Run SAST only on PRs (already configured)
- Use Intelligent Test Runner to skip unchanged tests
- Consider test parallelization for faster runs

## Next Steps

Once setup is complete:

1. ✅ Close and delete test PR
2. ✅ Review [DATADOG_CI.md](DATADOG_CI.md) for detailed features
3. ✅ Set up Datadog monitors and alerts
4. ✅ Configure team notifications
5. ✅ Train team on using Datadog CI dashboards

## Support

**Issues with setup:**
- Open GitHub issue: https://github.com/DataDog/pup/issues
- Internal Slack: #ci-visibility, #code-analysis

**Datadog support:**
- Documentation: https://docs.datadoghq.com/continuous_integration/
- Community: https://community.datadoghq.com/
- Support: https://app.datadoghq.com/help

## Checklist

Use this checklist to track setup progress:

- [ ] Obtained Datadog API key
- [ ] Obtained Datadog Application key
- [ ] Determined Datadog site
- [ ] Added DD_API_KEY secret to GitHub
- [ ] Added DD_APP_KEY secret to GitHub
- [ ] Added DD_SITE secret to GitHub (optional)
- [ ] Enabled GitHub Actions
- [ ] Created test PR
- [ ] Verified tests appear in Test Visibility
- [ ] Verified coverage appears in Code Coverage
- [ ] Verified pipelines appear in CI Pipelines
- [ ] Verified SAST runs on PR
- [ ] Configured monitors (optional)
- [ ] Set up Slack notifications (optional)
- [ ] Configured branch protection rules (optional)
- [ ] Documented setup for team
