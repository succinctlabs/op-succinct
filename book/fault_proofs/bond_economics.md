# Bond Economics and Sizing

OP Succinct Lite uses a proposer bond and a challenger bond for each dispute game.
The SP1 proof verifies the proposed state transition.
The bonds price participation and fund the response to a challenge.
Operators must also maintain at least one funded challenger that monitors games.

Each game accepts one challenge.
A challenge starts one proof window.
This single-round flow lets operators size bonds around the cost and concurrency of one game.

## Bond Roles

| Bond | Posted by | Purpose | Normal outcome |
|------|-----------|---------|----------------|
| Proposer bond | The game creator when it creates a game | Prices game creation and backs the proposal | Returned after a defender win |
| Challenger bond | The challenger when it calls `challenge()` | Prices challenges and funds the proof response | Paid to the prover after a valid proof |

The proposer bond is the `DisputeGameFactory` initial bond for the game type.
The challenger bond is an immutable value in the game implementation.
Both deposits remain in the game contract until the game is resolved, finalized, and claimed.

The contract distributes the bonds as follows:

| Game outcome | Proposer bond | Challenger bond |
|--------------|---------------|-----------------|
| Unchallenged | Returned to proposer | No challenger bond was posted |
| Unchallenged with a valid proof | Returned to proposer | No challenger bond was posted, so the prover receives no reward |
| Challenged with a valid proof | Returned to proposer | Paid to the prover |
| Challenged without a valid proof before the deadline | Paid to challenger | Returned to challenger |
| Improper game | Refunded to proposer | Refunded to challenger if one was posted |

See [Architecture: Reward Distribution](./fault_proof_architecture.md#reward-distribution) for the contract flow.
If a parent game resolves with `CHALLENGER_WINS`, its descendants also lose.
An unchallenged descendant has no challenger recipient, so its proposer bond is lost.

## Size the Challenger Bond

Use the marginal cost of defending one production game as the sizing base.
Include every cost that a nuisance challenge forces an honest defender to pay.
Express each cost in ETH before applying the sizing multiplier.

```text
defense cost
  = range proof cost
  + aggregation proof cost
  + L1 proof submission cost
  + operating buffer

challenger bond = about 10 * conservative defense cost
```

Treat 10 times the defense cost as a starting point.
Confirm that an honest challenger can fund the resulting bond and L1 gas before deployment.
A high challenger bond can reduce the number of parties that can challenge an invalid proposal.

Use this process to estimate the defense cost:

1. Select historical L2 ranges that match `PROPOSAL_INTERVAL_IN_BLOCKS` and the expected production workload.
2. Include high-load ranges and the production data availability mode.
3. Split each game range as the proposer does with `RANGE_SPLIT_COUNT`.
4. Run the [cost estimator](../advanced/cost-estimation-tools.md#cost-estimator) for each segment with `--no-safe-head-split` and the game L1 head.
5. Sum the reported SP1 gas and convert it to ETH with the current proving price and exchange rate.
6. Add aggregation proving, L1 proof submission, and an operating buffer.
7. Use the highest credible result from the sample.

The cost estimator reports range-program execution statistics and SP1 gas.
It does not calculate the full monetary cost of defending a game.
Aggregation proving and the L1 transaction must be added separately.

One game accepts one challenge, while several games can be challenged at the same time.
Provision enough challenger liquidity for the expected number of concurrent invalid games.
Provision enough proving capacity to defend all valid games within `MAX_PROVE_DURATION`.

```text
challenger liquidity
  = challenger bond * concurrent challenges
  + L1 gas reserve
```

## Size the Proposer Bond

The proposer posts a new initial bond for every game that it creates.
The bond cannot fund the next game while it remains locked in the current game.
The main sizing constraint is the amount of proposer capital that can remain locked across open games.

Estimate a conservative lock horizon:

```text
proposal period
  = PROPOSAL_INTERVAL_IN_BLOCKS * average L2 block time

lock horizon
  = MAX_CHALLENGE_DURATION
  + MAX_PROVE_DURATION
  + DISPUTE_GAME_FINALITY_DELAY_SECONDS
  + operating buffer

peak open games
  = ceil(lock horizon / proposal period)
  + concurrency buffer

proposer capital
  = INITIAL_BOND_WEI * peak open games
  + L1 gas reserve
```

The full lock horizon covers a challenge submitted near the end of the challenge window.
An unchallenged game usually releases its credit sooner because it does not use the proving window.
Parent-game dependencies, failed transactions, and delayed claims can extend the observed lock time.
Use the measured peak number of unclaimed games to refine the estimate after launch.

Choose an initial bond that leaves enough capital for the calculated peak and a safe operating reserve.
The amount should also make invalid or abandoned proposals costly.
Permissionless proposing requires extra attention to that deterrence value.

## Example

This example shows the method with illustrative values.
Measure the inputs for each production deployment.

Assume the following configuration:

- Average L2 block time: 2 seconds.
- Proposal interval: 1,800 blocks, or 1 hour.
- Maximum challenge duration: 7 days.
- Maximum prove duration: 1 day.
- Dispute game finality delay: 7 days.
- Initial bond: 0.01 ETH per game.
- Conservative defense cost: 0.005 ETH per game.

The conservative lock horizon is 15 days before adding an operating buffer.
An hourly proposal cadence produces about 360 open games during that period.
The proposer therefore needs about 3.6 ETH for game bonds, plus the concurrency buffer and L1 gas reserve.
The challenger bond starts near 0.05 ETH under the 10 times rule.
The operator must then confirm that an honest challenger can fund this amount across the expected concurrency.

## Configure and Review Bonds

Set `INITIAL_BOND_WEI` and `CHALLENGER_BOND_WEI` during [contract deployment](./deploy.md#optional-environment-variables).
Use `cast --to-wei <value> eth` to convert ETH to wei.

Read the current proposer bond from the factory:

```bash
cast call <FACTORY_ADDRESS> \
  "initBonds(uint32)(uint256)" <GAME_TYPE> \
  --rpc-url <L1_RPC>
```

Read the current game implementation and challenger bond:

```bash
cast call <FACTORY_ADDRESS> \
  "gameImpls(uint32)(address)" <GAME_TYPE> \
  --rpc-url <L1_RPC>

cast call <GAME_IMPLEMENTATION> \
  "challengerBond()(uint256)" \
  --rpc-url <L1_RPC>
```

The factory initial bond can be changed for future games.
Changing the challenger bond requires a [new game implementation](./upgrade.md).
Existing games keep the deposits that they received at creation and challenge time.

Recalculate both bonds after changes to workload, proposal interval, data availability mode, program version, proving price, L1 fees, or asset exchange rates.
Monitor the number of unclaimed games and the available proposer and challenger balances after deployment.
