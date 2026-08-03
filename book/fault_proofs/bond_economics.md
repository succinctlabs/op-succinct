# Bond Economics and Sizing

OP Succinct Lite uses a proposer bond and a challenger bond for each dispute game.
The SP1 proof verifies the proposed state transition.
The bonds make nuisance proposals and challenges costly and provide a reward for defending a valid proposal.

Each game accepts one challenge and opens one proof window.
Several games can be challenged at the same time, so operators must account for concurrent games.

## Bond Roles

| Bond | Posted by | Purpose |
|------|-----------|---------|
| Proposer bond | The game creator | Prices game creation and locks proposer capital while the game is open |
| Challenger bond | The challenger | Prices challenges and rewards a valid defense |

Both bonds remain locked until the game is resolved, finalized, and claimed.
See [Architecture: Reward Distribution](./fault_proof_architecture.md#reward-distribution) for the payout rules and contract flow.

## Size the Challenger Bond

Use the conservative cost of defending one challenged production game as the sizing base.

```text
defense cost
  = range proof cost
  + aggregation proof cost
  + L1 proof submission cost
  + operating buffer

challenger bond = about 10 * conservative defense cost
```

Treat 10 times the defense cost as a starting point.
The defender must pay the proving and L1 submission costs before it can claim the challenger bond.
Reserve enough proving funds to defend concurrent games within `MAX_PROVE_DURATION`.

Use representative production ranges and the production data availability mode when estimating range proof demand.
The [cost estimator](../advanced/cost-estimation-tools.md#cost-estimator) reports range-program execution statistics and SP1 gas.
It does not provide the proving rate or exchange rate, and it does not include aggregation proving or L1 submission.
These values are operator inputs when converting the estimate to ETH.

Confirm that at least one honest challenger can fund the selected bond and L1 gas.
A bond that is too high can reduce the number of parties able to challenge an invalid proposal.

## Size the Proposer Bond

The proposer posts a new initial bond for every game that it creates.
The main sizing constraint is the amount of proposer capital locked across open games.

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

The lock horizon covers a challenge submitted near the end of the challenge window.
Parent-game dependencies, failed transactions, and delayed claims can extend it.
After launch, use the measured peak number of unclaimed games to refine the estimate.

The initial bond should leave enough capital for the calculated peak and an operating reserve.
It should also make invalid or abandoned proposals costly, especially when proposing is permissionless.

## Configure and Review Bonds

Set `INITIAL_BOND_WEI` and `CHALLENGER_BOND_WEI` during [contract deployment](./deploy.md#optional-environment-variables).
See the [upgrade guide](./upgrade.md) when changing bond configuration after deployment.

Review both bonds after changes to workload, proposal interval, data availability mode, program version, proving rate, L1 fees, or asset exchange rates.
Monitor the number of unclaimed games and the available proposer, challenger, and proving balances.
