import test from "node:test";
import assert from "node:assert/strict";
import { diffReports, loadReport, projectRoot } from "./lib.mjs";

test("baseline is coherent", () => {
  const report = loadReport(projectRoot(), "lab1-contract-anchor");
  assert.equal(report.assessment.verdict, "coherent");
  assert.equal(report.assessment.warnings.length, 0);
});

test("drift lab reports behavioral drift", () => {
  const report = loadReport(projectRoot(), "lab2-behavioral-drift");
  assert.ok(report.assessment.warnings.some((warning) => warning.code === "behavioral_drift"));
});

test("version sprawl lab is fragmented", () => {
  const report = loadReport(projectRoot(), "lab4-version-sprawl");
  assert.equal(report.assessment.verdict, "fragmented");
});

test("runtime governed coexistence is governed", () => {
  const report = loadReport(projectRoot(), "lab5-runtime-governed-coexistence");
  assert.equal(report.assessment.verdict, "governed");
});

test("diff captures versioning shift", () => {
  const from = loadReport(projectRoot(), "lab4-version-sprawl");
  const to = loadReport(projectRoot(), "lab5-runtime-governed-coexistence");
  const diff = diffReports(from, to);
  assert.ok(diff.changedAxes.some((item) => item.includes("versioning")));
  assert.ok(diff.removedWarnings.includes("version_fragmentation"));
});

test("unknown lab error lists available options", () => {
  assert.throws(
    () => loadReport(projectRoot(), "does-not-exist"),
    /unknown lab "does-not-exist".*lab1-contract-anchor/
  );
});
