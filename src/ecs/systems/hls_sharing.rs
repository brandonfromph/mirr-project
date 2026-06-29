#![forbid(unsafe_code)]

use crate::ecs::components::HlsBindingComponent;
use crate::ecs::Registry;

/// ECS System: Resource Sharing.
///
/// Groups compatible operations (same resource kind, non-overlapping cycles)
/// and allocates physical resources to them. Writes to `HlsBindingComponent`.
pub fn hls_sharing_system(registry: &mut Registry) {
    let max_id = registry.active_entities();

    // Group operations by resource kind and store their earliest cycle
    let mut ops_by_kind: std::collections::HashMap<crate::hls::ResourceKind, Vec<(usize, u32)>> =
        std::collections::HashMap::new();

    for i in 0..max_id {
        if let Some(sched) = &registry.hls_schedules[i] {
            ops_by_kind.entry(sched.resource).or_default().push((i, sched.earliest));
        }
    }

    // Allocate physical instances for each kind
    let mut next_physical_id = 0;

    for (_kind, mut ops) in ops_by_kind {
        // Sort operations by earliest cycle to greedily pack them
        ops.sort_by_key(|&(_, earliest)| earliest);

        let mut physical_instances: Vec<Vec<(usize, u32)>> = Vec::new();

        for &(op, earliest) in &ops {
            let mut allocated = false;

            // Try to pack into an existing physical instance
            for instance in &mut physical_instances {
                let mut overlaps = false;
                for &(_other_op, other_earliest) in instance.iter() {
                    // Basic overlap check: if they can execute in the same cycle, they overlap
                    if earliest == other_earliest {
                        overlaps = true;
                        break;
                    }
                }

                if !overlaps {
                    instance.push((op, earliest));
                    allocated = true;
                    break;
                }
            }

            // If it couldn't be packed, allocate a new physical instance
            if !allocated {
                physical_instances.push(vec![(op, earliest)]);
            }
        }

        // Assign the physical resource IDs to the components
        for instance in physical_instances {
            let instance_id = next_physical_id;
            next_physical_id += 1;

            for &(op, _) in &instance {
                registry.hls_bindings[op] =
                    Some(HlsBindingComponent { physical_resource_id: instance_id });
            }
        }
    }
}
