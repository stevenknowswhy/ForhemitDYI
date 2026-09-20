# Architecture-analysis artifacts

`GAP-ANALYSIS.md` is the versioned result of `scripts/doc-graph.py`. CI compares it with the current design corpus and fails when it is stale.

`graph.json` and `engine-graph.mmd` are local generated views and remain ignored.

## Archive policy

`archive/` is an intentional design-history record, not a cache:

- archive an outgoing gap analysis only when a finalized architecture change materially changes the report
- do not publish during intermediate editing or parser experiments
- validate intermediate work with `python3 scripts/doc-graph.py --check`
- publish once with `python3 scripts/doc-graph.py` after the change is final
- do not keep duplicate or timestamp-only snapshots
- use Git history for line-level reconstruction; use the archive only for named report states that were actually reviewed or shipped

Existing snapshots are retained because they record the concentrated architecture reconciliation performed on 2026-09-19 and 2026-09-20. Future snapshots follow the policy above rather than being created on every local run.