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
