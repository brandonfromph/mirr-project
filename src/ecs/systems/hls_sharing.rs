#![forbid(unsafe_code)]

use crate::ecs::components::HlsBindingComponent;
use crate::ecs::Registry;

/// ECS System: Resource Sharing.
///
/// Groups compatible operations (same resource kind, non-overlapping cycles)
/// and allocates physical resources to them. Writes to `HlsBindingComponent`.
pub fn hls_sharing_system(registry: &mut Registry) {
    let max_id = registry.active_entities();

    // Group operations by resource kind
    let mut ops_by_kind: std::collections::HashMap<crate::hls::ResourceKind, Vec<usize>> =
        std::collections::HashMap::new();

    for i in 0..max_id {
        if let Some(sched) = &registry.hls_schedules[i] {
            ops_by_kind.entry(sched.resource).or_default().push(i);
        }
    }

    // Allocate physical instances for each kind
    let mut next_physical_id = 0;

    for (_kind, mut ops) in ops_by_kind {
        // Sort operations by earliest cycle to greedily pack them
        ops.sort_by_key(|&i| registry.hls_schedules[i].as_ref().unwrap().earliest);

        let mut physical_instances: Vec<Vec<usize>> = Vec::new();

        for &op in &ops {
            let op_sched = registry.hls_schedules[op].as_ref().unwrap();
            let mut allocated = false;

            // Try to pack into an existing physical instance
            for instance in &mut physical_instances {
                let mut overlaps = false;
                for &other_op in instance.iter() {
                    let other_sched = registry.hls_schedules[other_op].as_ref().unwrap();
                    // Basic overlap check: if they can execute in the same cycle, they overlap
                    // Using earliest for now, but should ideally use the actual scheduled cycle if we implement mid-cycle scheduling
                    if op_sched.earliest == other_sched.earliest {
                        overlaps = true;
                        break;
                    }
                }

                if !overlaps {
                    instance.push(op);
                    allocated = true;
                    break;
                }
            }

            // If it couldn't be packed, allocate a new physical instance
            if !allocated {
                physical_instances.push(vec![op]);
            }
        }

        // Assign the physical resource IDs to the components
        for instance in physical_instances {
            let instance_id = next_physical_id;
            next_physical_id += 1;

            for &op in &instance {
                registry.hls_bindings[op] =
                    Some(HlsBindingComponent { physical_resource_id: instance_id });
            }
        }
    }
}
