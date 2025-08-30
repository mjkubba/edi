# Implementation Guidelines

## Code Structure and Compatibility

When implementing new code:
- Keep the current structure and mimic existing patterns
- Don't create new structures unless absolutely necessary
- Ensure changes to existing code don't impact existing functionality
- Maintain backward compatibility at all times

## Rust Best Practices

### File Operations

When working with file operations in Rust:

1. Prefer using the simpler `fs::read_to_string()` and `fs::write()` functions over verbose `File::open()` + `read_to_string()` or `File::create()` + `write_all()` combinations
2. Avoid the `#[allow(clippy::verbose_file_reads)]` annotation by using the recommended methods
3. Use `serde_json::to_string_pretty()` + `fs::write()` instead of creating a file and then writing to it with `serde_json::to_writer_pretty()`
4. Keep imports organized by functionality (e.g., group path-related imports together)

## Git Workflow

### Committing Changes

Follow the git best practice of committing early and often. Run `git commit` often, but DO NOT ever run `git push`.

BEFORE committing a change, ALWAYS do the following steps:

1. Run `cargo build` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
2. Run `cargo test` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
3. Run `cargo +nightly fmt` to auto-format the code
4. Commit the changes

### Commit Messages

All commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

🤖 Assisted by Kiro
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools
- `ci`: Changes to CI configuration files and scripts

**Best practices:**
- Use the imperative mood ("add" not "added" or "adds")
- Don't end the subject line with a period
- Limit the subject line to 50 characters
- Capitalize the subject line
- Separate subject from body with a blank line
- Use the body to explain what and why vs. how
- Wrap the body at 72 characters

**Example:**
```
feat(edi835): Add support for additional loop structures

Implement Loop2100 and Loop2110 processing to handle provider
and service line information. Maintains backward compatibility
with existing parsing logic.

🤖 Assisted by Kiro
```