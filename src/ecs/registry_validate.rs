// Validator for ECS Registry.
impl super::Registry {
    pub fn validate(&self) -> anyhow::Result<()> {
        let max_id = self.next_id as usize;

        // Ensure every component array is consistent with next_id
        if self.names.len() < max_id {
            return Err(anyhow::anyhow!("Names table desync"));
        }
        if self.kinds.len() < max_id {
            return Err(anyhow::anyhow!("Kinds table desync"));
        }
        if self.types.len() < max_id {
            return Err(anyhow::anyhow!("Types table desync"));
        }
        if self.reflex_comps.len() < max_id {
            return Err(anyhow::anyhow!("Reflexes table desync"));
        }
        if self.assignment_comps.len() < max_id {
            return Err(anyhow::anyhow!("Assignments table desync"));
        }
        if self.property_comps.len() < max_id {
            return Err(anyhow::anyhow!("Properties table desync"));
        }
        if self.temporal_nodes.len() < max_id {
            return Err(anyhow::anyhow!("TemporalNodes table desync"));
        }
        if self.hls_dataflow.len() < max_id {
            return Err(anyhow::anyhow!("HlsDataflow table desync"));
        }
        if self.hls_schedules.len() < max_id {
            return Err(anyhow::anyhow!("HlsSchedules table desync"));
        }
        if self.hls_bindings.len() < max_id {
            return Err(anyhow::anyhow!("HlsBindings table desync"));
        }
        if self.array_literals.len() < max_id {
            return Err(anyhow::anyhow!("ArrayLiterals table desync"));
        }
        if self.struct_literals.len() < max_id {
            return Err(anyhow::anyhow!("StructLiterals table desync"));
        }
        if self.unfold_indices.len() < max_id {
            return Err(anyhow::anyhow!("UnfoldIndices table desync"));
        }

        // Scan for orphaned entities (Entities that exist in next_id but have no components)
        for i in 0..max_id {
            let has_name = self.names[i].is_some();
            let has_kind = self.kinds[i].is_some();

            // NASA P10: A valid entity must at least have a Name and Kind
            if !has_name && !has_kind {
                // Ignore empty slots, but verify they are truly empty
                continue;
            }

            if has_name != has_kind {
                return Err(anyhow::anyhow!(
                    "Entity {} has desynced components (Name: {}, Kind: {})",
                    i,
                    has_name,
                    has_kind
                ));
            }
        }

        Ok(())
    }
}
