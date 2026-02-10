# Datadog CI Integration

This document describes the Datadog CI/CD product integrations configured for the pup project.

## Overview

Pup uses multiple Datadog products to monitor and improve the CI/CD pipeline:

1. **CI Visibility** - Track test execution, performance, and flakiness
2. **Test Visibility** - Detailed test analytics and performance metrics
3. **Code Coverage** - Track coverage trends and identify untested code
4. **Static Analysis (SAST)** - Automated security and code quality scanning

## Products Integrated

### 1. Test Visibility

**What it does:** Instruments Go tests to send execution data to Datadog, enabling:
- Test performance tracking
- Flaky test detection
- Test failure analysis
- Historical test trends

**Implementation:**
- Uses `orchestrion` to automatically instrument Go tests
- Runs in agentless mode for GitHub Actions compatibility
- Sends test results directly to Datadog API

**Configuration:**
```yaml
DD_CIVISIBILITY_AGENTLESS_ENABLED: true
DD_CIVISIBILITY_GIT_UPLOAD_ENABLED: true
DD_CIVISIBILITY_ENABLED: true
```

**View in Datadog:** [Test Visibility Dashboard](https://app.datadoghq.com/ci/test-runs)

### 2. Code Coverage

**What it does:** Uploads coverage reports to Datadog for:
- Coverage trend analysis
- Per-commit coverage tracking
- Coverage regression detection
- Branch/PR coverage comparison

**Implementation:**
- Generates coverage with `go test -coverprofile`
- Uploads using `datadog-ci coverage upload`
- Supports Go coverage format

**View in Datadog:** [Code Coverage Dashboard](https://app.datadoghq.com/ci/coverage)

### 3. CI Pipeline Visibility

**What it does:** Tracks GitHub Actions workflow execution:
- Pipeline duration and success rates
- Job-level performance
- Bottleneck identification
- Historical pipeline trends

**Implementation:**
- Automatic tracking via Datadog GitHub Apps integration
- Requires GitHub Apps installation (see setup below)

**View in Datadog:** [CI Pipelines Dashboard](https://app.datadoghq.com/ci/pipelines)

### 4. Static Analysis (SAST)

**What it does:** Scans code for security vulnerabilities and quality issues:
- Security vulnerability detection
- Code quality issues
- Best practice violations
- Custom rule enforcement

**Implementation:**
- Runs on pull requests only
- Uses `datadog-ci sast scan`
- Results posted to PR and Datadog

**View in Datadog:** [Security Dashboard](https://app.datadoghq.com/security)

## Setup Requirements

### Repository Secrets

Add these secrets to your GitHub repository:

```bash
DD_API_KEY      # Datadog API key (required)
DD_APP_KEY      # Datadog Application key (required for SAST)
DD_SITE         # Datadog site (optional, defaults to datadoghq.com)
```

**To add secrets:**
1. Go to repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add each secret with its value

### Datadog Site Configuration

Set `DD_SITE` based on your Datadog region:
- US1: `datadoghq.com` (default)
- US3: `us3.datadoghq.com`
- US5: `us5.datadoghq.com`
- EU1: `datadoghq.eu`
- US1-FED: `ddog-gov.com`
- AP1: `ap1.datadoghq.com`

### GitHub Apps Integration (Optional)

For full CI Pipeline Visibility, install the Datadog GitHub Apps:

1. Go to [Datadog GitHub Integration](https://app.datadoghq.com/integrations/github)
2. Click "Install GitHub App"
3. Authorize for your repository
4. Configure pipeline tracking

## Features by Product

### Test Visibility Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| Test Performance | Track test execution time | Identify slow tests |
| Flaky Test Detection | Automatic detection of flaky tests | Improve reliability |
| Test Trends | Historical performance data | Track improvements |
| Failure Analysis | Detailed failure insights | Faster debugging |
| Intelligent Test Runner | Run only impacted tests | Faster CI runs |

### Code Coverage Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| Coverage Trends | Track coverage over time | Prevent regressions |
| File-level Coverage | Per-file coverage reports | Identify gaps |
| Branch Comparison | Compare PR vs base branch | Review coverage changes |
| Coverage Gates | Enforce minimum coverage | Maintain quality |

### SAST Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| Vulnerability Detection | Find security issues | Prevent exploits |
| Code Quality | Detect code smells | Improve maintainability |
| Custom Rules | Define project rules | Enforce standards |
| PR Comments | Inline findings on PRs | Faster remediation |

## Workflow Configuration

### Test Job with CI Visibility

```yaml
test:
  env:
    DD_API_KEY: ${{ secrets.DD_API_KEY }}
    DD_CIVISIBILITY_AGENTLESS_ENABLED: true
    DD_CIVISIBILITY_ENABLED: true
  steps:
    - name: Install orchestrion
      run: go install github.com/DataDog/orchestrion@latest

    - name: Run tests
      run: orchestrion go test -v -race ./...

    - name: Upload coverage
      run: datadog-ci coverage upload --format=go-cover coverage.out
```

### SAST Job

```yaml
sast:
  if: github.event_name == 'pull_request'
  env:
    DD_API_KEY: ${{ secrets.DD_API_KEY }}
    DD_APP_KEY: ${{ secrets.DD_APP_KEY }}
  steps:
    - name: Run SAST
      run: datadog-ci sast scan --service=pup --env=ci
```

## Local Development

### Running Tests with CI Visibility Locally

```bash
# Export Datadog credentials
export DD_API_KEY="your-api-key"
export DD_SITE="datadoghq.com"
export DD_CIVISIBILITY_AGENTLESS_ENABLED=true
export DD_CIVISIBILITY_ENABLED=true
export DD_SERVICE="pup"
export DD_ENV="local"

# Install orchestrion
go install github.com/DataDog/orchestrion@latest

# Run tests with instrumentation
orchestrion go test -v ./...

# Run with coverage
orchestrion go test -coverprofile=coverage.out ./pkg/...
```

### Uploading Coverage Locally

```bash
# Install datadog-ci
npm install -g @datadog/datadog-ci

# Upload coverage
export DATADOG_API_KEY="your-api-key"
datadog-ci coverage upload --format=go-cover coverage.out
```

### Running SAST Locally

```bash
# Export credentials
export DD_API_KEY="your-api-key"
export DD_APP_KEY="your-app-key"

# Run SAST scan
datadog-ci sast scan --service=pup --env=local
```

## Viewing Results

### Datadog UI Locations

| Product | Dashboard URL |
|---------|--------------|
| Test Visibility | https://app.datadoghq.com/ci/test-runs |
| Code Coverage | https://app.datadoghq.com/ci/coverage |
| CI Pipelines | https://app.datadoghq.com/ci/pipelines |
| SAST Results | https://app.datadoghq.com/security/appsec/findings |

### GitHub PR Integration

When configured, Datadog will:
- ✅ Post coverage changes as PR comments
- ✅ Add SAST findings as review comments
- ✅ Update PR status checks for quality gates
- ✅ Show test failures inline with code

## Troubleshooting

### Tests Not Appearing in Datadog

**Symptoms:** Tests run but don't show in Test Visibility dashboard

**Solutions:**
1. Verify `DD_API_KEY` is set correctly
2. Check `DD_CIVISIBILITY_ENABLED=true` is set
3. Ensure `orchestrion` is installed and in PATH
4. Check Datadog site is correct (`DD_SITE`)
5. Look for errors in test output

**Debug command:**
```bash
DD_TRACE_DEBUG=true orchestrion go test -v ./...
```

### Coverage Upload Fails

**Symptoms:** `datadog-ci coverage upload` command fails

**Solutions:**
1. Verify coverage file exists: `ls -la coverage.out`
2. Check API key: `echo $DATADOG_API_KEY`
3. Verify coverage format is correct
4. Check network connectivity to Datadog

**Manual verification:**
```bash
# Check coverage file format
head coverage.out

# Test API connectivity
curl -H "DD-API-KEY: $DATADOG_API_KEY" \
  https://api.datadoghq.com/api/v1/validate
```

### SAST Scan Fails

**Symptoms:** `datadog-ci sast scan` exits with error

**Solutions:**
1. Verify both `DD_API_KEY` and `DD_APP_KEY` are set
2. Check repository has code to scan
3. Ensure git history is available (fetch-depth: 0)
4. Review datadog-ci version compatibility

**Check configuration:**
```bash
datadog-ci --version
datadog-ci sast scan --dry-run
```

### orchestrion Hangs

**Symptoms:** Tests hang when using orchestrion

**Solutions:**
1. This is expected in CI with keychain access
2. Use conditional: `if [ -n "$DD_API_KEY" ]; then ... fi`
3. Ensure agentless mode is enabled
4. Check for blocking I/O operations

### Permission Denied Errors

**Symptoms:** GitHub Actions fails with permission errors

**Solutions:**
1. Add required permissions to workflow:
   ```yaml
   permissions:
     contents: write
     security-events: write
     pull-requests: write
   ```
2. Verify GitHub Apps has repository access
3. Check branch protection rules

## Cost Considerations

### Test Visibility Pricing

- Charged per test execution
- Check current pricing: https://www.datadoghq.com/pricing/

**Optimization tips:**
- Use Intelligent Test Runner to run fewer tests
- Filter test execution in local development
- Consider usage limits for free tier

### Code Coverage Pricing

- Included with Test Visibility
- No additional charge for coverage uploads

### SAST Pricing

- Charged per analyzed commit
- Runs on PRs only to minimize usage
- Consider running on specific branches

## Best Practices

### 1. **Minimize Test Runtime**
   - Use Intelligent Test Runner
   - Parallelize test execution
   - Cache dependencies

### 2. **Optimize Coverage Collection**
   - Only generate coverage for pkg/ directory
   - Upload once per PR (not per commit)
   - Use coverage caching

### 3. **SAST Efficiency**
   - Run only on pull requests
   - Skip on draft PRs if needed
   - Use incremental analysis

### 4. **Secret Management**
   - Rotate API keys regularly
   - Use separate keys per environment
   - Never commit keys to repository

### 5. **Dashboard Monitoring**
   - Set up alerts for test failures
   - Monitor coverage trends
   - Review SAST findings regularly

## Further Reading

- [Datadog CI Visibility Documentation](https://docs.datadoghq.com/continuous_integration/)
- [Go Test Visibility Setup](https://docs.datadoghq.com/tests/setup/go/)
- [Code Coverage Documentation](https://docs.datadoghq.com/code_coverage/)
- [Static Analysis Documentation](https://docs.datadoghq.com/code_security/static_analysis/)
- [datadog-ci CLI Reference](https://github.com/DataDog/datadog-ci)
- [orchestrion Repository](https://github.com/DataDog/orchestrion)

## Support

**Internal Datadog Support:**
- Slack: #ci-visibility, #code-analysis
- Documentation: https://docs.datadoghq.com/

**External Support:**
- GitHub Issues: https://github.com/DataDog/datadog-ci/issues
- Community: https://community.datadoghq.com/
