//! Contains the [PrecompileOverride] trait implementation for the FPVM-accelerated precompiles.
use alloc::sync::Arc;
use kona_executor::{TrieDB, TrieDBProvider};
use kona_mpt::TrieHinter;
use revm::{
    db::states::state::State,
    handler::register::EvmHandler,
    precompile::{bn128, Precompile, PrecompileResult, PrecompileWithAddress},
    primitives::{spec_to_generic, Bytes, SpecId},
};

/// Create an annotated precompile that simply tracks the cycle count of a precompile.
macro_rules! create_annotated_precompile {
    ($precompile:expr, $name:expr) => {
        PrecompileWithAddress(
            $precompile.0,
            Precompile::Standard(|input: &Bytes, gas_limit: u64| -> PrecompileResult {
                let precompile = $precompile.precompile();
                match precompile {
                    Precompile::Standard(precompile) => {
                        #[cfg(target_os = "zkvm")]
                        println!(concat!("cycle-tracker-report-start: precompile-", $name));

                        let result = precompile(input, gas_limit);

                        #[cfg(target_os = "zkvm")]
                        println!(concat!("cycle-tracker-report-end: precompile-", $name));

                        result
                    }
                    _ => panic!("Annotated precompile must be a standard precompile."),
                }
            }),
        )
    };
}

pub(crate) const ANNOTATED_BN_ADD: PrecompileWithAddress =
    create_annotated_precompile!(bn128::add::ISTANBUL, "bn-add");
pub(crate) const ANNOTATED_BN_MUL: PrecompileWithAddress =
    create_annotated_precompile!(bn128::mul::ISTANBUL, "bn-mul");
pub(crate) const ANNOTATED_BN_PAIR: PrecompileWithAddress =
    create_annotated_precompile!(bn128::pair::ISTANBUL, "bn-pair");
pub(crate) const ANNOTATED_EC_RECOVER: PrecompileWithAddress =
    create_annotated_precompile!(revm::precompile::secp256k1::ECRECOVER, "ec-recover");
pub(crate) const ANNOTATED_P256_VERIFY: PrecompileWithAddress =
    create_annotated_precompile!(revm::precompile::secp256r1::P256VERIFY, "p256-verify");
pub(crate) const ANNOTATED_KZG_EVAL: PrecompileWithAddress = create_annotated_precompile!(
    revm::precompile::kzg_point_evaluation::POINT_EVALUATION,
    "kzg-eval"
);

// Source: https://github.com/anton-rs/kona/blob/main/bin/client/src/fault/handler/mod.rs#L20-L42
pub fn zkvm_handle_register<F, H>(handler: &mut EvmHandler<'_, (), &mut State<&mut TrieDB<F, H>>>)
where
    F: TrieDBProvider,
    H: TrieHinter,
{
    let spec_id = handler.cfg.spec_id;

    handler.pre_execution.load_precompiles = Arc::new(move || {
        let mut ctx_precompiles = spec_to_generic!(spec_id, {
            revm::optimism::load_precompiles::<SPEC, (), &mut State<&mut TrieDB<F, H>>>()
        });

        // Extend with ZKVM-accelerated precompiles and annotated precompiles that track the
        // cycle count.
        let override_precompiles = [
            ANNOTATED_BN_ADD,
            ANNOTATED_BN_MUL,
            ANNOTATED_BN_PAIR,
            ANNOTATED_EC_RECOVER,
            ANNOTATED_P256_VERIFY,
            ANNOTATED_KZG_EVAL,
        ];
        ctx_precompiles.extend(override_precompiles);

        ctx_precompiles
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;
    use revm::{
        db::BenchmarkDB,
        interpreter::opcode::{PUSH1, SSTORE},
        precompile::u64_to_address,
        primitives::{Address, Bytecode, U256},
        Context, Database,
    };

    fn setup_evm() -> EVM<EthersDB> {
        let bytecode = Bytecode::new_legacy([PUSH1, 0x01, PUSH1, 0x01, SSTORE].into());
        let ctx = Context::mainnet()
            .modify_cfg_chained(|cfg| cfg.spec = SpecId::PRAGUE)
            .with_db(BenchmarkDB::new_bytecode(bytecode))
            .modify_tx_chained(|tx| {
                tx.tx_type = TransactionType::Eip7702.into();
                tx.gas_limit = 100_000;
                tx.authorization_list = vec![auth];
                tx.caller = EEADDRESS;
                tx.kind = TxKind::Call(signer.address());
            });

        let mut evm = ctx.build_mainnet();
    }

    #[test]
    fn test_precompiles() {
        let mut evm = setup_evm();

        // Test BN_ADD
        let bn_add_input = hex!(
            "
            18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9
            063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266
            08b2294b7a1259ccaa000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000"
        );
        let result = evm.transact_ref(u64_to_address(6), bn_add_input.into(), 0.into());
        assert!(result.is_ok(), "BN_ADD failed");

        // Test BN_MUL
        let bn_mul_input = hex!(
            "
            2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb7
            21611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204
        "
        );
        let result = evm.transact_ref(u64_to_address(7), bn_mul_input.into(), 0.into());
        assert!(result.is_ok(), "BN_MUL failed");

        // Test EC_RECOVER
        let ec_recover_input = hex!(
            "
            47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad
            000000000000000000000000000000000000000000000000000000000000001b
            650acf9d3f5f0a2c799776a1254355d5f4061762a237396a99a0e0e3fc2bcd6
            207c9ece04a9b5ef3d1c1dc2fb4d00d3f1aa2cbec6db7c9e2b1b1a8c24813d0
        "
        );
        let result = evm.transact_ref(u64_to_address(1), ec_recover_input.into(), 0.into());
        assert!(result.is_ok(), "EC_RECOVER failed");

        // Test P256_VERIFY
        let p256_verify_input = hex!(
            "
            79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798
            483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8
            e28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68
            554199c47d08ffb10d4b8e28d959f2815b16f81798483ada7726a3c4655da4f
        "
        );
        let result = evm.transact_ref(u64_to_address(0xF), p256_verify_input.into(), 0.into());
        assert!(result.is_ok(), "P256_VERIFY failed");

        // Test KZG_EVAL
        let kzg_eval_input = hex!(
            "
            1234567890123456789012345678901234567890123456789012345678901234
            5678901234567890123456789012345678901234567890123456789012345678
        "
        );
        let result = evm.transact_ref(u64_to_address(0x14), kzg_eval_input.into(), 0.into());
        assert!(result.is_ok(), "KZG_EVAL failed");
    }
}
