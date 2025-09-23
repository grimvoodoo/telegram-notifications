# Codecov Setup

This project is configured to upload code coverage reports to Codecov, but requires setup of a repository secret.

## Quick Setup

1. **Sign up for Codecov**: Go to [codecov.io](https://codecov.io) and sign in with your GitHub account

2. **Add your repository**: Add `telegram-notifications` to your Codecov dashboard

3. **Get the repository token**: Copy the upload token from your Codecov repository settings

4. **Add GitHub secret**:
   - Go to your GitHub repository settings
   - Navigate to **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Name: `CODECOV_TOKEN`
   - Value: [paste your Codecov token]
   - Click **Add secret**

## Current Behavior

- ✅ **Coverage generation**: Works automatically in CI
- ✅ **CI continues**: Even if Codecov upload fails, CI won't fail
- ⚠️ **Upload optional**: Coverage uploads only if `CODECOV_TOKEN` is set

## Coverage Approach

The CI uses a fallback strategy for reliable coverage generation:

1. **Primary**: Standard coverage (unit tests + CLI tests) - most reliable
2. **Fallback 1**: Single-threaded with E2E tests if primary fails
3. **Fallback 2**: Library and binary code only if all else fails

This ensures coverage generation is robust and doesn't break CI pipelines.

## Coverage Scope

The coverage includes:
- ✅ All library code (`src/lib.rs`, `src/api.rs`, `src/config.rs`, etc.)
- ✅ Main binary code (`src/main.rs`)
- ✅ Unit tests (38 tests)
- ✅ CLI functionality tests
- ⚠️ E2E tests (when possible - these can be flaky in CI)

## Files Generated

- `lcov.info`: Coverage report in LCOV format for Codecov
- Coverage data includes line coverage and function coverage
- Typical file size: ~50KB with 1500+ lines of coverage data

## Without Codecov Token

If you don't want to use Codecov:
- The CI will still generate coverage locally
- Coverage upload will be skipped (with warning)
- All other CI jobs continue normally

## Alternative Coverage Tools

You can also use the generated `lcov.info` file with other tools:
- **Local HTML reports**: `genhtml lcov.info -o coverage-html`
- **VS Code**: Install Coverage Gutters extension
- **Other services**: Upload to Coveralls, CodeClimate, etc.

## Troubleshooting

### "Token required" error
- Add the `CODECOV_TOKEN` secret as described above
- The current workflow won't fail CI even with this error

### Coverage generation fails
- Check the CI logs for specific error messages
- The workflow has multiple fallback approaches
- Local generation using `./generate-coverage.sh` can help debug

### Low coverage numbers
- E2E tests might be skipped in CI (this is normal)
- Main application logic should still have good coverage
- Unit tests provide the most reliable coverage metrics