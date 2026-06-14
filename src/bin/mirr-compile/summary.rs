//! Pipeline summary printing.

#![forbid(unsafe_code)]

pub(super) fn print_summary(result: &mirrc::pipeline::PipelineResult, show_stats: bool) {
    let registry = result.ecs_registry.as_ref().expect("ECS registry required");
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    let mut signal_count = 0;
    let mut guard_count = 0;
    let mut reflex_count = 0;

    for kind_comp in registry.kinds.iter().flatten() {
        match kind_comp.0 {
            mirrc::ecs::EntityKind::SIGNAL(_) => signal_count += 1,
            mirrc::ecs::EntityKind::GUARD => guard_count += 1,
            mirrc::ecs::EntityKind::REFLEX => reflex_count += 1,
            _ => {}
        }
    }

    eprintln!("MIRR Compile: {}", module_name);
    eprintln!("  Signals: {}  Guards: {}  Reflexes: {}", signal_count, guard_count, reflex_count,);

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
