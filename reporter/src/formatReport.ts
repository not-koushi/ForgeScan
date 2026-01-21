type Finding = {
  category: string;
  message: string;
};

export function formatReport(findings: Finding[]) {
  console.log("ForgeScan Security Report");
  console.table(findings);
}