// Copyright (C) 2023 Subspace Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Primitives for Receipts.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_core::H256;
use sp_domains::fraud_proof::FraudProof;
use sp_domains::{DomainId, ExecutionReceipt};
use sp_runtime::traits::NumberFor;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait SettlementApi<DomainHash: Encode + Decode> {
        /// Returns the trace of given domain receipt hash.
        fn execution_trace(domain_id: DomainId, receipt_hash: H256) -> Vec<DomainHash>;

        /// Returns the state root of given domain block.
        fn state_root(
            domain_id: DomainId,
            domain_block_number: NumberFor<Block>,
            domain_block_hash: Block::Hash,
        ) -> Option<DomainHash>;

        /// Returns the primary block hash for given domain block number.
        fn primary_hash(
            domain_id: DomainId,
            domain_block_number: NumberFor<Block>,
        ) -> Option<Block::Hash>;

        /// Returns the receipts pruning depth.
        fn receipts_pruning_depth() -> NumberFor<Block>;

        /// Returns the best execution chain number.
        fn head_receipt_number(domain_id: DomainId) -> NumberFor<Block>;

        /// Returns the block number of oldest execution receipt.
        fn oldest_receipt_number(domain_id: DomainId) -> NumberFor<Block>;

        /// Returns the maximum receipt drift.
        fn maximum_receipt_drift() -> NumberFor<Block>;

        /// Extract the receipts from the given extrinsics.
        fn extract_receipts(
            extrinsics: Vec<Block::Extrinsic>,
            domain_id: DomainId,
        ) -> Vec<ExecutionReceipt<NumberFor<Block>, Block::Hash, DomainHash>>;

        /// Extract the fraud proofs from the given extrinsics.
        fn extract_fraud_proofs(
            extrinsics: Vec<Block::Extrinsic>,
            domain_id: DomainId,
        ) -> Vec<FraudProof<NumberFor<Block>, Block::Hash>>;

        /// Submits the fraud proof via an unsigned extrinsic.
        fn submit_fraud_proof_unsigned(fraud_proof: FraudProof<NumberFor<Block>, Block::Hash>);
    }
}
