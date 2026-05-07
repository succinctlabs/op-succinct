//! [`PrecompileProvider`] for FPVM-accelerated OP Stack precompiles.

use alloc::string::String;
use alloy_primitives::{Address, Bytes};
use op_revm::{precompiles::OpPrecompiles, OpSpecId};
use revm::{
    context::{Cfg, ContextTr},
    context_interface::JournalTr,
    handler::{EthPrecompiles, PrecompileProvider},
    interpreter::{CallInput, CallInputs, Gas, InstructionResult, InterpreterResult},
    precompile::PrecompileError,
};
#[cfg(any(test, target_os = "zkvm"))]
use revm_precompile::PrecompileId;

mod custom;
pub use custom::CustomCrypto;

mod factory;
pub use factory::ZkvmOpEvmFactory;

/// Tracker names for accelerated precompiles.
/// These names are used in cycle-tracker-report events and must match
/// the keys expected by stats.rs and validity/src/types.rs.
pub mod cycle_tracker {
    /// Prefix for all precompile cycle tracker keys.
    pub const PREFIX: &str = "precompile-";

    /// Individual tracker names (without prefix).
    pub mod names {
        pub const BN_ADD: &str = "bn-add";
        pub const BN_MUL: &str = "bn-mul";
        pub const BN_PAIR: &str = "bn-pair";
        pub const EC_RECOVER: &str = "ec-recover";
        pub const P256_VERIFY: &str = "p256-verify";
        pub const KZG_EVAL: &str = "kzg-eval";
    }

    /// Full cycle tracker keys (with "precompile-" prefix).
    /// These match the keys in ExecutionReport.cycle_tracker.
    pub mod keys {
        pub const BN_ADD: &str = "precompile-bn-add";
        pub const BN_MUL: &str = "precompile-bn-mul";
        pub const BN_PAIR: &str = "precompile-bn-pair";
        pub const EC_RECOVER: &str = "precompile-ec-recover";
        pub const P256_VERIFY: &str = "precompile-p256-verify";
        pub const KZG_EVAL: &str = "precompile-kzg-eval";
    }
}

/// Get the cycle tracker name for a precompile by its ID.
/// Returns None if the precompile is not accelerated/tracked.
#[cfg(any(test, target_os = "zkvm"))]
#[inline]
fn get_precompile_tracker_name(id: &PrecompileId) -> Option<&'static str> {
    match id {
        PrecompileId::Bn254Add => Some(cycle_tracker::names::BN_ADD),
        PrecompileId::Bn254Mul => Some(cycle_tracker::names::BN_MUL),
        PrecompileId::Bn254Pairing => Some(cycle_tracker::names::BN_PAIR),
        PrecompileId::EcRec => Some(cycle_tracker::names::EC_RECOVER),
        PrecompileId::P256Verify => Some(cycle_tracker::names::P256_VERIFY),
        PrecompileId::KzgPointEvaluation => Some(cycle_tracker::names::KZG_EVAL),
        _ => None,
    }
}

/// The ZKVM-cycle-tracking precompiles.
#[derive(Debug)]
pub struct OpZkvmPrecompiles {
    /// The default [`EthPrecompiles`] provider.
    inner: EthPrecompiles,
    /// The [`OpSpecId`] of the precompiles.
    spec: OpSpecId,
}

impl OpZkvmPrecompiles {
    /// Create a new precompile provider with the given [`OpSpecId`].
    #[inline]
    pub fn new_with_spec(spec: OpSpecId) -> Self {
        let precompiles = OpPrecompiles::new_with_spec(spec).precompiles();

        Self { inner: EthPrecompiles { precompiles, spec: spec.into_eth_spec() }, spec }
    }
}

impl<CTX> PrecompileProvider<CTX> for OpZkvmPrecompiles
where
    CTX: ContextTr<Cfg: Cfg<Spec = OpSpecId>>,
{
    type Output = InterpreterResult;

    #[inline]
    fn set_spec(&mut self, spec: <CTX::Cfg as Cfg>::Spec) -> bool {
        if spec == self.spec {
            return false;
        }
        *self = Self::new_with_spec(spec);
        true
    }

    // NOTE: This `run` mirrors the canonical `EthPrecompiles::run` in
    // revm-handler v15.0.0 / op-revm v15.0.0, with cycle-tracker prints
    // wrapped around `precompile.execute()` for the zkVM target. Keep the
    // body in sync when bumping revm-handler / op-revm — see
    // https://github.com/bluealloy/revm/blob/9bc0c04fda0891e0e8d2e2a6dfd0af81c2af18c4/crates/handler/src/precompile_provider.rs#L99-L160
    #[inline]
    fn run(
        &mut self,
        context: &mut CTX,
        inputs: &CallInputs,
    ) -> Result<Option<Self::Output>, String> {
        // Bail before allocating `result` or materializing input bytes when
        // the call is not to a precompile; this mirrors canonical revm and
        // keeps the non-precompile call path cheap in the zkVM.
        let Some(precompile) = self.inner.precompiles.get(&inputs.bytecode_address) else {
            return Ok(None);
        };

        let mut result = InterpreterResult {
            result: InstructionResult::Return,
            gas: Gas::new(inputs.gas_limit),
            output: Bytes::new(),
        };

        // Track cycles for accelerated precompiles. zkVM acceleration comes
        // from patched crypto crates ([patch.crates-io] in workspace
        // Cargo.toml); the wrapper only adds cycle-tracker prints.
        #[cfg(target_os = "zkvm")]
        let tracker_name = get_precompile_tracker_name(precompile.id());

        use revm::context::LocalContextTr;
        let exec_result = {
            // SP1 program builds pull in `winnow`, which implements
            // `AsRef<{BStr,Bytes}> for [u8]`, making `r.as_ref()` ambiguous
            // in the riscv32 target. `Option::as_deref` is unambiguous
            // because it knows the `Deref::Target = [u8]`, so we keep this
            // shape rather than the literal canonical revm form.
            let shared_buffer;
            let input_bytes = match &inputs.input {
                CallInput::SharedBuffer(range) => {
                    shared_buffer = context.local().shared_memory_buffer_slice(range.clone());
                    shared_buffer.as_deref().unwrap_or(&[])
                }
                CallInput::Bytes(bytes) => bytes.0.iter().as_slice(),
            };

            #[cfg(target_os = "zkvm")]
            if let Some(name) = tracker_name {
                println!("cycle-tracker-report-start: precompile-{}", name);
            }

            let exec_result = precompile.execute(input_bytes, inputs.gas_limit);

            #[cfg(target_os = "zkvm")]
            if let Some(name) = tracker_name {
                println!("cycle-tracker-report-end: precompile-{}", name);
            }

            exec_result
        };

        match exec_result {
            Ok(output) => {
                result.gas.record_refund(output.gas_refunded);
                let underflow = result.gas.record_cost(output.gas_used);
                assert!(underflow, "Gas underflow is not possible");
                result.result = if output.reverted {
                    InstructionResult::Revert
                } else {
                    InstructionResult::Return
                };
                result.output = output.bytes;
            }
            Err(PrecompileError::Fatal(e)) => return Err(e),
            Err(e) => {
                result.result = if e.is_oog() {
                    InstructionResult::PrecompileOOG
                } else {
                    InstructionResult::PrecompileError
                };
                if !e.is_oog() && context.journal().depth() == 1 {
                    context.local_mut().set_precompile_error_context(e.to_string());
                }
            }
        }

        Ok(Some(result))
    }

    #[inline]
    fn warm_addresses(&self) -> Box<impl Iterator<Item = Address>> {
        self.inner.warm_addresses()
    }

    #[inline]
    fn contains(&self, address: &Address) -> bool {
        self.inner.contains(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use alloy_primitives::U256;
    use op_revm::{precompiles::bn254_pair, DefaultOp as _, OpContext};
    use revm::{
        context::LocalContextTr as _,
        database::EmptyDB,
        handler::PrecompileProvider,
        interpreter::{CallInput, CallScheme, CallValue},
        Context,
    };
    use revm_precompile::{bn254, kzg_point_evaluation, secp256k1, secp256r1, PrecompileId};

    type TestContext = OpContext<EmptyDB>;

    const ALL_OP_SPECS: [OpSpecId; 11] = [
        OpSpecId::BEDROCK,
        OpSpecId::REGOLITH,
        OpSpecId::CANYON,
        OpSpecId::ECOTONE,
        OpSpecId::FJORD,
        OpSpecId::GRANITE,
        OpSpecId::HOLOCENE,
        OpSpecId::ISTHMUS,
        OpSpecId::JOVIAN,
        OpSpecId::INTEROP,
        OpSpecId::OSAKA,
    ];

    // Compile-time guard: a new `OpSpecId` variant must be added to
    // `ALL_OP_SPECS` (and to the parity / regression tests below). The
    // exhaustive match here will fail to compile until the new variant is
    // wired in.
    #[allow(dead_code)]
    fn _assert_all_op_specs_covered(spec: OpSpecId) {
        match spec {
            OpSpecId::BEDROCK |
            OpSpecId::REGOLITH |
            OpSpecId::CANYON |
            OpSpecId::ECOTONE |
            OpSpecId::FJORD |
            OpSpecId::GRANITE |
            OpSpecId::HOLOCENE |
            OpSpecId::ISTHMUS |
            OpSpecId::JOVIAN |
            OpSpecId::INTEROP |
            OpSpecId::OSAKA => {}
        }
    }

    /// Creates a [`CallInputs`] with `bytecode_address` set to the given address
    /// and `target_address` set to zero, simulating a DELEGATECALL scenario.
    fn create_call_inputs(address: Address, input: Bytes, gas_limit: u64) -> CallInputs {
        CallInputs {
            input: CallInput::Bytes(input),
            gas_limit,
            bytecode_address: address,
            target_address: Address::ZERO, // Simulates DELEGATECALL context
            caller: Address::ZERO,
            value: CallValue::Transfer(U256::ZERO),
            scheme: CallScheme::Call,
            is_static: false,
            return_memory_offset: 0..0,
            known_bytecode: None,
        }
    }

    fn create_test_context() -> TestContext {
        Context::op().with_db(EmptyDB::new())
    }

    fn run_op_precompile(
        spec: OpSpecId,
        call_inputs: &CallInputs,
    ) -> Result<Option<InterpreterResult>, String> {
        let mut ctx = create_test_context();
        let mut precompiles = OpPrecompiles::new_with_spec(spec);
        precompiles.run(&mut ctx, call_inputs)
    }

    fn run_zkvm_precompile(
        spec: OpSpecId,
        call_inputs: &CallInputs,
    ) -> Result<Option<InterpreterResult>, String> {
        let mut ctx = create_test_context();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(spec);
        precompiles.run(&mut ctx, call_inputs)
    }

    fn run_op_precompile_with_top_level_error_context(
        spec: OpSpecId,
        call_inputs: &CallInputs,
    ) -> (Result<Option<InterpreterResult>, String>, Option<String>) {
        let mut ctx = create_test_context();
        let checkpoint = ctx.journal_mut().checkpoint();
        let mut precompiles = OpPrecompiles::new_with_spec(spec);
        let result = precompiles.run(&mut ctx, call_inputs);
        let error_context = ctx.local_mut().take_precompile_error_context();
        ctx.journal_mut().checkpoint_revert(checkpoint);
        (result, error_context)
    }

    fn run_zkvm_precompile_with_top_level_error_context(
        spec: OpSpecId,
        call_inputs: &CallInputs,
    ) -> (Result<Option<InterpreterResult>, String>, Option<String>) {
        let mut ctx = create_test_context();
        let checkpoint = ctx.journal_mut().checkpoint();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(spec);
        let result = precompiles.run(&mut ctx, call_inputs);
        let error_context = ctx.local_mut().take_precompile_error_context();
        ctx.journal_mut().checkpoint_revert(checkpoint);
        (result, error_context)
    }

    fn assert_run_matches_op_revm(spec: OpSpecId, address: Address, input: Bytes, gas_limit: u64) {
        let call_inputs = create_call_inputs(address, input, gas_limit);
        let op_result = run_op_precompile(spec, &call_inputs);
        let zkvm_result = run_zkvm_precompile(spec, &call_inputs);

        assert_eq!(
            zkvm_result, op_result,
            "ZKVM precompile execution must match canonical OP execution for {spec:?} at {address:?}",
        );
    }

    // ===== Precompile Provider Functional Tests =====

    /// Test that precompiles are looked up by `bytecode_address`, not `target_address`.
    /// This is critical for DELEGATECALL scenarios where these addresses differ.
    #[test]
    fn test_precompile_lookup_uses_bytecode_address() {
        let mut ctx = create_test_context();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(OpSpecId::BEDROCK);

        // SHA256 precompile at address 0x02
        let sha256_addr = revm::precompile::u64_to_address(2);

        // Create inputs where bytecode_address != target_address (DELEGATECALL scenario)
        let call_inputs = create_call_inputs(sha256_addr, Bytes::from_static(b"test"), u64::MAX);

        // Verify target_address is different from bytecode_address
        assert_ne!(call_inputs.bytecode_address, call_inputs.target_address);

        // Should find the precompile via bytecode_address
        let result = precompiles.run(&mut ctx, &call_inputs).unwrap();
        assert!(result.is_some(), "Precompile should be found via bytecode_address");

        let interpreter_result = result.unwrap();
        assert_eq!(interpreter_result.result, InstructionResult::Return);
        assert!(!interpreter_result.output.is_empty());
    }

    /// Test that a non-existent precompile returns None.
    #[test]
    fn test_run_nonexistent_precompile() {
        let mut ctx = create_test_context();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(OpSpecId::BEDROCK);

        let fake_addr = Address::from_slice(&[0xFFu8; 20]);
        let call_inputs = create_call_inputs(fake_addr, Bytes::new(), u64::MAX);

        let result = precompiles.run(&mut ctx, &call_inputs).unwrap();
        assert!(result.is_none());
    }

    /// Test out-of-gas handling for precompiles.
    #[test]
    fn test_run_out_of_gas() {
        let mut ctx = create_test_context();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(OpSpecId::BEDROCK);

        let sha256_addr = revm::precompile::u64_to_address(2);
        let call_inputs = create_call_inputs(sha256_addr, Bytes::from_static(b"test"), 0);

        let result = precompiles.run(&mut ctx, &call_inputs).unwrap();
        assert!(result.is_some());

        let interpreter_result = result.unwrap();
        assert_eq!(interpreter_result.result, InstructionResult::PrecompileOOG);
    }

    /// Test SharedBuffer input handling.
    #[test]
    fn test_run_with_shared_buffer_empty() {
        let mut ctx = create_test_context();
        let mut precompiles = OpZkvmPrecompiles::new_with_spec(OpSpecId::BEDROCK);

        let sha256_addr = revm::precompile::u64_to_address(2);
        let call_inputs = CallInputs {
            input: CallInput::SharedBuffer(0..0),
            gas_limit: u64::MAX,
            bytecode_address: sha256_addr,
            target_address: Address::ZERO,
            caller: Address::ZERO,
            value: CallValue::Transfer(U256::ZERO),
            scheme: CallScheme::Call,
            is_static: false,
            return_memory_offset: 0..0,
            known_bytecode: None,
        };

        let result = precompiles.run(&mut ctx, &call_inputs).unwrap();
        assert!(result.is_some());
    }

    // ===== Cycle Tracker Name Tests =====

    #[test]
    fn test_precompile_tracker_name_bn_add() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::Bn254Add),
            Some(cycle_tracker::names::BN_ADD)
        );
    }

    #[test]
    fn test_precompile_tracker_name_bn_mul() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::Bn254Mul),
            Some(cycle_tracker::names::BN_MUL)
        );
    }

    #[test]
    fn test_precompile_tracker_name_bn_pair() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::Bn254Pairing),
            Some(cycle_tracker::names::BN_PAIR)
        );
    }

    #[test]
    fn test_precompile_tracker_name_ecrecover() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::EcRec),
            Some(cycle_tracker::names::EC_RECOVER)
        );
    }

    #[test]
    fn test_precompile_tracker_name_p256verify() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::P256Verify),
            Some(cycle_tracker::names::P256_VERIFY)
        );
    }

    #[test]
    fn test_precompile_tracker_name_kzg_eval() {
        assert_eq!(
            get_precompile_tracker_name(&PrecompileId::KzgPointEvaluation),
            Some(cycle_tracker::names::KZG_EVAL)
        );
    }

    #[test]
    fn test_unknown_precompile_returns_none() {
        // SHA256 is a precompile but not accelerated/tracked
        assert_eq!(get_precompile_tracker_name(&PrecompileId::Sha256), None);
        assert_eq!(get_precompile_tracker_name(&PrecompileId::Identity), None);
    }

    // ===== Consistency Tests =====

    #[test]
    fn test_tracked_precompile_ids_have_tracker_names() {
        let tracked_ids = [
            PrecompileId::Bn254Add,
            PrecompileId::Bn254Mul,
            PrecompileId::Bn254Pairing,
            PrecompileId::EcRec,
            PrecompileId::P256Verify,
            PrecompileId::KzgPointEvaluation,
        ];

        for id in &tracked_ids {
            let tracker = get_precompile_tracker_name(id);
            assert!(tracker.is_some(), "Precompile {id:?} is missing a cycle tracker name",);
        }
    }

    #[test]
    fn test_canonical_tracked_precompile_addresses_keep_tracked_ids() {
        let tracked_precompiles = [
            (bn254::add::ADDRESS, PrecompileId::Bn254Add),
            (bn254::mul::ADDRESS, PrecompileId::Bn254Mul),
            (bn254::pair::ADDRESS, PrecompileId::Bn254Pairing),
            (*secp256k1::ECRECOVER.address(), PrecompileId::EcRec),
            (*secp256r1::P256VERIFY.address(), PrecompileId::P256Verify),
            (kzg_point_evaluation::ADDRESS, PrecompileId::KzgPointEvaluation),
        ];

        for spec in ALL_OP_SPECS {
            let op_precompiles = OpPrecompiles::new_with_spec(spec);

            for (address, expected_id) in &tracked_precompiles {
                let Some(precompile) = op_precompiles.precompiles().get(address) else {
                    continue;
                };

                assert_eq!(
                    precompile.id(),
                    expected_id,
                    "Canonical OP precompile at {address:?} for {spec:?} must keep its tracked ID",
                );
                assert!(
                    get_precompile_tracker_name(precompile.id()).is_some(),
                    "Canonical OP precompile {:?} at {address:?} for {spec:?} is missing a cycle tracker name",
                    precompile.id(),
                );
            }
        }
    }

    #[test]
    fn test_zkvm_precompiles_match_op_revm_precompiles() {
        for spec in ALL_OP_SPECS {
            let op_precompiles = OpPrecompiles::new_with_spec(spec);
            let zkvm_precompiles = OpZkvmPrecompiles::new_with_spec(spec);

            let op_addresses: Vec<_> =
                <OpPrecompiles as PrecompileProvider<TestContext>>::warm_addresses(&op_precompiles)
                    .collect();
            let zkvm_addresses: Vec<_> =
                <OpZkvmPrecompiles as PrecompileProvider<TestContext>>::warm_addresses(
                    &zkvm_precompiles,
                )
                .collect();

            assert_eq!(
                zkvm_addresses.len(),
                op_addresses.len(),
                "ZKVM and canonical OP precompile counts must match for {spec:?}",
            );

            for address in &op_addresses {
                assert!(
                    <OpZkvmPrecompiles as PrecompileProvider<TestContext>>::contains(
                        &zkvm_precompiles,
                        address,
                    ),
                    "ZKVM precompiles missing canonical OP precompile {address:?} for {spec:?}",
                );
            }

            for address in &zkvm_addresses {
                assert!(
                    <OpPrecompiles as PrecompileProvider<TestContext>>::contains(
                        &op_precompiles,
                        address,
                    ),
                    "ZKVM precompiles contain non-canonical OP precompile {address:?} for {spec:?}",
                );
            }
        }
    }

    #[test]
    fn test_zkvm_precompile_execution_matches_op_revm() {
        for spec in ALL_OP_SPECS {
            let op_precompiles = OpPrecompiles::new_with_spec(spec);
            let op_addresses: Vec<_> =
                <OpPrecompiles as PrecompileProvider<TestContext>>::warm_addresses(&op_precompiles)
                    .collect();

            for address in op_addresses {
                assert_run_matches_op_revm(spec, address, Bytes::new(), u64::MAX);
                assert_run_matches_op_revm(spec, address, Bytes::new(), 0);
            }
        }
    }

    #[test]
    fn test_top_level_precompile_error_context_matches_op_revm() {
        let call_inputs = create_call_inputs(kzg_point_evaluation::ADDRESS, Bytes::new(), u64::MAX);

        let (op_result, op_error_context) =
            run_op_precompile_with_top_level_error_context(OpSpecId::ECOTONE, &call_inputs);
        let (zkvm_result, zkvm_error_context) =
            run_zkvm_precompile_with_top_level_error_context(OpSpecId::ECOTONE, &call_inputs);

        assert_eq!(
            zkvm_result, op_result,
            "ZKVM KZG error result must match canonical OP precompile behavior",
        );
        assert_eq!(
            zkvm_error_context, op_error_context,
            "ZKVM KZG top-level error context must match canonical OP precompile behavior",
        );
        assert!(
            zkvm_error_context.is_some(),
            "Invalid KZG input should set top-level precompile error context",
        );
    }

    /// Smallest `PAIR_ELEMENT_LEN`-aligned input length strictly greater
    /// than `max`, so `bn254::run_pair`'s modulo check does not fire first
    /// and the wrapper specifically exercises the per-fork cap check.
    const fn oversized_aligned_pair_input_len(max: usize) -> usize {
        (max / bn254::PAIR_ELEMENT_LEN + 1) * bn254::PAIR_ELEMENT_LEN
    }

    #[test]
    fn test_jovian_family_uses_canonical_bn254_pairing_limits() {
        for spec in [OpSpecId::JOVIAN, OpSpecId::INTEROP, OpSpecId::OSAKA] {
            let oversized_pairing_input =
                vec![0; oversized_aligned_pair_input_len(bn254_pair::JOVIAN_MAX_INPUT_SIZE)];
            let call_inputs =
                create_call_inputs(bn254::pair::ADDRESS, oversized_pairing_input.into(), u64::MAX);

            let op_result = run_op_precompile(spec, &call_inputs);
            let zkvm_result = run_zkvm_precompile(spec, &call_inputs);

            assert_eq!(
                zkvm_result, op_result,
                "{spec:?} BN254 pairing behavior must match canonical OP precompile behavior",
            );

            let result = zkvm_result.unwrap().expect("BN254 pairing precompile must exist");

            assert_eq!(
                result.result,
                InstructionResult::PrecompileError,
                "{spec:?} must use canonical Jovian BN254 pairing limits",
            );
        }
    }

    /// Direct regression test for the headline bug: under the old
    /// ISTANBUL overlay, BN254 pairing accepted arbitrary input length on
    /// Granite/Holocene/Isthmus. Canonical Granite caps input at
    /// `GRANITE_MAX_INPUT_SIZE`; the wrapper must enforce the same cap.
    /// Input length is aligned to `PAIR_ELEMENT_LEN` so the test
    /// distinguishes the cap from `bn254::run_pair`'s modulo check.
    #[test]
    fn test_granite_family_uses_canonical_bn254_pairing_limits() {
        for spec in [OpSpecId::GRANITE, OpSpecId::HOLOCENE, OpSpecId::ISTHMUS] {
            let oversized_pairing_input =
                vec![0; oversized_aligned_pair_input_len(bn254_pair::GRANITE_MAX_INPUT_SIZE)];
            let call_inputs =
                create_call_inputs(bn254::pair::ADDRESS, oversized_pairing_input.into(), u64::MAX);

            let op_result = run_op_precompile(spec, &call_inputs);
            let zkvm_result = run_zkvm_precompile(spec, &call_inputs);

            assert_eq!(
                zkvm_result, op_result,
                "{spec:?} BN254 pairing behavior must match canonical OP precompile behavior",
            );

            let result = zkvm_result.unwrap().expect("BN254 pairing precompile must exist");

            assert_eq!(
                result.result,
                InstructionResult::PrecompileError,
                "{spec:?} must use canonical Granite BN254 pairing limits",
            );
        }
    }

    #[test]
    fn test_tracker_keys_match_expected_format() {
        let expected_keys = [
            cycle_tracker::keys::BN_ADD,
            cycle_tracker::keys::BN_MUL,
            cycle_tracker::keys::BN_PAIR,
            cycle_tracker::keys::EC_RECOVER,
            cycle_tracker::keys::P256_VERIFY,
            cycle_tracker::keys::KZG_EVAL,
        ];

        for key in &expected_keys {
            assert!(
                key.starts_with(cycle_tracker::PREFIX),
                "Key '{}' should start with '{}'",
                key,
                cycle_tracker::PREFIX
            );
            assert!(!key.contains(' '), "Key '{}' contains spaces", key);
            assert!(
                !key[cycle_tracker::PREFIX.len()..].contains('_'),
                "Key '{}' contains underscores (should use dashes)",
                key
            );
        }
    }

    #[test]
    fn test_names_and_keys_are_consistent() {
        assert_eq!(
            cycle_tracker::keys::BN_ADD,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::BN_ADD)
        );
        assert_eq!(
            cycle_tracker::keys::BN_MUL,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::BN_MUL)
        );
        assert_eq!(
            cycle_tracker::keys::BN_PAIR,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::BN_PAIR)
        );
        assert_eq!(
            cycle_tracker::keys::EC_RECOVER,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::EC_RECOVER)
        );
        assert_eq!(
            cycle_tracker::keys::P256_VERIFY,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::P256_VERIFY)
        );
        assert_eq!(
            cycle_tracker::keys::KZG_EVAL,
            format!("{}{}", cycle_tracker::PREFIX, cycle_tracker::names::KZG_EVAL)
        );
    }
}
