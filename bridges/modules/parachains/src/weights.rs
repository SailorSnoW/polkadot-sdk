// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_parachains`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-06, STEPS: 50, REPEAT: 20
//! LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled
//! CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_parachains
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/parachains/src/weights.rs
// --template=./.maintain/millau-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for `pallet_bridge_parachains`.
pub trait WeightInfo {
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight;
	fn submit_parachain_heads_with_1kb_proof() -> Weight;
	fn submit_parachain_heads_with_16kb_proof() -> Weight;
}

/// Weights for `pallet_bridge_parachains` using the Millau node and recommended hardware.
pub struct MillauWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for MillauWeight<T> {
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight {
		(0 as Weight)
			.saturating_add((18_706_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(p as Weight)))
	}
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		(27_549_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		(80_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight {
		(0 as Weight)
			.saturating_add((18_706_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(p as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(p as Weight)))
	}
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		(27_549_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		(80_792_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
