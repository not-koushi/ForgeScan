import * as fs from "fs";

type Finding = {
  category: string;
  severity: "low" | "medium" | "high";
  path?: string;
  entropy?: number;
  package?: string;
  similar_to?: string;
};

type Report = {
  tool: string;
  version: string;
  findings: Finding[];
};

const input = process.argv[2];

if (!input) {
  console.error("Usage: node formatReport.js <report.json>");
  process.exit(1);
}

const raw = fs.readFileSync(input, "utf-8");
const report: Report = JSON.parse(raw);

console.log(`\nForgeScan Report (${report.version})\n`);

for (const f of report.findings) {
  if (f.category === "obfuscation") {
    console.log(
      `[${f.severity.toUpperCase()}] ${f.path} (entropy: ${f.entropy?.toFixed(2)})`
    );
  } else if (f.category === "typosquatting") {
    console.log(
      `[MEDIUM] Package "${f.package}" resembles "${f.similar_to}"`
    );
  }
}