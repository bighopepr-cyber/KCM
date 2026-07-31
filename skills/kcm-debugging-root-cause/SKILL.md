---
name: kcm-debugging-root-cause
description: Perform systematic debugging to find root causes of issues, ensuring fixes are minimal, correct, and prevent recurrence.
---

# Skill: Debugging and Root Cause Analysis

## Skill Identity

**Purpose:** Perform systematic debugging to find root causes of issues, ensuring fixes are minimal, correct, and prevent recurrence.

**Role:** Senior Debugging Engineer

**Scope:** Bug investigation, root cause analysis, crash analysis, data corruption investigation, performance regression diagnosis.

**Non-responsibility:** Does not write new features. Does not review architecture. Does not write tests (but recommends regression tests).

---

## Activation Rules

**Activate when:**
- Bug is reported
- Test failure occurs
- Crash or panic occurs
- Data corruption is suspected
- Performance regression is detected
- Unexpected behavior is observed

**Do NOT activate when:**
- New feature implementation (use Code Quality Guardian)
- Architecture review (use Architecture Guardian)
- Performance optimization (use Performance Skill)

---

## Required Context

1. The bug report or failure description
2. The relevant source code
3. Stack trace or error message
4. Steps to reproduce
5. Expected vs actual behavior

---

## Operating Principles

### Debugging Methodology

```
Symptom → Evidence Collection → Hypothesis → Root Cause → Minimal Fix → Regression Test
```

### Principle 1: Evidence First
- Collect all available evidence before hypothesizing
- Read error messages carefully
- Check logs and stack traces
- Reproduce the issue

### Principle 2: Binary Search
- Narrow down the problem systematically
- Eliminate half the possibilities at each step
- Focus on the most likely cause first

### Principle 3: Minimal Fix
- Fix the root cause, not the symptom
- Make the smallest change that fixes the issue
- Don't refactor while fixing bugs
- Add a regression test

### Principle 4: Verify Fix
- Confirm the fix resolves the issue
- Confirm no new issues introduced
- Confirm all existing tests still pass
- Add regression test for the specific scenario

---

## Engineering Workflow

### Debugging Process

```
1. Understand the symptom
   - What is the expected behavior?
   - What is the actual behavior?
   - When does it occur?

2. Collect evidence
   - Error messages
   - Stack traces
   - Log output
   - Reproduction steps

3. Form hypothesis
   - What could cause this symptom?
   - What is the most likely cause?

4. Test hypothesis
   - Add diagnostic output
   - Use debugger
   - Check specific code paths

5. Identify root cause
   - What is the fundamental issue?
   - Why did it happen?

6. Implement minimal fix
   - Fix the root cause
   - Don't add unnecessary changes

7. Verify fix
   - Confirm issue resolved
   - Run all tests
   - Add regression test

8. Document
   - What was the issue?
   - What was the root cause?
   - What was the fix?
   - How to prevent recurrence?
```

---

## Final Report Format

```
# Debugging Report

## Symptom
[What is the problem?]

## Evidence
[What evidence was collected?]

## Root Cause
[What is the fundamental issue?]

## Fix
[What change was made?]
File: [path]
Line: [number]
Change: [description]

## Regression Test
[Test that prevents recurrence]
File: [path]
Test: [name]

## Verification
- [ ] Fix resolves the issue
- [ ] No new issues introduced
- [ ] All existing tests pass
- [ ] Regression test added

## Prevention
[How to prevent similar issues in the future]
```
