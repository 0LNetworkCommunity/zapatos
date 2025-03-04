// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use anyhow::Result;
use aptos_state_view::TStateView;
use aptos_types::{
    access_path::AccessPath,
    state_store::{
        state_key::StateKey, state_storage_usage::StateStorageUsage, state_value::StateValue,
    },
};
use move_core_types::language_storage::ModuleId;
use std::collections::HashMap;

// `StateView` has no data given we are creating the genesis
pub struct GenesisStateView { //////// 0L //////// make public
    state_data: HashMap<StateKey, Vec<u8>>,
}

impl GenesisStateView {
    pub fn new() -> Self { //////// 0L //////// make public
        Self {
            state_data: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module_id: &ModuleId, blob: &[u8]) { //////// 0L //////// make public
        self.state_data.insert(
            StateKey::access_path(AccessPath::from(module_id)),
            blob.to_vec(),
        );
    }
}

impl TStateView for GenesisStateView {
    type Key = StateKey;

    fn get_state_value(&self, state_key: &StateKey) -> Result<Option<StateValue>> {
        Ok(self
            .state_data
            .get(state_key)
            .cloned()
            .map(StateValue::new_legacy))
    }

    fn is_genesis(&self) -> bool {
        true
    }

    fn get_usage(&self) -> Result<StateStorageUsage> {
        Ok(StateStorageUsage::zero())
    }
}
