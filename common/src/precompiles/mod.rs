//! Contains the [PrecompileOverride] trait implementation for the FPVM-accelerated precompiles.

use alloc::sync::Arc;
use kona_executor::PrecompileOverride;
use kona_mpt::{TrieDB, TrieDBFetcher, TrieDBHinter};
use revm::{
    handler::register::EvmHandler,
    precompile::{
        bn128, hash, identity, kzg_point_evaluation, modexp, Precompile, PrecompileResult,
        PrecompileSpecId, PrecompileWithAddress,
    },
    primitives::Bytes,
    ContextPrecompiles, State,
};

mod ecrecover;

macro_rules! create_annotated_precompile {
    ($precompile:expr, $name:expr) => {
        PrecompileWithAddress(
            $precompile.0,
            Precompile::Standard(|input: &Bytes, gas_limit: u64| -> PrecompileResult {
                let precompile = $precompile.precompile();
                match precompile {
                    Precompile::Standard(precompile) => {
                        println!(concat!("cycle-tracker-start: precompile-", $name));
                        let result = precompile(input, gas_limit);
                        println!(concat!("cycle-tracker-end: precompile-", $name));
                        result
                    }
                    _ => panic!("Annotated precompile must be a standard precompile."),
                }
            }),
        )
    };
}

pub(crate) const ANNOTATED_SHA256: PrecompileWithAddress =
    create_annotated_precompile!(hash::SHA256, "sha256");
pub(crate) const ANNOTATED_RIPEMD160: PrecompileWithAddress =
    create_annotated_precompile!(hash::RIPEMD160, "ripemd160");
pub(crate) const ANNOTATED_IDENTITY: PrecompileWithAddress =
    create_annotated_precompile!(identity::FUN, "identity");
pub(crate) const ANNOTATED_BN_ADD: PrecompileWithAddress =
    create_annotated_precompile!(bn128::add::ISTANBUL, "bn-add");
pub(crate) const ANNOTATED_BN_MUL: PrecompileWithAddress =
    create_annotated_precompile!(bn128::mul::ISTANBUL, "bn-mul");
pub(crate) const ANNOTATED_BN_PAIR: PrecompileWithAddress =
    create_annotated_precompile!(bn128::pair::ISTANBUL, "bn-pair");
pub(crate) const ANNOTATED_MODEXP: PrecompileWithAddress =
    create_annotated_precompile!(modexp::BERLIN, "modexp");
pub(crate) const ANNOTATED_KZG_POINT_EVAL: PrecompileWithAddress = create_annotated_precompile!(
    kzg_point_evaluation::POINT_EVALUATION,
    "kzg-point-evaluation"
);

/// The [PrecompileOverride] implementation for the FPVM-accelerated precompiles.
#[derive(Debug)]
pub struct ZKVMPrecompileOverride<F, H>
where
    F: TrieDBFetcher,
    H: TrieDBHinter,
{
    _phantom: core::marker::PhantomData<(F, H)>,
}

impl<F, H> Default for ZKVMPrecompileOverride<F, H>
where
    F: TrieDBFetcher,
    H: TrieDBHinter,
{
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData::<(F, H)>,
        }
    }
}

impl<F, H> PrecompileOverride<F, H> for ZKVMPrecompileOverride<F, H>
where
    F: TrieDBFetcher,
    H: TrieDBHinter,
{
    fn set_precompiles(handler: &mut EvmHandler<'_, (), &mut State<TrieDB<F, H>>>) {
        let spec_id = handler.cfg.spec_id;

        handler.pre_execution.load_precompiles = Arc::new(move || {
            let mut ctx_precompiles =
                ContextPrecompiles::new(PrecompileSpecId::from_spec_id(spec_id)).clone();

            // Extend with ZKVM-accelerated precompiles and annotated precompiles that track the cycle count.
            let override_precompiles = [
                ecrecover::ZKVM_ECRECOVER,
                // Start of annotated precompiles that simply track cycle count.
                ANNOTATED_SHA256,
                ANNOTATED_RIPEMD160,
                ANNOTATED_IDENTITY,
                ANNOTATED_BN_ADD,
                ANNOTATED_BN_MUL,
                ANNOTATED_BN_PAIR,
                ANNOTATED_MODEXP,
                ANNOTATED_KZG_POINT_EVAL,
            ];
            ctx_precompiles.extend(override_precompiles);

            ctx_precompiles
        });
    }
}
