# Homebrew Tap Setup Guide

This guide documents the setup required to enable automatic Homebrew formula publishing to the `DataDog/homebrew-pack` tap.

## Overview

When a new release is tagged, GoReleaser will automatically:
1. Build multi-platform binaries
2. Create a GitHub release
3. Generate and push a Homebrew formula to `DataDog/homebrew-pack`
4. Users can then install via: `brew install datadog/pack/pup`

## Prerequisites

### 1. Repository Setup

The `DataDog/homebrew-pack` repository must:
- ✅ Exist at https://github.com/DataDog/homebrew-pack
- ✅ Be public (or have appropriate access configured)
- ✅ Have a `Formula/` directory (GoReleaser will create it if missing)
- ✅ Follow Homebrew tap naming conventions (`homebrew-*` prefix)

### 2. GitHub Personal Access Token (PAT)

You need to create a **Fine-grained Personal Access Token** with the following permissions:

#### Token Permissions Required:
- **Repository access**: `DataDog/homebrew-pack`
- **Repository permissions**:
  - Contents: `Read and Write` (to push formula updates)
  - Metadata: `Read-only` (automatically granted)

#### Creating the Token:

1. Go to: https://github.com/settings/tokens?type=beta
2. Click **Generate new token** (Fine-grained)
3. Configure:
   - **Token name**: `pup-homebrew-tap-publisher`
   - **Expiration**: `No expiration` or `Custom` (recommend 1 year)
   - **Repository access**: Select **Only select repositories** → Choose `DataDog/homebrew-pack`
   - **Permissions**: Set `Contents` to `Read and Write`
4. Click **Generate token**
5. **Copy the token immediately** (you won't be able to see it again)

### 3. Add Token as GitHub Secret

Add the PAT to the `DataDog/pup` repository secrets:

1. Go to: https://github.com/DataDog/pup/settings/secrets/actions
2. Click **New repository secret**
3. Configure:
   - **Name**: `HOMEBREW_TAP_TOKEN`
   - **Secret**: Paste the PAT created above
4. Click **Add secret**

## Verification Checklist

Before creating your first release with Homebrew tap publishing:

- [ ] `DataDog/homebrew-pack` repository exists and is public
- [ ] Fine-grained PAT created with `Contents: Read and Write` on `homebrew-pack`
- [ ] `HOMEBREW_TAP_TOKEN` secret added to `DataDog/pup` repository
- [ ] Release workflow has `HOMEBREW_TAP_TOKEN` in env (already done in `.github/workflows/release.yml`)
- [ ] GoReleaser config includes `brews` section (already done in `.goreleaser.yml`)

## Testing the Setup

### Test with a Pre-release

To test without affecting production:

1. Create a pre-release tag:
   ```bash
   git tag -a v0.9.0-beta.1 -m "Test release for Homebrew tap"
   git push origin v0.9.0-beta.1
   ```

2. Monitor the GitHub Actions workflow:
   - Go to: https://github.com/DataDog/pup/actions
   - Check the "Release" workflow run
   - Verify GoReleaser successfully pushes to `homebrew-pack`

3. Verify the formula was created:
   - Check: https://github.com/DataDog/homebrew-pack/blob/main/Formula/pup.rb
   - The formula should be auto-generated with version `0.9.0-beta.1`

4. Test installation (optional):
   ```bash
   brew tap datadog/pack
   brew install pup
   pup version
   ```

5. Clean up the test release if needed:
   ```bash
   git tag -d v0.9.0-beta.1
   git push origin :refs/tags/v0.9.0-beta.1
   ```

## First Production Release

Once testing is successful:

1. Create a production release tag:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

2. The workflow will:
   - Build binaries for all platforms
   - Create GitHub release with artifacts
   - Push `pup.rb` formula to `DataDog/homebrew-pack`

3. Users can install via:
   ```bash
   brew tap datadog/pack
   brew install pup
   ```

## Troubleshooting

### Error: "failed to publish artifacts: formula: authentication required"

**Cause**: The `HOMEBREW_TAP_TOKEN` is missing or invalid.

**Solution**:
1. Verify the secret exists: https://github.com/DataDog/pup/settings/secrets/actions
2. Check the token hasn't expired
3. Verify token has `Contents: Read and Write` permissions on `homebrew-pack`

### Error: "failed to publish artifacts: formula: repository not found"

**Cause**: The `homebrew-pack` repository doesn't exist or is private.

**Solution**:
1. Verify repository exists: https://github.com/DataDog/homebrew-pack
2. Ensure it's public or the PAT has access
3. Check the repository name is exactly `homebrew-pack` (case-sensitive)

### Error: "failed to push formula: permission denied"

**Cause**: The PAT doesn't have write permissions.

**Solution**:
1. Recreate the PAT with `Contents: Read and Write`
2. Update the `HOMEBREW_TAP_TOKEN` secret

### Formula not updating with new versions

**Cause**: GoReleaser might be skipping the formula update.

**Solution**:
1. Check the release workflow logs for errors
2. Ensure the tag matches the version pattern (e.g., `v1.2.3`)
3. Verify the `homebrew-pack` repository is not archived or locked

## Maintenance

### Token Expiration

If you set an expiration date on the PAT:
1. Set a calendar reminder 1 week before expiration
2. Generate a new token with the same permissions
3. Update the `HOMEBREW_TAP_TOKEN` secret

### Updating Formula Configuration

To modify the formula (e.g., add dependencies, change test commands):
1. Edit `.goreleaser.yml` → `brews` section
2. Test with a pre-release tag
3. The formula will auto-update on the next release

## Security Notes

- ✅ Use **fine-grained PATs** (not classic tokens) - more secure and scoped
- ✅ Limit token access to **only** `DataDog/homebrew-pack`
- ✅ Set reasonable expiration dates (1 year recommended)
- ✅ Never commit tokens to the repository
- ✅ Rotate tokens periodically

## References

- [GoReleaser Homebrew Tap Documentation](https://goreleaser.com/customization/homebrew/)
- [Homebrew Tap Creation Guide](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [GitHub Fine-grained PAT Documentation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token#creating-a-fine-grained-personal-access-token)
