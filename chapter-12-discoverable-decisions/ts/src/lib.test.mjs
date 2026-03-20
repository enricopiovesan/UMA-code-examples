import test from "node:test";
import assert from "node:assert/strict";
import { diffReports, loadReport, projectRoot } from "./lib.mjs";

test("capability projection starts opaque", () => {
  const report = loadReport(projectRoot(), "lab1-capability-projection");
  assert.equal(report.assessment.verdict, "opaque");
  assert.ok(report.assessment.warnings.some((warning) => warning.code === "proposal_hidden"));
});

test("authority feedback is discoverable", () => {
  const report = loadReport(projectRoot(), "lab3-authority-feedback");
  assert.equal(report.assessment.verdict, "discoverable");
  assert.equal(report.assessment.warnings.length, 0);
});

test("approved execution without full trace is flagged", () => {
  const report = loadReport(projectRoot(), "lab5-approved-execution");
  assert.ok(report.assessment.warnings.some((warning) => warning.code === "partial_trace"));
});

test("queryable trace is governed", () => {
  const report = loadReport(projectRoot(), "lab6-queryable-trace");
  assert.equal(report.assessment.verdict, "governed");
});

test("diff captures traceability shift", () => {
  const from = loadReport(projectRoot(), "lab5-approved-execution");
  const to = loadReport(projectRoot(), "lab6-queryable-trace");
  const diff = diffReports(from, to);
  assert.ok(diff.changedAxes.some((item) => item.includes("traceability")));
  assert.ok(diff.removedWarnings.includes("partial_trace"));
});
