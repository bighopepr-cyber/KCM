# Code Quality Checklist

## Rust Quality
- [ ] No unwrap() in production code
- [ ] No panic!() in production code
- [ ] No TODO/FIXME/HACK markers
- [ ] No placeholder implementations
- [ ] All public APIs return Result<T, KcmError>

## Naming
- [ ] Types: PascalCase
- [ ] Functions: snake_case
- [ ] Constants: SCREAMING_SNAKE_CASE
- [ ] Modules: snake_case

## Complexity
- [ ] Functions < 50 lines
- [ ] Cyclomatic complexity < 10
- [ ] No deeply nested code
- [ ] Clear control flow

## Clippy
- [ ] Zero clippy warnings
- [ ] Zero fmt diff
- [ ] All lints resolved

## Documentation
- [ ] All public functions have doc comments
- [ ] All public types have doc comments
- [ ] All unsafe blocks have SAFETY comments