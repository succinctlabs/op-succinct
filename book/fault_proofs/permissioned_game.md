# Permissioned Fault Dispute Game

The `OPSuccinctPermissionedFaultDisputeGame` is a specialized child contract of the `OPSuccinctFaultDisputeGame` that implements role-based access control to restrict who can participate in the game. This contract can serve as a fallback in cases where a permissionless system is undesirable or if the existing permissionless system encounters issues.

## Overview
In a typical fault proof system, multiple entities may freely propose and challenge state roots in order to reach consensus. However, permissionless designs can introduce complexity and costs that some networks might prefer to avoid. By contrast, the `OPSuccinctPermissionedFaultDisputeGame` restricts participation via two new roles: a `Proposer` and a `Challenger`.

Each relevant interaction—including `prove`, `challenge`, and `resolve`—is accessible only to addresses that have been granted one of these roles. Furthermore, the `initialize()` function is exclusively reserved for the `Proposer`.

In the event that you wish to switch back to a permissionless approach, the `RESPECTED_GAME_TYPE` parameter in the `OptimismPortal` can be changed to reference a different dispute game deployment.

## Roles
This contract introduces two immutable roles:

`Proposer`

- Can create a new `PermissionedFaultDisputeGame` through the factory contract.
- Can participate in the games they have created (submit proofs, resolve games, etc.).
- Is stored in the internal immutable variable `PROPOSER`.

`Challenger`

- Can challenge games created by the `Proposer`.
- Is stored in the internal immutable variable `CHALLENGER`.

These roles are set once in the constructor and cannot be changed afterward.

## Access Control Modifiers
To enforce the restricted access, each state-mutating function in this contract is overridden and guarded by at least one of the following checks:

- `onlyProposer`: Ensures the caller is the `PROPOSER`.
- `onlyChallenger`: Ensures the caller is the `CHALLENGER`.

Any call from an address outside of `PROPOSER` or `CHALLENGER` will revert.

Additionally, the `initialize()` function is only callable if `tx.origin` is the `PROPOSER`. This further ensures that no other address can set up or finalize the initial parameters of a newly created game.

## Benefits

- Controlled Environment: Provides a more controlled environment for dispute resolution compared to a fully permissionless system.
- Cost Management: Limits the number of participants, potentially reducing on-chain costs associated with open participation.
- Fallback Mechanism: Serves as a backup system if the primary (permissionless) mechanism fails or becomes prohibitively expensive.
- Simplified Security Model: By limiting who can interact, the attack surface is reduced, improving overall security.

## Limitations

- Centralization: Restricting access to just two addresses introduces a more centralized approach.
- Immutable Roles: Once set, the roles cannot be changed, so mistakes in deployment or role assignment are permanent. Need to redeploy the contract to change the roles.
