# Migrating from Optimistic to ZK Proofs

This guide walks through migrating an existing OP Stack chain from optimistic fault proofs (e.g., Cannon) to ZK proofs using OP Succinct. The migration is a **hot swap** — the L2 chain continues producing blocks normally with zero downtime. Only the L1 verification mechanism changes.

```admonish warning
This guide assumes your chain already uses `DisputeGameFactory`, `AnchorStateRegistry`, and `OptimismPortal2`. If your chain uses the legacy `L2OutputOracle`, you must migrate to the fault proof system first (separate migration).
```

## How It Works

The OP Stack's `DisputeGameFactory` supports multiple game types simultaneously. Migration works by registering a new ZK game type alongside the existing optimistic game type, then switching the chain's "respected" game type.

Withdrawals remain safe throughout the migration. Each game records an immutable `wasRespectedGameTypeWhenCreated` flag at creation time. Games created while their type was respected retain that flag forever, so in-flight withdrawals proven against old games can still finalize after the switch. No user action is required.

## Choose Your Target Mode

OP Succinct offers two ZK proving modes. Choose one before proceeding:

| | OP Succinct (Validity) | OP Succinct Lite (Fault Proofs) |
|---|---|---|
| **Proof type** | Full validity proofs — every block is proven | ZK fault dispute games — single-round interactive |
| **Game type** | `6` (`OP_SUCCINCT`) | `42` (`OP_SUCCINCT_FAULT_DISPUTE_GAME`) |
| **Core contracts** | `OPSuccinctL2OutputOracle` + `OPSuccinctDisputeGame` | `OPSuccinctFaultDisputeGame` |
| **Challenger needed** | No | Yes |
| **Finalization** | After proof submitted + finality delay | After challenge window + finality delay |

The rest of this guide uses **`<GAME_TYPE>`** to refer to either `6` or `42` depending on your choice.

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
- L2 Rollup Node RPC (`op-node`), preferably with [SafeDB enabled](../fault_proofs/best_practices.md#safedb-configuration)
- SP1 proving cluster or [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/quickstart) access
- L1 Beacon RPC (if using SafeDB or fast finality mode)
- [Foundry](https://book.getfoundry.sh/getting-started/installation), [Rust](https://www.rust-lang.org/tools/install) (latest stable), [just](https://github.com/casey/just)

## Rollback Plan

Before starting, understand how to revert if needed:

1. Stop the OP Succinct proposer (and challenger, if fault proof mode).
2. Revert the respected game type (Guardian):
   ```bash
   cast send $ANCHOR_STATE_REGISTRY_ADDRESS "setRespectedGameType(uint32)" <OLD_GAME_TYPE> \
     --rpc-url $L1_RPC --private-key $GUARDIAN_PRIVATE_KEY
   ```
3. Restart the old `op-proposer` and `op-challenger`.

ZK games created during the migration window remain valid but no new ones will be created after rollback. The registered ZK implementation stays in the factory (there is no `removeImplementation`) but becomes inert once the respected type is switched back.

## Migration Steps

### Phase 1: Deploy the ZK Game Implementation

This phase registers the new ZK game type in your existing `DisputeGameFactory`. No on-chain behavior changes yet — the old optimistic system continues operating normally.

#### 1.1 Clone and build

```bash
git clone https://github.com/succinctlabs/op-succinct.git
cd op-succinct/contracts
forge build
cd ..
```

---

#### If migrating to OP Succinct (Validity Mode, game type 6)

##### 1.2 Configure environment

Create a `.env` file in the project root. See [Environment Variables](../validity/contracts/environment.md) for the full reference.

Key variables for migration:

| Variable | Migration Value |
|----------|----------------|
| `L1_RPC`, `L2_RPC`, `L2_NODE_RPC` | Your node endpoints |
| `PRIVATE_KEY` | Must be the **Factory Owner** key |

##### 1.3 Deploy OPSuccinctL2OutputOracle

Deploy the oracle contract following the [validity deployment guide](../validity/contracts/deploy.md):

```bash
just deploy-oracle
```

Save the proxy address from the output.

##### 1.4 Deploy OPSuccinctDisputeGame and register in factory

Since your chain already has a `DisputeGameFactory`, you need to deploy the `OPSuccinctDisputeGame` wrapper and register it manually — do **not** use `just deploy-dispute-game-factory` as that creates a new factory. See [OptimismPortal2 Support — Existing DisputeGameFactory](../validity/contracts/optimism-portal-2.md#existing-disputegamefactory) for details.

1. Deploy the `OPSuccinctDisputeGame` contract (using `forge create` or a custom script with `L2OO_ADDRESS` set to the oracle proxy from step 1.3).

2. Register the game implementation in your existing factory:
   ```bash
   cast send $FACTORY_ADDRESS "setImplementation(uint32,address)" 6 $DISPUTE_GAME_ADDRESS \
     --rpc-url $L1_RPC --private-key $PRIVATE_KEY
   ```

3. Link the factory to the oracle:
   ```bash
   cast send $L2OO_ADDRESS "setDisputeGameFactory(address)" $FACTORY_ADDRESS \
     --rpc-url $L1_RPC --private-key $PRIVATE_KEY
   ```

4. Set the initial bond:
   ```bash
   cast send $FACTORY_ADDRESS "setInitBond(uint32,uint256)" 6 $INITIAL_BOND_WEI \
     --rpc-url $L1_RPC --private-key $PRIVATE_KEY
   ```

##### 1.5 Verify registration

```bash
# Should return the OPSuccinctDisputeGame address (non-zero)
cast call $FACTORY_ADDRESS "gameImpls(uint32)" 6 --rpc-url $L1_RPC

# Should return the bond amount
cast call $FACTORY_ADDRESS "initBonds(uint32)" 6 --rpc-url $L1_RPC
```

---

#### If migrating to OP Succinct Lite (Fault Proof Mode, game type 42)

##### 1.2 Configure environment

Create a `.env` file in the project root. This uses the same environment variables as a fresh deployment — see [Contract Configuration](../fault_proofs/deploy.md#contract-configuration) for the full reference.

Key differences for migration (vs. greenfield deploy):

| Variable | Migration Value |
|----------|----------------|
| `FACTORY_ADDRESS` | Your **existing** `DisputeGameFactory` proxy address |
| `OPTIMISM_PORTAL2_ADDRESS` | Your **existing** `OptimismPortal2` address |
| `ANCHOR_STATE_REGISTRY` | Your **existing** `AnchorStateRegistry` proxy address |
| `PRIVATE_KEY` | Must be the **Factory Owner** key |
| `GAME_TYPE` | `42` |

##### 1.3 Access control configuration

Configure proposer and challenger access control as described in the [deployment guide](../fault_proofs/deploy.md#required-environment-variables). Permissioned mode (`PERMISSIONLESS_MODE=false`) is recommended for initial migration. See [Fallback Timeout Mechanism](../fault_proofs/deploy.md#fallback-timeout-mechanism) for details on permissionless fallback behavior.

##### 1.4 Deploy and register

Use the [upgrade script](../fault_proofs/upgrade.md#upgrade-command) to deploy `OPSuccinctFaultDisputeGame` and register it in your existing factory. The script calls `DisputeGameFactory.setImplementation(42, newImpl)`.

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

For bond sizing guidance, see the [deployment guide](../fault_proofs/deploy.md#optional-environment-variables).

##### 1.5 Verify registration

```bash
# Should return the new implementation address (non-zero)
cast call $FACTORY_ADDRESS "gameImpls(uint32)" 42 --rpc-url $L1_RPC

# Should return the bond amount
cast call $FACTORY_ADDRESS "initBonds(uint32)" 42 --rpc-url $L1_RPC
```

See the [upgrade verification](../fault_proofs/upgrade.md#verification) guide for more details.

---

### Phase 2: Activate the ZK Game Type

This is the cutover step. After this, the chain recognizes ZK games as the canonical proof type.

#### 2.1 Set respected game type

Call `setRespectedGameType` on the `AnchorStateRegistry`. This requires the **Guardian** key. Use `6` for validity mode or `42` for fault proof mode:

```bash
cast send $ANCHOR_STATE_REGISTRY_ADDRESS "setRespectedGameType(uint32)" <GAME_TYPE> \
  --rpc-url $L1_RPC --private-key $GUARDIAN_PRIVATE_KEY
```

Verify:

```bash
cast call $ANCHOR_STATE_REGISTRY_ADDRESS "respectedGameType()" --rpc-url $L1_RPC
```

```admonish important
Only start the OP Succinct proposer **after** `setRespectedGameType` is confirmed. Games created before this call will have `wasRespectedGameTypeWhenCreated = false` and cannot be used for withdrawal proofs.
```

#### 2.2 Start the proposer

**Validity mode:** Create a `.env` file with proposer configuration. See [Proposer Configuration](../validity/proposer.md) for the full list of variables.

Migration-specific notes:
- Set `L2OO_ADDRESS` to the `OPSuccinctL2OutputOracle` proxy deployed in Phase 1.
- Set `DGF_ADDRESS` to your existing `DisputeGameFactory` address. This makes the proposer create dispute games via the factory (required for `OptimismPortal2` compatibility).
- Consider starting with `OP_SUCCINCT_MOCK=true` for initial validation, then switching to real proofs.

```bash
docker compose up
```

**Fault proof mode:** Create `.env.proposer` in the `fault-proof` directory. See [Proposer Configuration](../fault_proofs/proposer.md#configuration) for the full list of variables.

Migration-specific notes:
- Use the `ANCHOR_STATE_REGISTRY_ADDRESS` and `FACTORY_ADDRESS` from your existing deployment.
- Set `GAME_TYPE=42`.
- Consider starting with `MOCK_MODE=true` for initial validation, then switching to real proofs.

```bash
cd fault-proof
cargo run --bin proposer
```

Watch for logs confirming ZK games are being created (e.g., `Game created successfully`).

#### 2.3 Start the challenger (fault proof mode only)

This step only applies to **OP Succinct Lite (fault proof mode)**. Validity mode does not require a separate challenger.

Create `.env.challenger` in the `fault-proof` directory. See [Challenger Configuration](../fault_proofs/challenger.md#configuration) for the full reference.

```bash
cargo run --bin challenger
```

#### 2.4 Verify migration

| What to Check | How |
|---------------|-----|
| New impl registered | `cast call $FACTORY_ADDRESS "gameImpls(uint32)" <GAME_TYPE>` returns non-zero |
| Respected type updated | `cast call $ANCHOR_STATE_REGISTRY_ADDRESS "respectedGameType()"` returns your game type |
| ZK proposer creating games | Proposer logs show games being created |
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
| Games resolving/finalizing normally | Proposer logs show resolution |
| Bonds being claimed | Log: `Claimed bond` (fault proof mode) |
| Challenger active | Log: `Game challenged successfully` (fault proof mode only) |
| Old games winding down | Decreasing count of unresolved old-type games |

For metrics endpoints, see [Validity Proposer](../validity/proposer.md) (`METRICS_PORT`) or [FP Proposer](../fault_proofs/proposer.md#optional-environment-variables) (`PROPOSER_METRICS_PORT`) and [FP Challenger](../fault_proofs/challenger.md#optional-environment-variables) (`CHALLENGER_METRICS_PORT`).

## Withdrawal Safety Details

For operators who want a deeper understanding of withdrawal behavior during migration:

### How withdrawals work across the game type switch

- **Old games** (created before switch): `wasRespectedGameTypeWhenCreated = true` (immutable). Withdrawals can be proven and finalized against these games normally.
- **New ZK games**: `wasRespectedGameTypeWhenCreated = true` since the new type is now respected. Work normally.

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
