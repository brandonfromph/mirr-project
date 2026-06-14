//! Pipeline summary printing.

#![forbid(unsafe_code)]

pub(super) fn print_summary(result: &mirrc::pipeline::PipelineResult, show_stats: bool) {
    let module = &result.program.module;
    eprintln!("MIRR Compile: {}", module.name);
    eprintln!(
        "  Signals: {}  Guards: {}  Reflexes: {}",
        module.signals.len(),
        module.guards.len(),
        module.reflexes.len(),
    );

    if let Some(ss) = &result.simplify_stats {
        eprintln!(
            "  Simplify: {} rules applied, {} -> {} nodes",
            ss.rules_applied, ss.nodes_before, ss.nodes_after,
        );
    }

    if let Some(wr) = &result.width_stats {
        let diag_count = wr.diagnostics_count;
        let scc_count = wr.scc_count;
        eprintln!("  Width: {diag_count} diagnostics, {scc_count} SCCs");
    }

    if let Some(tn) = &result.temporal_netlist {
        eprintln!("  Temporal: {} guards, {} signals", tn.guards.len(), tn.signals.len(),);
    }

    if let Some(tr) = &result.totality_result {
        let status = if tr.is_total { "TOTAL" } else { "NOT TOTAL" };
        eprintln!(
            "  Totality: {} (bounds: {}, completeness: {}, coverage: {}, acyclicity: {})",
            status,
            tr.resource_bound.pass,
            tr.output_completeness.pass,
            tr.guard_coverage.pass,
            tr.acyclicity.pass,
        );
    }

    if let Some(sr) = &result.symbolic_result {
        let status = if sr.converged { "converged" } else { "did not converge" };
        eprintln!(
            "  Symbolic: {} iterations, {} violations, {}",
            sr.iterations,
            sr.violations.len(),
            status,
        );
    }

    if show_stats {
        if let Some(wr) = &result.width_stats {
            eprintln!(
                "  [stats] nodes_analyzed={} rounds={} sccs={} expansive={} nonexpansive={}",
                wr.nodes_analyzed,
                wr.propagation_rounds,
                wr.scc_count,
                wr.expansive_count,
                wr.nonexpansive_count,
            );
        }
    }
}
