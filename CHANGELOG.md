## 0.10.0

### Breaking Changes

- genericize parsing errors and remove error Strings
- fix all compilation errors from error refactor

### Features

- impl FromStr for MemoryReference

### Fixes

- support escaped double quotes and backslashes in strings (#120)
- fix performance regression (#113)
- do not get line/column info for tokens except on error
- require dynamic error to by Send (#108)
- bump thiserror version and update import name (#103)
- identifier parser (#100)
- test cases with rstest
- test cases should not violate the spec
- remove a `dbg!` statement left over from #88
- use structured error
- update node version and dependencies for semantic-release (#84)
- update semantic-release version as per dependabot suggestion (#83)
- DEFCAL MEASURE serialization
- test roundtrip of program->string->program
- linting
- Instruction used/blocked frames calculation (#74)

## 0.15.0-rc.0

### Breaking Changes

- genericize parsing errors and remove error Strings
- fix all compilation errors from error refactor

### Features

- impl FromStr for MemoryReference

### Fixes

- support escaped double quotes and backslashes in strings (#120)
- fix performance regression (#113)
- do not get line/column info for tokens except on error
- require dynamic error to by Send (#108)
- bump thiserror version and update import name (#103)
- identifier parser (#100)
- test cases with rstest
- test cases should not violate the spec
- remove a `dbg!` statement left over from #88
- use structured error
- update node version and dependencies for semantic-release (#84)
- update semantic-release version as per dependabot suggestion (#83)
- DEFCAL MEASURE serialization
- test roundtrip of program->string->program
- linting
- Instruction used/blocked frames calculation (#74)