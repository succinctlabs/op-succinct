//! [`EvmFactory`] implementation for the EVM in the ZKVM environment.

use super::OpZkvmPrecompiles;
use alloy_evm::{Database, EvmEnv, EvmFactory};
use alloy_op_evm::{
    post_exec::{PostExecEvmFactoryHooks, PostExecExecutedTx, PostExecTxContext, WarmingState},
    OpEvm, OpEvmContext, OpTx, OpTxError,
};
use op_revm::{L1BlockInfo, OpBuilder, OpHaltReason, OpSpecId, OpTransaction};
use revm::{
    context::{result::EVMError, BlockEnv, CfgEnv},
    inspector::NoOpInspector,
    Context, Inspector, MainContext,
};

/// Factory producing [`OpEvm`]s with FPVM-accelerated precompile overrides enabled.
#[derive(Debug, Clone)]
pub struct ZkvmOpEvmFactory {}

impl ZkvmOpEvmFactory {
    /// Creates a new [`ZkvmOpEvmFactory`].
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ZkvmOpEvmFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl EvmFactory for ZkvmOpEvmFactory {
    type Evm<DB: Database, I: Inspector<OpEvmContext<DB>>> = OpEvm<DB, I, OpZkvmPrecompiles, OpTx>;
    type Context<DB: Database> = OpEvmContext<DB>;
    type Tx = OpTx;
    type Error<DBError: core::error::Error + Send + Sync + 'static> = EVMError<DBError, OpTxError>;
    type HaltReason = OpHaltReason;
    type Spec = OpSpecId;
    type BlockEnv = BlockEnv;
    type Precompiles = OpZkvmPrecompiles;

    fn create_evm<DB: Database>(
        &self,
        db: DB,
        input: EvmEnv<OpSpecId>,
    ) -> Self::Evm<DB, NoOpInspector> {
        let spec_id = input.cfg_env.spec;
        OpEvm::new(
            Context::mainnet()
                .with_tx(OpTx(OpTransaction::builder().build_fill()))
                .with_cfg(CfgEnv::new_with_spec(OpSpecId::BEDROCK))
                .with_chain(L1BlockInfo::default())
                .with_db(db)
                .with_block(input.block_env)
                .with_cfg(input.cfg_env)
                .build_op_with_inspector(NoOpInspector {})
                .with_precompiles(OpZkvmPrecompiles::new_with_spec(spec_id)),
            false,
        )
    }

    fn create_evm_with_inspector<DB: Database, I: Inspector<Self::Context<DB>>>(
        &self,
        db: DB,
        input: EvmEnv<OpSpecId>,
        inspector: I,
    ) -> Self::Evm<DB, I> {
        let spec_id = input.cfg_env.spec;
        OpEvm::new(
            Context::mainnet()
                .with_tx(OpTx(OpTransaction::builder().build_fill()))
                .with_cfg(CfgEnv::new_with_spec(OpSpecId::BEDROCK))
                .with_chain(L1BlockInfo::default())
                .with_db(db)
                .with_block(input.block_env)
                .with_cfg(input.cfg_env)
                .build_op_with_inspector(inspector)
                .with_precompiles(OpZkvmPrecompiles::new_with_spec(spec_id)),
            true,
        )
    }
}

// Post-exec hooks for `ZkvmOpEvmFactory`, mirroring the canonical `OpEvmFactory`
// impl in alloy-op-evm. `KonaExecutor`/`OpBlockExecutorFactory` require the EVM
// factory to be wrapped in `PostExecEvmFactoryAdapter<F>`, which in turn requires
// `F: PostExecEvmFactoryHooks`. The hooks delegate to the inherent post-exec
// methods on the produced [`OpEvm`].
impl PostExecEvmFactoryHooks for ZkvmOpEvmFactory {
    fn begin_post_exec_tx<DB, I>(evm: &mut Self::Evm<DB, I>, ctx: PostExecTxContext)
    where
        DB: Database,
        I: Inspector<Self::Context<DB>>,
    {
        evm.begin_post_exec_tx(ctx);
    }

    fn take_last_post_exec_tx_result<DB, I>(evm: &mut Self::Evm<DB, I>) -> PostExecExecutedTx
    where
        DB: Database,
        I: Inspector<Self::Context<DB>>,
    {
        evm.take_last_post_exec_tx_result()
    }

    fn warming_state<DB, I>(evm: &Self::Evm<DB, I>) -> WarmingState
    where
        DB: Database,
        I: Inspector<Self::Context<DB>>,
    {
        evm.warming_state()
    }

    fn seed_warming_state<DB, I>(evm: &mut Self::Evm<DB, I>, state: WarmingState)
    where
        DB: Database,
        I: Inspector<Self::Context<DB>>,
    {
        evm.seed_warming_state(state);
    }
}
