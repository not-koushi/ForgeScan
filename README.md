# ForgeScan

ForgeScan is a supply-chain security scanner designed to detect two common classes of risks in JavaScript/npm projects:

1. **Typo-squatting dependencies** in `package.json`
2. **Suspiciously obfuscated source code** using Shannon entropy analysis

ForgeScan is intentionally split into:
- A **Rust-based scanning engine** (fast, deterministic, CI-friendly)
- A **TypeScript-based reporter** (human-readable output)

The tool scans a target project *before installation*, making it suitable for pre-install, pre-commit, or CI use.

---

## Features

- Detects typo-squatting dependencies using edit-distance heuristics
- Flags anomalous JavaScript obfuscation via Shannon entropy
- Emits clean, structured JSON output
- Optional human-readable reporting via TypeScript
- Works on Windows without requiring Docker or WSL
- Does **not** execute or install scanned dependencies

---

## Prerequisites (Windows)

Install the following before proceeding:

- **Git**
- **Rust (stable)**  
  https://www.rust-lang.org/tools/install
- **Node.js (LTS)**  
  https://nodejs.org/

Verify installations:

```powershell
rustc --version
cargo --version
node --version
npm --version
```

---

## Download and Setup

Clone the repository:

```powershell
git clone https://github.com/not-koushi/ForgeScan.git
cd ForgeScan
```

---

## Install Reporter Dependencies

The reporter is optional but recommended for readable output.

```powershell
cd reporter 
npm install
cd ..
```

## Prepare a Scan Target

ForgeScan scans **other projects**, not itself.

This repository includes a ready-to-use demo target in `scan-target/`.

⚠️ Do not run `npm install` **inside** `scan-target`/
The scan target intentionally contains fake dependencies to demonstrate typo-squatting detection.

---

## Running ForgeScan (Windows)

**Step 1:** Move into the Rust engine

```powershell
cd engine
```

**Step 2:** Run a full scan and emit JSON

```powershell
cargo run -- ..\scan-target\src --deps --json |
    Out-File -Encoding utf8 ..\report.json
```
What this does:
- Scans JavaScript source files for obfuscation
- Scans `package.json` for typo-squatting
- Emits **pure JSON** (no extra logs)
- Write results to `report.json`

**Step 3:** Return to project root

```powershell
cd ..
```

---

## Viewing the Results

**Option A:** Inspet Raw JSON

```powershell
Get-Content report.json
```

The JSON output is suitable for CI pipelines and automation.

**Option B:** Run the TypeScript Reporter (Recommended)

```powershell
npx ts-node reporter/src/formatReport.ts report.json
```

*Example Output:*

```text
ForgeScan Report (1.0.0)

[HIGH] scan-target/src/obfuscated-high.js (entropy: 6.3)
[MEDIUM] scan-target/src/packed-medium.js (entropy: 5.0)
[MEDIUM] scan-target/src/suspicious.js (entropy: 4.9)
[MEDIUM] Package "expres" resembles "express"
[MEDIUM] Package "lodas" resembles "lodash"
```

---

## Common Usage Patterns

**Scan source code only**

```powershell
cargo run -- ../scan-target/src
```

**Scan source code and dependencies**

```powershell
cargo run -- ../scan-target/src --deps
```

**JSON-only output (CI Usage)**

```powershell
cargo run -- ../scan-target/src --deps --json
```

---

## Important Notes
- `report.json` is **generated output** and should not be committed
- `node_modules/` is never requried for scanning
- Dependency checks operate on `package.json` only
- ForgeScan is a **static analyzer** and does not execute code

---

## When to use ForgeScan

ForgeScan is best suited for:
- Pre-install dependency checks
- Pre-commit or pre-merge validation
- CI pipelines
- Security demonstrations and research

It is **not** a runtime malware detector.

---

## Author
*Built by Koushik Panchadarla*