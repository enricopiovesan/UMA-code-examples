import test from "node:test";
import assert from "node:assert/strict";
import { diffReports, loadReport, projectRoot } from "./lib.mjs";

test("baseline is coherent", () => {
  const report = loadReport(projectRoot(), "lab1-baseline");
  assert.equal(report.assessment.verdict, "coherent");
  assert.equal(report.assessment.warnings.length, 0);
});

test("over granular lab warns", () => {
  const report = loadReport(projectRoot(), "lab2-over-granular");
  assert.ok(report.assessment.warnings.some((warning) => warning.code === "over_granular"));
});

test("runtime ambiguity is explicit", () => {
  const report = loadReport(projectRoot(), "lab4-runtime-ambiguity");
  assert.equal(report.assessment.verdict, "ambiguous");
});

test("diff reports added warning", () => {
  const from = loadReport(projectRoot(), "lab1-baseline");
  const to = loadReport(projectRoot(), "lab5-over-orchestrated");
  const diff = diffReports(from, to);
  assert.ok(diff.addedWarnings.includes("over_orchestrated"));
});

test("over orchestrated lab stays focused on orchestration", () => {
  const report = loadReport(projectRoot(), "lab5-over-orchestrated");
  const warningCodes = report.assessment.warnings.map((warning) => warning.code);
  assert.ok(warningCodes.includes("over_orchestrated"));
  assert.ok(warningCodes.includes("metadata_bloat"));
  assert.ok(warningCodes.includes("state_drift"));
  assert.ok(!warningCodes.includes("over_granular"));
});

test("unknown lab error lists available options", () => {
  assert.throws(
    () => loadReport(projectRoot(), "does-not-exist"),
    /unknown lab "does-not-exist".*lab1-baseline/
  );
});
