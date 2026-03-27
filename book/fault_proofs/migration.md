# Migrating from Optimistic to ZK Fault Proofs

This guide walks through migrating an existing OP Stack chain from optimistic fault proofs (e.g., Cannon) to ZK fault proofs using OP Succinct Lite. The migration is a **hot swap** — the L2 chain continues producing blocks normally with zero downtime. Only the L1 verification mechanism changes.

```admonish warning
This guide assumes your chain already uses `DisputeGameFactory`, `AnchorStateRegistry`, and `OptimismPortal2`. If your chain uses the legacy `L2OutputOracle`, you must migrate to the fault proof system first (separate migration).
```

## How It Works

The OP Stack's `DisputeGameFactory` supports multiple game types simultaneously. Migration works by registering a new ZK game type (type `42` for OP Succinct Lite) alongside the existing optimistic game type, then switching the chain's "respected" game type.

Withdrawals remain safe throughout the migration. Each game records an immutable `wasRespectedGameTypeWhenCreated` flag at creation time. Games created while their type was respected retain that flag forever, so in-flight withdrawals proven against old games can still finalize after the switch. No user action is required.

## Prerequisites

### Existing Infrastructure

Your chain must already be running a modern OP Stack with:

- `DisputeGameFactory` (proxy)
- `AnchorStateRegistry` (proxy)
- `OptimismPortal2`
- `op-proposer` and `op-challenger` (OP native)

### Required Access

The migration requires two distinct privileged roles:

| Role | Used For | How to Identify |
|------|----------|-----------------|
| **Factory Owner** | `setImplementation`, `setInitBond` on DisputeGameFactory | `DisputeGameFactory.owner()` |
| **Guardian** | `setRespectedGameType` on AnchorStateRegistry | `SystemConfig.guardian()` |

These are typically different keys. Confirm you have access to both before proceeding.

### Infrastructure & Tooling

- L1 Archive Node RPC
- L2 Execution Node RPC (`op-geth`)
- L2 Rollup Node RPC (`op-node`), preferably with [SafeDB enabled](./best_practices.md#safe-db-configuration)
- SP1 proving cluster or [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/quickstart) access
- L1 Beacon RPC (if using SafeDB or fast finality mode)
- [Foundry](https://book.getfoundry.sh/getting-started/installation), [Rust](https://www.rust-lang.org/tools/install) (latest stable), [just](https://github.com/casey/just)

## Rollback Plan

Before starting, understand how to revert if needed:

1. Stop the OP Succinct proposer and challenger.
2. Revert the respected game type (Guardian):
   ```bash
   cast send $ANCHOR_STATE_REGISTRY_ADDRESS "setRespectedGameType(uint32)" <OLD_GAME_TYPE> \
     --rpc-url $L1_RPC --private-key $GUARDIAN_PRIVATE_KEY
   ```
3. Restart the old `op-proposer` and `op-challenger`.

ZK games created during the migration window retain `wasRespectedGameTypeWhenCreated = true` and remain valid, but no new ZK games will be created after rollback.

## Migration Steps

### Phase 1: Deploy the ZK Game Implementation

This phase registers the OP Succinct Lite game type in your existing `DisputeGameFactory`. No on-chain behavior changes yet — the old optimistic system continues operating normally.

#### 1.1 Clone and build

```bash
git clone https://github.com/succinctlabs/op-succinct.git
cd op-succinct/contracts
forge build
cd ..
```

#### 1.2 Configure environment

Create a `.env` file in the project root. This uses the same environment variables as a fresh deployment — see [Contract Configuration](./deploy.md#contract-configuration) for the full reference.

Key differences for migration (vs. greenfield deploy):

| Variable | Migration Value |
|----------|----------------|
| `FACTORY_ADDRESS` | Your **existing** `DisputeGameFactory` proxy address |
| `OPTIMISM_PORTAL2_ADDRESS` | Your **existing** `OptimismPortal2` address |
| `ANCHOR_STATE_REGISTRY` | Your **existing** `AnchorStateRegistry` proxy address |
| `PRIVATE_KEY` | Must be the **Factory Owner** key |
| `GAME_TYPE` | `42` |

#### 1.3 Access control configuration

Configure proposer and challenger access control as described in the [deployment guide](./deploy.md#required-environment-variables). Permissioned mode (`PERMISSIONLESS_MODE=false`) is recommended for initial migration. See [Fallback Timeout Mechanism](./deploy.md#fallback-timeout-mechanism) for details on permissionless fallback behavior.

#### 1.4 Deploy and register

Use the [upgrade script](./upgrade.md#upgrade-command) to deploy `OPSuccinctFaultDisputeGame` and register it in your existing factory. The script calls `DisputeGameFactory.setImplementation(42, newImpl)`.

```bash
# Dry run first to verify
DRY_RUN=true just upgrade-fault-dispute-game

# Execute
DRY_RUN=false just upgrade-fault-dispute-game
```

You must also set the initial bond for the new game type. If the upgrade script does not call `setInitBond`, run it manually:

```bash
cast send $FACTORY_ADDRESS "setInitBond(uint32,uint256)" 42 $INITIAL_BOND_WEI \
  --rpc-url $L1_RPC --private-key $PRIVATE_KEY
```

For bond sizing guidance, see the [deployment guide](./deploy.md#optional-environment-variables).

#### 1.5 Verify registration

```bash
# Should return the new implementation address (non-zero)
cast call $FACTORY_ADDRESS "gameImpls(uint32)" 42 --rpc-url $L1_RPC

# Should return the bond amount
cast call $FACTORY_ADDRESS "initBonds(uint32)" 42 --rpc-url $L1_RPC
```

See the [upgrade verification](./upgrade.md#verification) guide for more details.

### Phase 2: Activate the ZK Game Type

This is the cutover step. After this, the chain recognizes ZK games as the canonical proof type.

#### 2.1 Set respected game type

Call `setRespectedGameType` on the `AnchorStateRegistry`. This requires the **Guardian** key:

```bash
cast send $ANCHOR_STATE_REGISTRY_ADDRESS "setRespectedGameType(uint32)" 42 \
  --rpc-url $L1_RPC --private-key $GUARDIAN_PRIVATE_KEY
```

Verify:

```bash
cast call $ANCHOR_STATE_REGISTRY_ADDRESS "respectedGameType()" --rpc-url $L1_RPC
# Should return 42 (0x2a)
```

```admonish important
Only start the OP Succinct proposer **after** `setRespectedGameType` is confirmed. Games created before this call will have `wasRespectedGameTypeWhenCreated = false` and cannot be used for withdrawal proofs.
```

#### 2.2 Start OP Succinct proposer

Create `.env.proposer` in the `fault-proof` directory. See [Proposer Configuration](./proposer.md#configuration) for the full list of required and optional variables.

Migration-specific notes:
- Use the `ANCHOR_STATE_REGISTRY_ADDRESS` and `FACTORY_ADDRESS` from your existing deployment.
- Set `GAME_TYPE=42`.
- Consider starting with `MOCK_MODE=true` for initial validation, then switching to real proofs.

```bash
cd fault-proof
cargo run --bin proposer
```

Watch for this log confirming ZK games are being created:

```
INFO Game created successfully ...
```

#### 2.3 Start OP Succinct challenger

Create `.env.challenger` in the `fault-proof` directory. The challenger requires `L1_RPC`, `L2_RPC`, `ANCHOR_STATE_REGISTRY_ADDRESS`, `FACTORY_ADDRESS`, `GAME_TYPE`, and a signing key. See [Challenger Configuration](./challenger.md#configuration) for the full reference.

```bash
cargo run --bin challenger
```

#### 2.4 Verify migration

| What to Check | How |
|---------------|-----|
| New impl registered | `cast call $FACTORY_ADDRESS "gameImpls(uint32)" 42` returns non-zero |
| Respected type updated | `cast call $ANCHOR_STATE_REGISTRY_ADDRESS "respectedGameType()"` returns `42` |
| ZK proposer creating games | Log: `Game created successfully` |
| Old proposer stopped creating | No new games with old type appearing |

### Phase 3: Wind Down the Old System

#### 3.1 Stop the old op-proposer

Once the OP Succinct proposer is creating games successfully, stop the OP native `op-proposer`. Any new optimistic games it creates after `setRespectedGameType` will have `wasRespectedGameTypeWhenCreated = false` and won't be usable for withdrawals.

```bash
# Stop the OP native proposer (method depends on your deployment)
# e.g., systemctl stop op-proposer, docker stop op-proposer, etc.
```

#### 3.2 Wait for old games to resolve

Old optimistic games created **before** the game type switch still have `wasRespectedGameTypeWhenCreated = true` and must be allowed to complete their lifecycle. This ensures any in-flight withdrawals proven against those games can finalize normally.

The maximum wait time depends on your **existing optimistic game type's parameters** (not the new ZK game's config). Check your current game implementation's timing values — typically this is on the order of 1–2 weeks.

#### 3.3 Stop the old op-challenger

Once all old optimistic games have resolved, stop the OP native `op-challenger`.

#### 3.4 (Optional) Retire old games

To blanket-invalidate all games created before a certain point, the Guardian can set a retirement timestamp:

```bash
cast send $ANCHOR_STATE_REGISTRY_ADDRESS "updateRetirementTimestamp()" \
  --rpc-url $L1_RPC --private-key $GUARDIAN_PRIVATE_KEY
```

This marks all games created at or before `block.timestamp` as retired (`isGameRetired() = true`), preventing them from being used for new withdrawal proofs. Only do this after confirming all legitimate withdrawals from old games have finalized.

#### Post-migration monitoring

| What to Check | How |
|---------------|-----|
| Games resolving normally | Log: `Resolved game` |
| Bonds being claimed | Log: `Claimed bond` |
| Challenger active | Log: `Game challenged successfully` (for invalid games) |
| Old games winding down | Decreasing count of unresolved old-type games |

For metrics endpoints, see [Proposer Configuration](./proposer.md#optional-environment-variables) (`PROPOSER_METRICS_PORT`) and [Challenger Configuration](./challenger.md#optional-environment-variables) (`CHALLENGER_METRICS_PORT`).

## Withdrawal Safety Details

For operators who want a deeper understanding of withdrawal behavior during migration:

### How withdrawals work across the game type switch

- **Old games** (created before switch): `wasRespectedGameTypeWhenCreated = true` (immutable). Withdrawals can be proven and finalized against these games normally.
- **New ZK games** (type 42): `wasRespectedGameTypeWhenCreated = true` since type 42 is now respected. Work normally.

### Withdrawal timeline

```
T0  Game created
T1  User calls proveWithdrawalTransaction() (can happen any time after T0)
T2  Game resolved (DEFENDER_WINS)
T3  T2 + DISPUTE_GAME_FINALITY_DELAY → game finalized
T4  max(T3, T1 + PROOF_MATURITY_DELAY) → withdrawal finalizable
T5  User calls finalizeWithdrawalTransaction()
```

Note: withdrawal proving (`proveWithdrawalTransaction`) does not require the game to be finalized — it can be called at any point after game creation. The finality and maturity checks only apply at finalization time.

### Emergency invalidation

If old games need to be invalidated immediately (e.g., security incident), the Guardian can:
- Call `updateRetirementTimestamp()` to retire all pre-migration games
- Call `blacklistDisputeGame(address)` to target specific games

Both may block pending withdrawals — use with caution.
