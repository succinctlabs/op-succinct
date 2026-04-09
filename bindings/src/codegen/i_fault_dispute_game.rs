///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Types {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputRootProof {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub stateRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub messagePasserStorageRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub latestBlockhash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OutputRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: OutputRootProof) -> Self {
                (
                    value.version,
                    value.stateRoot,
                    value.messagePasserStorageRoot,
                    value.latestBlockhash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    version: tuple.0,
                    stateRoot: tuple.1,
                    messagePasserStorageRoot: tuple.2,
                    latestBlockhash: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputRootProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputRootProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stateRoot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.messagePasserStorageRoot,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.latestBlockhash),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OutputRootProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OutputRootProof {
            const NAME: &'static str = "OutputRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputRootProof(bytes32 version,bytes32 stateRoot,bytes32 messagePasserStorageRoot,bytes32 latestBlockhash)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.version)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stateRoot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.messagePasserStorageRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.latestBlockhash,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputRootProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.version,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stateRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.messagePasserStorageRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.latestBlockhash,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.version,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stateRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.messagePasserStorageRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.latestBlockhash,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> TypesInstance<P, N> {
        TypesInstance::<P, N>::new(address, __provider)
    }
    /**A [`Types`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Types`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for TypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> TypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TypesInstance<P, N> {
            TypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library Types {
    struct OutputRootProof {
        bytes32 version;
        bytes32 stateRoot;
        bytes32 messagePasserStorageRoot;
        bytes32 latestBlockhash;
    }
}

interface IFaultDisputeGame {
    type BondDistributionMode is uint8;
    type GameStatus is uint8;
    type Claim is bytes32;
    type Clock is uint128;
    type Duration is uint64;
    type GameType is uint32;
    type Hash is bytes32;
    type Position is uint128;
    type Timestamp is uint64;
    struct GameConstructorParams {
        GameType gameType;
        Claim absolutePrestate;
        uint256 maxGameDepth;
        uint256 splitDepth;
        Duration clockExtension;
        Duration maxClockDuration;
        address vm;
        address weth;
        address anchorStateRegistry;
        uint256 l2ChainId;
    }

    error AlreadyInitialized();
    error AnchorRootNotFound();
    error BadExtraData();
    error BlockNumberMatches();
    error BondTransferFailed();
    error CannotDefendRootClaim();
    error ClaimAboveSplit();
    error ClaimAlreadyExists();
    error ClaimAlreadyResolved();
    error ClockNotExpired();
    error ClockTimeExceeded();
    error ContentLengthMismatch();
    error DuplicateStep();
    error EmptyItem();
    error GameDepthExceeded();
    error GameNotFinalized();
    error GameNotInProgress();
    error GameNotResolved();
    error GamePaused();
    error IncorrectBondAmount();
    error InvalidBondDistributionMode();
    error InvalidChallengePeriod();
    error InvalidClockExtension();
    error InvalidDataRemainder();
    error InvalidDisputedClaimIndex();
    error InvalidHeader();
    error InvalidHeaderRLP();
    error InvalidLocalIdent();
    error InvalidOutputRootProof();
    error InvalidParent();
    error InvalidPrestate();
    error InvalidSplitDepth();
    error L2BlockNumberChallenged();
    error MaxDepthTooLarge();
    error NoCreditToClaim();
    error OutOfOrderResolution();
    error ReservedGameType();
    error UnexpectedList();
    error UnexpectedRootClaim(Claim rootClaim);
    error UnexpectedString();
    error ValidStep();

    event GameClosed(BondDistributionMode bondDistributionMode);
    event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
    event Resolved(GameStatus indexed status);

    function __constructor__(GameConstructorParams memory _params) external;
    function absolutePrestate() external view returns (Claim absolutePrestate_);
    function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
    function anchorStateRegistry() external view returns (address registry_);
    function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function bondDistributionMode() external view returns (BondDistributionMode);
    function challengeRootL2Block(Types.OutputRootProof memory _outputRootProof, bytes memory _headerRLP) external;
    function claimCredit(address _recipient) external;
    function claimData(uint256) external view returns (uint32 parentIndex, address counteredBy, address claimant, uint128 bond, Claim claim, Position position, Clock clock);
    function claimDataLen() external view returns (uint256 len_);
    function claims(Hash) external view returns (bool);
    function clockExtension() external view returns (Duration clockExtension_);
    function closeGame() external;
    function createdAt() external view returns (Timestamp);
    function credit(address _recipient) external view returns (uint256 credit_);
    function defend(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function extraData() external pure returns (bytes memory extraData_);
    function gameCreator() external pure returns (address creator_);
    function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
    function gameType() external view returns (GameType gameType_);
    function getChallengerDuration(uint256 _claimIndex) external view returns (Duration duration_);
    function getNumToResolve(uint256 _claimIndex) external view returns (uint256 numRemainingChildren_);
    function getRequiredBond(Position _position) external view returns (uint256 requiredBond_);
    function hasUnlockedCredit(address) external view returns (bool);
    function initialize() external payable;
    function l1Head() external pure returns (Hash l1Head_);
    function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
    function l2BlockNumberChallenged() external view returns (bool);
    function l2BlockNumberChallenger() external view returns (address);
    function l2ChainId() external view returns (uint256 l2ChainId_);
    function l2SequenceNumber() external pure returns (uint256 l2SequenceNumber_);
    function maxClockDuration() external view returns (Duration maxClockDuration_);
    function maxGameDepth() external view returns (uint256 maxGameDepth_);
    function move(Claim _disputed, uint256 _challengeIndex, Claim _claim, bool _isAttack) external payable;
    function normalModeCredit(address) external view returns (uint256);
    function refundModeCredit(address) external view returns (uint256);
    function resolutionCheckpoints(uint256) external view returns (bool initialCheckpointComplete, uint32 subgameIndex, Position leftmostPosition, address counteredBy);
    function resolve() external returns (GameStatus status_);
    function resolveClaim(uint256 _claimIndex, uint256 _numToResolve) external;
    function resolvedAt() external view returns (Timestamp);
    function resolvedSubgames(uint256) external view returns (bool);
    function rootClaim() external pure returns (Claim rootClaim_);
    function splitDepth() external view returns (uint256 splitDepth_);
    function startingBlockNumber() external view returns (uint256 startingBlockNumber_);
    function startingOutputRoot() external view returns (Hash root, uint256 l2SequenceNumber);
    function startingRootHash() external view returns (Hash startingRootHash_);
    function status() external view returns (GameStatus);
    function step(uint256 _claimIndex, bool _isAttack, bytes memory _stateData, bytes memory _proof) external;
    function subgames(uint256, uint256) external view returns (uint256);
    function version() external pure returns (string memory);
    function vm() external view returns (address vm_);
    function wasRespectedGameTypeWhenCreated() external view returns (bool);
    function weth() external view returns (address weth_);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "__constructor__",
    "inputs": [
      {
        "name": "_params",
        "type": "tuple",
        "internalType": "struct IFaultDisputeGame.GameConstructorParams",
        "components": [
          {
            "name": "gameType",
            "type": "uint32",
            "internalType": "GameType"
          },
          {
            "name": "absolutePrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "maxGameDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "splitDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "clockExtension",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "maxClockDuration",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "vm",
            "type": "address",
            "internalType": "contract IBigStepper"
          },
          {
            "name": "weth",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          },
          {
            "name": "anchorStateRegistry",
            "type": "address",
            "internalType": "contract IAnchorStateRegistry"
          },
          {
            "name": "l2ChainId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "absolutePrestate",
    "inputs": [],
    "outputs": [
      {
        "name": "absolutePrestate_",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addLocalData",
    "inputs": [
      {
        "name": "_ident",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_execLeafIdx",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "anchorStateRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "registry_",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "attack",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_parentIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "bondDistributionMode",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum BondDistributionMode"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "challengeRootL2Block",
    "inputs": [
      {
        "name": "_outputRootProof",
        "type": "tuple",
        "internalType": "struct Types.OutputRootProof",
        "components": [
          {
            "name": "version",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "messagePasserStorageRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "latestBlockhash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_headerRLP",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimCredit",
    "inputs": [
      {
        "name": "_recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimData",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "parentIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "counteredBy",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "bond",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "claim",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "position",
        "type": "uint128",
        "internalType": "Position"
      },
      {
        "name": "clock",
        "type": "uint128",
        "internalType": "Clock"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claimDataLen",
    "inputs": [],
    "outputs": [
      {
        "name": "len_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claims",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "Hash"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "clockExtension",
    "inputs": [],
    "outputs": [
      {
        "name": "clockExtension_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeGame",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createdAt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "Timestamp"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "credit",
    "inputs": [
      {
        "name": "_recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "credit_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "defend",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_parentIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "extraData",
    "inputs": [],
    "outputs": [
      {
        "name": "extraData_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "gameCreator",
    "inputs": [],
    "outputs": [
      {
        "name": "creator_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "gameData",
    "inputs": [],
    "outputs": [
      {
        "name": "gameType_",
        "type": "uint32",
        "internalType": "GameType"
      },
      {
        "name": "rootClaim_",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "extraData_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "gameType",
    "inputs": [],
    "outputs": [
      {
        "name": "gameType_",
        "type": "uint32",
        "internalType": "GameType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getChallengerDuration",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "duration_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNumToResolve",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "numRemainingChildren_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRequiredBond",
    "inputs": [
      {
        "name": "_position",
        "type": "uint128",
        "internalType": "Position"
      }
    ],
    "outputs": [
      {
        "name": "requiredBond_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hasUnlockedCredit",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "l1Head",
    "inputs": [],
    "outputs": [
      {
        "name": "l1Head_",
        "type": "bytes32",
        "internalType": "Hash"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "l2BlockNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "l2BlockNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "l2BlockNumberChallenged",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l2BlockNumberChallenger",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l2ChainId",
    "inputs": [],
    "outputs": [
      {
        "name": "l2ChainId_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l2SequenceNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "l2SequenceNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "maxClockDuration",
    "inputs": [],
    "outputs": [
      {
        "name": "maxClockDuration_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxGameDepth",
    "inputs": [],
    "outputs": [
      {
        "name": "maxGameDepth_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "move",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_challengeIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_isAttack",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "normalModeCredit",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "refundModeCredit",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "resolutionCheckpoints",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "initialCheckpointComplete",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "subgameIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "leftmostPosition",
        "type": "uint128",
        "internalType": "Position"
      },
      {
        "name": "counteredBy",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "resolve",
    "inputs": [],
    "outputs": [
      {
        "name": "status_",
        "type": "uint8",
        "internalType": "enum GameStatus"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolveClaim",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_numToResolve",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolvedAt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "Timestamp"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "resolvedSubgames",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "rootClaim",
    "inputs": [],
    "outputs": [
      {
        "name": "rootClaim_",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "splitDepth",
    "inputs": [],
    "outputs": [
      {
        "name": "splitDepth_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingBlockNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "startingBlockNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingOutputRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "Hash"
      },
      {
        "name": "l2SequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingRootHash",
    "inputs": [],
    "outputs": [
      {
        "name": "startingRootHash_",
        "type": "bytes32",
        "internalType": "Hash"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "status",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum GameStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "step",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_isAttack",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_stateData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_proof",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "subgames",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "vm",
    "inputs": [],
    "outputs": [
      {
        "name": "vm_",
        "type": "address",
        "internalType": "contract IBigStepper"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "wasRespectedGameTypeWhenCreated",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "weth",
    "inputs": [],
    "outputs": [
      {
        "name": "weth_",
        "type": "address",
        "internalType": "contract IDelayedWETH"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "GameClosed",
    "inputs": [
      {
        "name": "bondDistributionMode",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum BondDistributionMode"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Move",
    "inputs": [
      {
        "name": "parentIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "claim",
        "type": "bytes32",
        "indexed": true,
        "internalType": "Claim"
      },
      {
        "name": "claimant",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Resolved",
    "inputs": [
      {
        "name": "status",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum GameStatus"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AnchorRootNotFound",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadExtraData",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BlockNumberMatches",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BondTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotDefendRootClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAboveSplit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAlreadyResolved",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClockNotExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClockTimeExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContentLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DuplicateStep",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyItem",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameDepthExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotFinalized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotInProgress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotResolved",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GamePaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IncorrectBondAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBondDistributionMode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChallengePeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidClockExtension",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDataRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDisputedClaimIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeader",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeaderRLP",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLocalIdent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOutputRootProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidParent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPrestate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSplitDepth",
    "inputs": []
  },
  {
    "type": "error",
    "name": "L2BlockNumberChallenged",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxDepthTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoCreditToClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfOrderResolution",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReservedGameType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnexpectedList",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnexpectedRootClaim",
    "inputs": [
      {
        "name": "rootClaim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnexpectedString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidStep",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IFaultDisputeGame {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BondDistributionMode(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BondDistributionMode> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl BondDistributionMode {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for BondDistributionMode {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<BondDistributionMode> for u8 {
            fn from(value: BondDistributionMode) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BondDistributionMode {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BondDistributionMode {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<GameStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl GameStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for GameStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<GameStatus> for u8 {
            fn from(value: GameStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GameStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Claim(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Claim>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Claim {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::FixedBytes<32>,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::FixedBytes<32> {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<alloy::sol_types::private::FixedBytes<32>> for Claim {
            fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Claim> for alloy::sol_types::private::FixedBytes<32> {
            fn from(value: Claim) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Claim {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Claim {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Clock(u128);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Clock> for u128 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<128>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Clock {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u128) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u128 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u128> for Clock {
            fn from(value: u128) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Clock> for u128 {
            fn from(value: Clock) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Clock {
            type RustType = u128;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Clock {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Duration(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Duration> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Duration {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u64> for Duration {
            fn from(value: u64) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Duration> for u64 {
            fn from(value: Duration) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Duration {
            type RustType = u64;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Duration {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameType(u32);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<GameType> for u32 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl GameType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u32) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u32 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u32> for GameType {
            fn from(value: u32) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<GameType> for u32 {
            fn from(value: GameType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GameType {
            type RustType = u32;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameType {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Hash(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Hash>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Hash {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::FixedBytes<32>,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::FixedBytes<32> {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<alloy::sol_types::private::FixedBytes<32>> for Hash {
            fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Hash> for alloy::sol_types::private::FixedBytes<32> {
            fn from(value: Hash) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Hash {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Hash {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Position(u128);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Position> for u128 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<128>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Position {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u128) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u128 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u128> for Position {
            fn from(value: u128) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Position> for u128 {
            fn from(value: Position) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Position {
            type RustType = u128;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Position {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Timestamp(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Timestamp> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Timestamp {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u64> for Timestamp {
            fn from(value: u64) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Timestamp> for u64 {
            fn from(value: Timestamp) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Timestamp {
            type RustType = u64;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Timestamp {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct GameConstructorParams { GameType gameType; Claim absolutePrestate; uint256 maxGameDepth; uint256 splitDepth; Duration clockExtension; Duration maxClockDuration; address vm; address weth; address anchorStateRegistry; uint256 l2ChainId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameConstructorParams {
        #[allow(missing_docs)]
        pub gameType: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub absolutePrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub splitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clockExtension: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxClockDuration: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub vm: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub weth: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub anchorStateRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            GameType,
            Claim,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <GameType as alloy::sol_types::SolType>::RustType,
            <Claim as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <Duration as alloy::sol_types::SolType>::RustType,
            <Duration as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GameConstructorParams> for UnderlyingRustTuple<'_> {
            fn from(value: GameConstructorParams) -> Self {
                (
                    value.gameType,
                    value.absolutePrestate,
                    value.maxGameDepth,
                    value.splitDepth,
                    value.clockExtension,
                    value.maxClockDuration,
                    value.vm,
                    value.weth,
                    value.anchorStateRegistry,
                    value.l2ChainId,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameConstructorParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    gameType: tuple.0,
                    absolutePrestate: tuple.1,
                    maxGameDepth: tuple.2,
                    splitDepth: tuple.3,
                    clockExtension: tuple.4,
                    maxClockDuration: tuple.5,
                    vm: tuple.6,
                    weth: tuple.7,
                    anchorStateRegistry: tuple.8,
                    l2ChainId: tuple.9,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GameConstructorParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GameConstructorParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <GameType as alloy_sol_types::SolType>::tokenize(&self.gameType),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.absolutePrestate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGameDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.splitDepth),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.clockExtension,
                    ),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.maxClockDuration,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.vm,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.weth,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.anchorStateRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2ChainId),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GameConstructorParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for GameConstructorParams {
            const NAME: &'static str = "GameConstructorParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GameConstructorParams(uint32 gameType,bytes32 absolutePrestate,uint256 maxGameDepth,uint256 splitDepth,uint64 clockExtension,uint64 maxClockDuration,address vm,address weth,address anchorStateRegistry,uint256 l2ChainId)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <GameType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.gameType,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.absolutePrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxGameDepth)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.splitDepth)
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.clockExtension,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxClockDuration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.vm,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.weth,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.anchorStateRegistry,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2ChainId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameConstructorParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <GameType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gameType,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.absolutePrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxGameDepth,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.splitDepth,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.clockExtension,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxClockDuration,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.vm,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.weth,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.anchorStateRegistry,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2ChainId,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <GameType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gameType,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.absolutePrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxGameDepth,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.splitDepth,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.clockExtension,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxClockDuration,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.vm,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.weth,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.anchorStateRegistry,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2ChainId,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `AnchorRootNotFound()` and selector `0x6a6bc3b2`.
```solidity
error AnchorRootNotFound();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AnchorRootNotFound;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AnchorRootNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: AnchorRootNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AnchorRootNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AnchorRootNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AnchorRootNotFound()";
            const SELECTOR: [u8; 4] = [106u8, 107u8, 195u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `BadExtraData()` and selector `0x9824bdab`.
```solidity
error BadExtraData();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadExtraData;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BadExtraData> for UnderlyingRustTuple<'_> {
            fn from(value: BadExtraData) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadExtraData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadExtraData {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadExtraData()";
            const SELECTOR: [u8; 4] = [152u8, 36u8, 189u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `BlockNumberMatches()` and selector `0xb8ed8830`.
```solidity
error BlockNumberMatches();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BlockNumberMatches;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BlockNumberMatches> for UnderlyingRustTuple<'_> {
            fn from(value: BlockNumberMatches) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockNumberMatches {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BlockNumberMatches {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BlockNumberMatches()";
            const SELECTOR: [u8; 4] = [184u8, 237u8, 136u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `BondTransferFailed()` and selector `0x83e6cc6b`.
```solidity
error BondTransferFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BondTransferFailed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BondTransferFailed> for UnderlyingRustTuple<'_> {
            fn from(value: BondTransferFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BondTransferFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BondTransferFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BondTransferFailed()";
            const SELECTOR: [u8; 4] = [131u8, 230u8, 204u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `CannotDefendRootClaim()` and selector `0xa42637bc`.
```solidity
error CannotDefendRootClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotDefendRootClaim;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotDefendRootClaim> for UnderlyingRustTuple<'_> {
            fn from(value: CannotDefendRootClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotDefendRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotDefendRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotDefendRootClaim()";
            const SELECTOR: [u8; 4] = [164u8, 38u8, 55u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ClaimAboveSplit()` and selector `0xb34b5c22`.
```solidity
error ClaimAboveSplit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAboveSplit;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ClaimAboveSplit> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAboveSplit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAboveSplit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAboveSplit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAboveSplit()";
            const SELECTOR: [u8; 4] = [179u8, 75u8, 92u8, 34u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ClaimAlreadyExists()` and selector `0x80497e3b`.
```solidity
error ClaimAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAlreadyExists;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ClaimAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAlreadyExists()";
            const SELECTOR: [u8; 4] = [128u8, 73u8, 126u8, 59u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ClaimAlreadyResolved()` and selector `0xf1a94581`.
```solidity
error ClaimAlreadyResolved();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAlreadyResolved;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ClaimAlreadyResolved> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAlreadyResolved) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAlreadyResolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAlreadyResolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAlreadyResolved()";
            const SELECTOR: [u8; 4] = [241u8, 169u8, 69u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ClockNotExpired()` and selector `0xf2440b53`.
```solidity
error ClockNotExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClockNotExpired;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ClockNotExpired> for UnderlyingRustTuple<'_> {
            fn from(value: ClockNotExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClockNotExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClockNotExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClockNotExpired()";
            const SELECTOR: [u8; 4] = [242u8, 68u8, 11u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ClockTimeExceeded()` and selector `0x3381d114`.
```solidity
error ClockTimeExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClockTimeExceeded;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ClockTimeExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: ClockTimeExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClockTimeExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClockTimeExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClockTimeExceeded()";
            const SELECTOR: [u8; 4] = [51u8, 129u8, 209u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ContentLengthMismatch()` and selector `0x66c94485`.
```solidity
error ContentLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContentLengthMismatch;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContentLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: ContentLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContentLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContentLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContentLengthMismatch()";
            const SELECTOR: [u8; 4] = [102u8, 201u8, 68u8, 133u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `DuplicateStep()` and selector `0x9071e6af`.
```solidity
error DuplicateStep();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateStep;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DuplicateStep> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateStep) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateStep {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateStep {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateStep()";
            const SELECTOR: [u8; 4] = [144u8, 113u8, 230u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `EmptyItem()` and selector `0x5ab458fb`.
```solidity
error EmptyItem();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyItem;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyItem> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyItem) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyItem {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyItem {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyItem()";
            const SELECTOR: [u8; 4] = [90u8, 180u8, 88u8, 251u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `GameDepthExceeded()` and selector `0x56f57b2b`.
```solidity
error GameDepthExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameDepthExceeded;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GameDepthExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: GameDepthExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameDepthExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameDepthExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameDepthExceeded()";
            const SELECTOR: [u8; 4] = [86u8, 245u8, 123u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `GameNotFinalized()` and selector `0x4851bd9b`.
```solidity
error GameNotFinalized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotFinalized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GameNotFinalized> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotFinalized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotFinalized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotFinalized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotFinalized()";
            const SELECTOR: [u8; 4] = [72u8, 81u8, 189u8, 155u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `GameNotInProgress()` and selector `0x67fe1950`.
```solidity
error GameNotInProgress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotInProgress;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GameNotInProgress> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotInProgress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotInProgress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotInProgress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotInProgress()";
            const SELECTOR: [u8; 4] = [103u8, 254u8, 25u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `GameNotResolved()` and selector `0xc105260a`.
```solidity
error GameNotResolved();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotResolved;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GameNotResolved> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotResolved) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotResolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotResolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotResolved()";
            const SELECTOR: [u8; 4] = [193u8, 5u8, 38u8, 10u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `GamePaused()` and selector `0x379a7ed9`.
```solidity
error GamePaused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GamePaused;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GamePaused> for UnderlyingRustTuple<'_> {
            fn from(value: GamePaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GamePaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GamePaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GamePaused()";
            const SELECTOR: [u8; 4] = [55u8, 154u8, 126u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `IncorrectBondAmount()` and selector `0x8620aa19`.
```solidity
error IncorrectBondAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IncorrectBondAmount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<IncorrectBondAmount> for UnderlyingRustTuple<'_> {
            fn from(value: IncorrectBondAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IncorrectBondAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IncorrectBondAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IncorrectBondAmount()";
            const SELECTOR: [u8; 4] = [134u8, 32u8, 170u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidBondDistributionMode()` and selector `0x078a3df4`.
```solidity
error InvalidBondDistributionMode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBondDistributionMode;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBondDistributionMode>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBondDistributionMode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidBondDistributionMode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBondDistributionMode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBondDistributionMode()";
            const SELECTOR: [u8; 4] = [7u8, 138u8, 61u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidChallengePeriod()` and selector `0xb4e12433`.
```solidity
error InvalidChallengePeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidChallengePeriod;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidChallengePeriod> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidChallengePeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidChallengePeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidChallengePeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidChallengePeriod()";
            const SELECTOR: [u8; 4] = [180u8, 225u8, 36u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidClockExtension()` and selector `0x8d77ecac`.
```solidity
error InvalidClockExtension();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidClockExtension;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidClockExtension> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidClockExtension) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidClockExtension {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidClockExtension {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidClockExtension()";
            const SELECTOR: [u8; 4] = [141u8, 119u8, 236u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidDataRemainder()` and selector `0x5c5537b8`.
```solidity
error InvalidDataRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDataRemainder;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidDataRemainder> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDataRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidDataRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDataRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDataRemainder()";
            const SELECTOR: [u8; 4] = [92u8, 85u8, 55u8, 184u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidDisputedClaimIndex()` and selector `0x30140332`.
```solidity
error InvalidDisputedClaimIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDisputedClaimIndex;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidDisputedClaimIndex>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDisputedClaimIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidDisputedClaimIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDisputedClaimIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDisputedClaimIndex()";
            const SELECTOR: [u8; 4] = [48u8, 20u8, 3u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidHeader()` and selector `0xbabb01dd`.
```solidity
error InvalidHeader();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeader;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidHeader> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeader) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeader {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeader {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeader()";
            const SELECTOR: [u8; 4] = [186u8, 187u8, 1u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidHeaderRLP()` and selector `0xd81d583b`.
```solidity
error InvalidHeaderRLP();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeaderRLP;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidHeaderRLP> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeaderRLP) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeaderRLP {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeaderRLP {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeaderRLP()";
            const SELECTOR: [u8; 4] = [216u8, 29u8, 88u8, 59u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidLocalIdent()` and selector `0xff137e65`.
```solidity
error InvalidLocalIdent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLocalIdent;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLocalIdent> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLocalIdent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLocalIdent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLocalIdent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLocalIdent()";
            const SELECTOR: [u8; 4] = [255u8, 19u8, 126u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidOutputRootProof()` and selector `0x9cc00b5b`.
```solidity
error InvalidOutputRootProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOutputRootProof;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidOutputRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOutputRootProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOutputRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOutputRootProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOutputRootProof()";
            const SELECTOR: [u8; 4] = [156u8, 192u8, 11u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidParent()` and selector `0x5f53dd98`.
```solidity
error InvalidParent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidParent;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidParent> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidParent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidParent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidParent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidParent()";
            const SELECTOR: [u8; 4] = [95u8, 83u8, 221u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidPrestate()` and selector `0x696550ff`.
```solidity
error InvalidPrestate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPrestate;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidPrestate> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPrestate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPrestate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPrestate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPrestate()";
            const SELECTOR: [u8; 4] = [105u8, 101u8, 80u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `InvalidSplitDepth()` and selector `0xe62ccf39`.
```solidity
error InvalidSplitDepth();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSplitDepth;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSplitDepth> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSplitDepth) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSplitDepth {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSplitDepth {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSplitDepth()";
            const SELECTOR: [u8; 4] = [230u8, 44u8, 207u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `L2BlockNumberChallenged()` and selector `0x0ea2e752`.
```solidity
error L2BlockNumberChallenged();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L2BlockNumberChallenged;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<L2BlockNumberChallenged> for UnderlyingRustTuple<'_> {
            fn from(value: L2BlockNumberChallenged) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L2BlockNumberChallenged {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for L2BlockNumberChallenged {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L2BlockNumberChallenged()";
            const SELECTOR: [u8; 4] = [14u8, 162u8, 231u8, 82u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `MaxDepthTooLarge()` and selector `0x77dfe332`.
```solidity
error MaxDepthTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDepthTooLarge;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxDepthTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: MaxDepthTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxDepthTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDepthTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDepthTooLarge()";
            const SELECTOR: [u8; 4] = [119u8, 223u8, 227u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `NoCreditToClaim()` and selector `0x17bfe5f7`.
```solidity
error NoCreditToClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoCreditToClaim;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoCreditToClaim> for UnderlyingRustTuple<'_> {
            fn from(value: NoCreditToClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoCreditToClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoCreditToClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoCreditToClaim()";
            const SELECTOR: [u8; 4] = [23u8, 191u8, 229u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `OutOfOrderResolution()` and selector `0x9a076646`.
```solidity
error OutOfOrderResolution();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfOrderResolution;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OutOfOrderResolution> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfOrderResolution) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfOrderResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfOrderResolution {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfOrderResolution()";
            const SELECTOR: [u8; 4] = [154u8, 7u8, 102u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ReservedGameType()` and selector `0x39f68b38`.
```solidity
error ReservedGameType();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReservedGameType;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReservedGameType> for UnderlyingRustTuple<'_> {
            fn from(value: ReservedGameType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReservedGameType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReservedGameType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReservedGameType()";
            const SELECTOR: [u8; 4] = [57u8, 246u8, 139u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `UnexpectedList()` and selector `0x1ff9b2e4`.
```solidity
error UnexpectedList();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedList;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnexpectedList> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedList) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedList {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedList {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedList()";
            const SELECTOR: [u8; 4] = [31u8, 249u8, 178u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `UnexpectedRootClaim(bytes32)` and selector `0xf40239db`.
```solidity
error UnexpectedRootClaim(Claim rootClaim);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedRootClaim {
        #[allow(missing_docs)]
        pub rootClaim: <Claim as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (Claim,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (<Claim as alloy::sol_types::SolType>::RustType,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnexpectedRootClaim> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedRootClaim) -> Self {
                (value.rootClaim,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { rootClaim: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedRootClaim(bytes32)";
            const SELECTOR: [u8; 4] = [244u8, 2u8, 57u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Claim as alloy_sol_types::SolType>::tokenize(&self.rootClaim),)
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `UnexpectedString()` and selector `0x4b9c6abe`.
```solidity
error UnexpectedString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedString;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnexpectedString> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedString()";
            const SELECTOR: [u8; 4] = [75u8, 156u8, 106u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Custom error with signature `ValidStep()` and selector `0xfb4e40dd`.
```solidity
error ValidStep();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidStep;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidStep> for UnderlyingRustTuple<'_> {
            fn from(value: ValidStep) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidStep {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidStep {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidStep()";
            const SELECTOR: [u8; 4] = [251u8, 78u8, 64u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Event with signature `GameClosed(uint8)` and selector `0x9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f`.
```solidity
event GameClosed(BondDistributionMode bondDistributionMode);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GameClosed {
        #[allow(missing_docs)]
        pub bondDistributionMode: <BondDistributionMode as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for GameClosed {
            type DataTuple<'a> = (BondDistributionMode,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GameClosed(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                153u8, 8u8, 234u8, 172u8, 6u8, 69u8, 223u8, 157u8, 7u8, 4u8, 208u8,
                106u8, 220u8, 158u8, 7u8, 51u8, 124u8, 149u8, 29u8, 226u8, 240u8, 107u8,
                95u8, 40u8, 54u8, 21u8, 29u8, 72u8, 213u8, 228u8, 114u8, 47u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bondDistributionMode: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <BondDistributionMode as alloy_sol_types::SolType>::tokenize(
                        &self.bondDistributionMode,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for GameClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GameClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GameClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Move(uint256,bytes32,address)` and selector `0x9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be`.
```solidity
event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Move {
        #[allow(missing_docs)]
        pub parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub claimant: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Move {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Move(uint256,bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                155u8, 50u8, 69u8, 116u8, 14u8, 195u8, 177u8, 85u8, 9u8, 138u8, 85u8,
                190u8, 132u8, 149u8, 122u8, 77u8, 161u8, 62u8, 175u8, 127u8, 20u8, 168u8,
                188u8, 111u8, 83u8, 18u8, 108u8, 11u8, 147u8, 80u8, 242u8, 190u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    parentIndex: topics.1,
                    claim: topics.2,
                    claimant: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.parentIndex.clone(),
                    self.claim.clone(),
                    self.claimant.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.parentIndex);
                out[2usize] = <Claim as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claim,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claimant,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Move {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Move> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Move) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Resolved(uint8)` and selector `0x5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da60`.
```solidity
event Resolved(GameStatus indexed status);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Resolved {
        #[allow(missing_docs)]
        pub status: <GameStatus as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Resolved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, GameStatus);
            const SIGNATURE: &'static str = "Resolved(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                94u8, 24u8, 111u8, 9u8, 185u8, 201u8, 52u8, 145u8, 241u8, 78u8, 39u8,
                126u8, 234u8, 127u8, 170u8, 93u8, 230u8, 162u8, 212u8, 189u8, 167u8,
                90u8, 121u8, 175u8, 122u8, 54u8, 132u8, 251u8, 251u8, 66u8, 218u8, 96u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { status: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.status.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <GameStatus as alloy_sol_types::EventTopic>::encode_topic(
                    &self.status,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Resolved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Resolved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Resolved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `__constructor__((uint32,bytes32,uint256,uint256,uint64,uint64,address,address,address,uint256))` and selector `0x5599c646`.
```solidity
function __constructor__(GameConstructorParams memory _params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __constructor__Call {
        #[allow(missing_docs)]
        pub _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`__constructor__((uint32,bytes32,uint256,uint256,uint64,uint64,address,address,address,uint256))`](__constructor__Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __constructor__Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameConstructorParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameConstructorParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__constructor__Call> for UnderlyingRustTuple<'_> {
                fn from(value: __constructor__Call) -> Self {
                    (value._params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __constructor__Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _params: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__constructor__Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: __constructor__Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __constructor__Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl __constructor__Return {
            fn _tokenize(
                &self,
            ) -> <__constructor__Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __constructor__Call {
            type Parameters<'a> = (GameConstructorParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __constructor__Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__constructor__((uint32,bytes32,uint256,uint256,uint64,uint64,address,address,address,uint256))";
            const SELECTOR: [u8; 4] = [85u8, 153u8, 198u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <GameConstructorParams as alloy_sol_types::SolType>::tokenize(
                        &self._params,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                __constructor__Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `absolutePrestate()` and selector `0x8d450a95`.
```solidity
function absolutePrestate() external view returns (Claim absolutePrestate_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct absolutePrestateCall;
    ///Container type for the return parameters of the [`absolutePrestate()`](absolutePrestateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct absolutePrestateReturn {
        #[allow(missing_docs)]
        pub absolutePrestate_: <Claim as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<absolutePrestateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: absolutePrestateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for absolutePrestateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Claim,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<absolutePrestateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: absolutePrestateReturn) -> Self {
                    (value.absolutePrestate_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for absolutePrestateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { absolutePrestate_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for absolutePrestateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Claim as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Claim,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "absolutePrestate()";
            const SELECTOR: [u8; 4] = [141u8, 69u8, 10u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Claim as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: absolutePrestateReturn = r.into();
                        r.absolutePrestate_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: absolutePrestateReturn = r.into();
                        r.absolutePrestate_
                    })
            }
        }
    };
    /**Function with signature `addLocalData(uint256,uint256,uint256)` and selector `0xf8f43ff6`.
```solidity
function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLocalDataCall {
        #[allow(missing_docs)]
        pub _ident: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _execLeafIdx: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addLocalData(uint256,uint256,uint256)`](addLocalDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLocalDataReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLocalDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLocalDataCall) -> Self {
                    (value._ident, value._execLeafIdx, value._partOffset)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLocalDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ident: tuple.0,
                        _execLeafIdx: tuple.1,
                        _partOffset: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLocalDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLocalDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLocalDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addLocalDataReturn {
            fn _tokenize(
                &self,
            ) -> <addLocalDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLocalDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLocalDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLocalData(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [248u8, 244u8, 63u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._ident),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._execLeafIdx),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addLocalDataReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `anchorStateRegistry()` and selector `0x5c0cba33`.
```solidity
function anchorStateRegistry() external view returns (address registry_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anchorStateRegistryCall;
    ///Container type for the return parameters of the [`anchorStateRegistry()`](anchorStateRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anchorStateRegistryReturn {
        #[allow(missing_docs)]
        pub registry_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<anchorStateRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: anchorStateRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anchorStateRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<anchorStateRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: anchorStateRegistryReturn) -> Self {
                    (value.registry_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anchorStateRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registry_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for anchorStateRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "anchorStateRegistry()";
            const SELECTOR: [u8; 4] = [92u8, 12u8, 186u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: anchorStateRegistryReturn = r.into();
                        r.registry_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: anchorStateRegistryReturn = r.into();
                        r.registry_
                    })
            }
        }
    };
    /**Function with signature `attack(bytes32,uint256,bytes32)` and selector `0x472777c6`.
```solidity
function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attackCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`attack(bytes32,uint256,bytes32)`](attackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attackReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<attackCall> for UnderlyingRustTuple<'_> {
                fn from(value: attackCall) -> Self {
                    (value._disputed, value._parentIndex, value._claim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _parentIndex: tuple.1,
                        _claim: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<attackReturn> for UnderlyingRustTuple<'_> {
                fn from(value: attackReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl attackReturn {
            fn _tokenize(
                &self,
            ) -> <attackCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for attackCall {
            type Parameters<'a> = (Claim, alloy::sol_types::sol_data::Uint<256>, Claim);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = attackReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "attack(bytes32,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [71u8, 39u8, 119u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._parentIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                attackReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `bondDistributionMode()` and selector `0x378dd48c`.
```solidity
function bondDistributionMode() external view returns (BondDistributionMode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bondDistributionModeCall;
    ///Container type for the return parameters of the [`bondDistributionMode()`](bondDistributionModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bondDistributionModeReturn {
        #[allow(missing_docs)]
        pub _0: <BondDistributionMode as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bondDistributionModeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bondDistributionModeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bondDistributionModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (BondDistributionMode,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BondDistributionMode as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bondDistributionModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bondDistributionModeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bondDistributionModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bondDistributionModeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <BondDistributionMode as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (BondDistributionMode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bondDistributionMode()";
            const SELECTOR: [u8; 4] = [55u8, 141u8, 212u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<BondDistributionMode as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: bondDistributionModeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: bondDistributionModeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)` and selector `0x01935130`.
```solidity
function challengeRootL2Block(Types.OutputRootProof memory _outputRootProof, bytes memory _headerRLP) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeRootL2BlockCall {
        #[allow(missing_docs)]
        pub _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _headerRLP: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)`](challengeRootL2BlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeRootL2BlockReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<challengeRootL2BlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeRootL2BlockCall) -> Self {
                    (value._outputRootProof, value._headerRLP)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeRootL2BlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _outputRootProof: tuple.0,
                        _headerRLP: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<challengeRootL2BlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeRootL2BlockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeRootL2BlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl challengeRootL2BlockReturn {
            fn _tokenize(
                &self,
            ) -> <challengeRootL2BlockCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeRootL2BlockCall {
            type Parameters<'a> = (
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeRootL2BlockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [1u8, 147u8, 81u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::OutputRootProof as alloy_sol_types::SolType>::tokenize(
                        &self._outputRootProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._headerRLP,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                challengeRootL2BlockReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `claimCredit(address)` and selector `0x60e27464`.
```solidity
function claimCredit(address _recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCreditCall {
        #[allow(missing_docs)]
        pub _recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`claimCredit(address)`](claimCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCreditReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimCreditCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimCreditCall) -> Self {
                    (value._recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCreditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _recipient: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimCreditReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimCreditReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl claimCreditReturn {
            fn _tokenize(
                &self,
            ) -> <claimCreditCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimCreditReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimCredit(address)";
            const SELECTOR: [u8; 4] = [96u8, 226u8, 116u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimCreditReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `claimData(uint256)` and selector `0xc6f0308c`.
```solidity
function claimData(uint256) external view returns (uint32 parentIndex, address counteredBy, address claimant, uint128 bond, Claim claim, Position position, Clock clock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataCall(pub alloy::sol_types::private::primitives::aliases::U256);
    ///Container type for the return parameters of the [`claimData(uint256)`](claimDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataReturn {
        #[allow(missing_docs)]
        pub parentIndex: u32,
        #[allow(missing_docs)]
        pub counteredBy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub bond: u128,
        #[allow(missing_docs)]
        pub claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub position: <Position as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub clock: <Clock as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                Claim,
                Position,
                Clock,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u128,
                <Claim as alloy::sol_types::SolType>::RustType,
                <Position as alloy::sol_types::SolType>::RustType,
                <Clock as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataReturn) -> Self {
                    (
                        value.parentIndex,
                        value.counteredBy,
                        value.claimant,
                        value.bond,
                        value.claim,
                        value.position,
                        value.clock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        parentIndex: tuple.0,
                        counteredBy: tuple.1,
                        claimant: tuple.2,
                        bond: tuple.3,
                        claim: tuple.4,
                        position: tuple.5,
                        clock: tuple.6,
                    }
                }
            }
        }
        impl claimDataReturn {
            fn _tokenize(
                &self,
            ) -> <claimDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.parentIndex),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.counteredBy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.bond),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.claim),
                    <Position as alloy_sol_types::SolType>::tokenize(&self.position),
                    <Clock as alloy_sol_types::SolType>::tokenize(&self.clock),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                Claim,
                Position,
                Clock,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimData(uint256)";
            const SELECTOR: [u8; 4] = [198u8, 240u8, 48u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimDataReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `claimDataLen()` and selector `0x8980e0cc`.
```solidity
function claimDataLen() external view returns (uint256 len_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataLenCall;
    ///Container type for the return parameters of the [`claimDataLen()`](claimDataLenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataLenReturn {
        #[allow(missing_docs)]
        pub len_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimDataLenCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataLenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataLenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimDataLenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataLenReturn) -> Self {
                    (value.len_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataLenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { len_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDataLenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimDataLen()";
            const SELECTOR: [u8; 4] = [137u8, 128u8, 224u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: claimDataLenReturn = r.into();
                        r.len_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: claimDataLenReturn = r.into();
                        r.len_
                    })
            }
        }
    };
    /**Function with signature `claims(bytes32)` and selector `0xeff0f592`.
```solidity
function claims(Hash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsCall(pub <Hash as alloy::sol_types::SolType>::RustType);
    ///Container type for the return parameters of the [`claims(bytes32)`](claimsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimsCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimsCall {
            type Parameters<'a> = (Hash,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claims(bytes32)";
            const SELECTOR: [u8; 4] = [239u8, 240u8, 245u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Hash as alloy_sol_types::SolType>::tokenize(&self.0),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: claimsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: claimsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `clockExtension()` and selector `0x6b6716c0`.
```solidity
function clockExtension() external view returns (Duration clockExtension_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockExtensionCall;
    ///Container type for the return parameters of the [`clockExtension()`](clockExtensionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockExtensionReturn {
        #[allow(missing_docs)]
        pub clockExtension_: <Duration as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockExtensionCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockExtensionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockExtensionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockExtensionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clockExtensionReturn) -> Self {
                    (value.clockExtension_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clockExtensionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { clockExtension_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockExtensionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clockExtension()";
            const SELECTOR: [u8; 4] = [107u8, 103u8, 22u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: clockExtensionReturn = r.into();
                        r.clockExtension_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: clockExtensionReturn = r.into();
                        r.clockExtension_
                    })
            }
        }
    };
    /**Function with signature `closeGame()` and selector `0x786b844b`.
```solidity
function closeGame() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeGameCall;
    ///Container type for the return parameters of the [`closeGame()`](closeGameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeGameReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeGameCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeGameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeGameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeGameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeGameReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeGameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl closeGameReturn {
            fn _tokenize(
                &self,
            ) -> <closeGameCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeGameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeGameReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeGame()";
            const SELECTOR: [u8; 4] = [120u8, 107u8, 132u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                closeGameReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createdAt()` and selector `0xcf09e0d0`.
```solidity
function createdAt() external view returns (Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createdAtCall;
    ///Container type for the return parameters of the [`createdAt()`](createdAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createdAtReturn {
        #[allow(missing_docs)]
        pub _0: <Timestamp as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createdAtCall> for UnderlyingRustTuple<'_> {
                fn from(value: createdAtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createdAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Timestamp as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createdAtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createdAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createdAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createdAtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createdAt()";
            const SELECTOR: [u8; 4] = [207u8, 9u8, 224u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Timestamp as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createdAtReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: createdAtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `credit(address)` and selector `0xd5d44d80`.
```solidity
function credit(address _recipient) external view returns (uint256 credit_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creditCall {
        #[allow(missing_docs)]
        pub _recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`credit(address)`](creditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creditReturn {
        #[allow(missing_docs)]
        pub credit_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<creditCall> for UnderlyingRustTuple<'_> {
                fn from(value: creditCall) -> Self {
                    (value._recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _recipient: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<creditReturn> for UnderlyingRustTuple<'_> {
                fn from(value: creditReturn) -> Self {
                    (value.credit_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { credit_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for creditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "credit(address)";
            const SELECTOR: [u8; 4] = [213u8, 212u8, 77u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: creditReturn = r.into();
                        r.credit_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: creditReturn = r.into();
                        r.credit_
                    })
            }
        }
    };
    /**Function with signature `defend(bytes32,uint256,bytes32)` and selector `0x7b0f0adc`.
```solidity
function defend(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defendCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`defend(bytes32,uint256,bytes32)`](defendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defendReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defendCall> for UnderlyingRustTuple<'_> {
                fn from(value: defendCall) -> Self {
                    (value._disputed, value._parentIndex, value._claim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defendCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _parentIndex: tuple.1,
                        _claim: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defendReturn> for UnderlyingRustTuple<'_> {
                fn from(value: defendReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defendReturn {
            fn _tokenize(
                &self,
            ) -> <defendCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defendCall {
            type Parameters<'a> = (Claim, alloy::sol_types::sol_data::Uint<256>, Claim);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defendReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defend(bytes32,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [123u8, 15u8, 10u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._parentIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                defendReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `extraData()` and selector `0x609d3334`.
```solidity
function extraData() external pure returns (bytes memory extraData_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extraDataCall;
    ///Container type for the return parameters of the [`extraData()`](extraDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extraDataReturn {
        #[allow(missing_docs)]
        pub extraData_: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extraDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: extraDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extraDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extraDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: extraDataReturn) -> Self {
                    (value.extraData_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extraDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { extraData_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extraDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extraData()";
            const SELECTOR: [u8; 4] = [96u8, 157u8, 51u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: extraDataReturn = r.into();
                        r.extraData_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: extraDataReturn = r.into();
                        r.extraData_
                    })
            }
        }
    };
    /**Function with signature `gameCreator()` and selector `0x37b1b229`.
```solidity
function gameCreator() external pure returns (address creator_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameCreatorCall;
    ///Container type for the return parameters of the [`gameCreator()`](gameCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameCreatorReturn {
        #[allow(missing_docs)]
        pub creator_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameCreatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameCreatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameCreatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameCreatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameCreatorReturn) -> Self {
                    (value.creator_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameCreatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameCreator()";
            const SELECTOR: [u8; 4] = [55u8, 177u8, 178u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gameCreatorReturn = r.into();
                        r.creator_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: gameCreatorReturn = r.into();
                        r.creator_
                    })
            }
        }
    };
    /**Function with signature `gameData()` and selector `0xfa24f743`.
```solidity
function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameDataCall;
    ///Container type for the return parameters of the [`gameData()`](gameDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameDataReturn {
        #[allow(missing_docs)]
        pub gameType_: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub rootClaim_: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub extraData_: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                GameType,
                Claim,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameType as alloy::sol_types::SolType>::RustType,
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameDataReturn) -> Self {
                    (value.gameType_, value.rootClaim_, value.extraData_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        gameType_: tuple.0,
                        rootClaim_: tuple.1,
                        extraData_: tuple.2,
                    }
                }
            }
        }
        impl gameDataReturn {
            fn _tokenize(
                &self,
            ) -> <gameDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <GameType as alloy_sol_types::SolType>::tokenize(&self.gameType_),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.rootClaim_),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData_,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gameDataReturn;
            type ReturnTuple<'a> = (GameType, Claim, alloy::sol_types::sol_data::Bytes);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameData()";
            const SELECTOR: [u8; 4] = [250u8, 36u8, 247u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                gameDataReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `gameType()` and selector `0xbbdc02db`.
```solidity
function gameType() external view returns (GameType gameType_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameTypeCall;
    ///Container type for the return parameters of the [`gameType()`](gameTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameTypeReturn {
        #[allow(missing_docs)]
        pub gameType_: <GameType as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameTypeCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameTypeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gameTypeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameTypeReturn) -> Self {
                    (value.gameType_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { gameType_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameTypeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameType()";
            const SELECTOR: [u8; 4] = [187u8, 220u8, 2u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GameType as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gameTypeReturn = r.into();
                        r.gameType_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: gameTypeReturn = r.into();
                        r.gameType_
                    })
            }
        }
    };
    /**Function with signature `getChallengerDuration(uint256)` and selector `0xbd8da956`.
```solidity
function getChallengerDuration(uint256 _claimIndex) external view returns (Duration duration_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengerDurationCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getChallengerDuration(uint256)`](getChallengerDurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengerDurationReturn {
        #[allow(missing_docs)]
        pub duration_: <Duration as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getChallengerDurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengerDurationCall) -> Self {
                    (value._claimIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengerDurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _claimIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getChallengerDurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengerDurationReturn) -> Self {
                    (value.duration_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengerDurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { duration_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getChallengerDurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getChallengerDuration(uint256)";
            const SELECTOR: [u8; 4] = [189u8, 141u8, 169u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getChallengerDurationReturn = r.into();
                        r.duration_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getChallengerDurationReturn = r.into();
                        r.duration_
                    })
            }
        }
    };
    /**Function with signature `getNumToResolve(uint256)` and selector `0x5a5fa2d9`.
```solidity
function getNumToResolve(uint256 _claimIndex) external view returns (uint256 numRemainingChildren_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumToResolveCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getNumToResolve(uint256)`](getNumToResolveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumToResolveReturn {
        #[allow(missing_docs)]
        pub numRemainingChildren_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getNumToResolveCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNumToResolveCall) -> Self {
                    (value._claimIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNumToResolveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _claimIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getNumToResolveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumToResolveReturn) -> Self {
                    (value.numRemainingChildren_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumToResolveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        numRemainingChildren_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNumToResolveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNumToResolve(uint256)";
            const SELECTOR: [u8; 4] = [90u8, 95u8, 162u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getNumToResolveReturn = r.into();
                        r.numRemainingChildren_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getNumToResolveReturn = r.into();
                        r.numRemainingChildren_
                    })
            }
        }
    };
    /**Function with signature `getRequiredBond(uint128)` and selector `0xc395e1ca`.
```solidity
function getRequiredBond(Position _position) external view returns (uint256 requiredBond_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequiredBondCall {
        #[allow(missing_docs)]
        pub _position: <Position as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getRequiredBond(uint128)`](getRequiredBondCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequiredBondReturn {
        #[allow(missing_docs)]
        pub requiredBond_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Position,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Position as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRequiredBondCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRequiredBondCall) -> Self {
                    (value._position,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequiredBondCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _position: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRequiredBondReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRequiredBondReturn) -> Self {
                    (value.requiredBond_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRequiredBondReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requiredBond_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRequiredBondCall {
            type Parameters<'a> = (Position,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRequiredBond(uint128)";
            const SELECTOR: [u8; 4] = [195u8, 149u8, 225u8, 202u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Position as alloy_sol_types::SolType>::tokenize(&self._position),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRequiredBondReturn = r.into();
                        r.requiredBond_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRequiredBondReturn = r.into();
                        r.requiredBond_
                    })
            }
        }
    };
    /**Function with signature `hasUnlockedCredit(address)` and selector `0x222abf45`.
```solidity
function hasUnlockedCredit(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasUnlockedCreditCall(pub alloy::sol_types::private::Address);
    ///Container type for the return parameters of the [`hasUnlockedCredit(address)`](hasUnlockedCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasUnlockedCreditReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasUnlockedCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasUnlockedCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasUnlockedCreditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasUnlockedCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasUnlockedCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasUnlockedCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasUnlockedCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasUnlockedCredit(address)";
            const SELECTOR: [u8; 4] = [34u8, 42u8, 191u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: hasUnlockedCreditReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: hasUnlockedCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `initialize()` and selector `0x8129fc1c`.
```solidity
function initialize() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall;
    ///Container type for the return parameters of the [`initialize()`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize()";
            const SELECTOR: [u8; 4] = [129u8, 41u8, 252u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `l1Head()` and selector `0x6361506d`.
```solidity
function l1Head() external pure returns (Hash l1Head_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1HeadCall;
    ///Container type for the return parameters of the [`l1Head()`](l1HeadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1HeadReturn {
        #[allow(missing_docs)]
        pub l1Head_: <Hash as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1HeadCall> for UnderlyingRustTuple<'_> {
                fn from(value: l1HeadCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l1HeadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1HeadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l1HeadReturn) -> Self {
                    (value.l1Head_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l1HeadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l1Head_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l1HeadCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Hash as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Hash,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l1Head()";
            const SELECTOR: [u8; 4] = [99u8, 97u8, 80u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Hash as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l1HeadReturn = r.into();
                        r.l1Head_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l1HeadReturn = r.into();
                        r.l1Head_
                    })
            }
        }
    };
    /**Function with signature `l2BlockNumber()` and selector `0x8b85902b`.
```solidity
function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberCall;
    ///Container type for the return parameters of the [`l2BlockNumber()`](l2BlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberReturn {
        #[allow(missing_docs)]
        pub l2BlockNumber_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberReturn) -> Self {
                    (value.l2BlockNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2BlockNumber_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumber()";
            const SELECTOR: [u8; 4] = [139u8, 133u8, 144u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2BlockNumberReturn = r.into();
                        r.l2BlockNumber_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2BlockNumberReturn = r.into();
                        r.l2BlockNumber_
                    })
            }
        }
    };
    /**Function with signature `l2BlockNumberChallenged()` and selector `0x3e3ac912`.
```solidity
function l2BlockNumberChallenged() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengedCall;
    ///Container type for the return parameters of the [`l2BlockNumberChallenged()`](l2BlockNumberChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberChallengedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberChallengedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberChallengedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumberChallenged()";
            const SELECTOR: [u8; 4] = [62u8, 58u8, 201u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2BlockNumberChallengedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2BlockNumberChallengedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `l2BlockNumberChallenger()` and selector `0x30dbe570`.
```solidity
function l2BlockNumberChallenger() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengerCall;
    ///Container type for the return parameters of the [`l2BlockNumberChallenger()`](l2BlockNumberChallengerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberChallengerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2BlockNumberChallengerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberChallengerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumberChallenger()";
            const SELECTOR: [u8; 4] = [48u8, 219u8, 229u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2BlockNumberChallengerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2BlockNumberChallengerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `l2ChainId()` and selector `0xd6ae3cd5`.
```solidity
function l2ChainId() external view returns (uint256 l2ChainId_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2ChainIdCall;
    ///Container type for the return parameters of the [`l2ChainId()`](l2ChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2ChainIdReturn {
        #[allow(missing_docs)]
        pub l2ChainId_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2ChainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2ChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2ChainIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2ChainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2ChainIdReturn) -> Self {
                    (value.l2ChainId_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2ChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2ChainId_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2ChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2ChainId()";
            const SELECTOR: [u8; 4] = [214u8, 174u8, 60u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2ChainIdReturn = r.into();
                        r.l2ChainId_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2ChainIdReturn = r.into();
                        r.l2ChainId_
                    })
            }
        }
    };
    /**Function with signature `l2SequenceNumber()` and selector `0x99735e32`.
```solidity
function l2SequenceNumber() external pure returns (uint256 l2SequenceNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SequenceNumberCall;
    ///Container type for the return parameters of the [`l2SequenceNumber()`](l2SequenceNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SequenceNumberReturn {
        #[allow(missing_docs)]
        pub l2SequenceNumber_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2SequenceNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2SequenceNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2SequenceNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2SequenceNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2SequenceNumberReturn) -> Self {
                    (value.l2SequenceNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2SequenceNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2SequenceNumber_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2SequenceNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2SequenceNumber()";
            const SELECTOR: [u8; 4] = [153u8, 115u8, 94u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2SequenceNumberReturn = r.into();
                        r.l2SequenceNumber_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2SequenceNumberReturn = r.into();
                        r.l2SequenceNumber_
                    })
            }
        }
    };
    /**Function with signature `maxClockDuration()` and selector `0xdabd396d`.
```solidity
function maxClockDuration() external view returns (Duration maxClockDuration_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxClockDurationCall;
    ///Container type for the return parameters of the [`maxClockDuration()`](maxClockDurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxClockDurationReturn {
        #[allow(missing_docs)]
        pub maxClockDuration_: <Duration as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxClockDurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxClockDurationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxClockDurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxClockDurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxClockDurationReturn) -> Self {
                    (value.maxClockDuration_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxClockDurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { maxClockDuration_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxClockDurationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxClockDuration()";
            const SELECTOR: [u8; 4] = [218u8, 189u8, 57u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maxClockDurationReturn = r.into();
                        r.maxClockDuration_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: maxClockDurationReturn = r.into();
                        r.maxClockDuration_
                    })
            }
        }
    };
    /**Function with signature `maxGameDepth()` and selector `0xfa315aa9`.
```solidity
function maxGameDepth() external view returns (uint256 maxGameDepth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxGameDepthCall;
    ///Container type for the return parameters of the [`maxGameDepth()`](maxGameDepthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxGameDepthReturn {
        #[allow(missing_docs)]
        pub maxGameDepth_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxGameDepthCall> for UnderlyingRustTuple<'_> {
                fn from(value: maxGameDepthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxGameDepthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxGameDepthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: maxGameDepthReturn) -> Self {
                    (value.maxGameDepth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxGameDepthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { maxGameDepth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxGameDepthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxGameDepth()";
            const SELECTOR: [u8; 4] = [250u8, 49u8, 90u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maxGameDepthReturn = r.into();
                        r.maxGameDepth_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: maxGameDepthReturn = r.into();
                        r.maxGameDepth_
                    })
            }
        }
    };
    /**Function with signature `move(bytes32,uint256,bytes32,bool)` and selector `0x6f034409`.
```solidity
function r#move(Claim _disputed, uint256 _challengeIndex, Claim _claim, bool _isAttack) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moveCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _challengeIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _isAttack: bool,
    }
    ///Container type for the return parameters of the [`move(bytes32,uint256,bytes32,bool)`](moveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moveReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<moveCall> for UnderlyingRustTuple<'_> {
                fn from(value: moveCall) -> Self {
                    (
                        value._disputed,
                        value._challengeIndex,
                        value._claim,
                        value._isAttack,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _challengeIndex: tuple.1,
                        _claim: tuple.2,
                        _isAttack: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<moveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: moveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl moveReturn {
            fn _tokenize(
                &self,
            ) -> <moveCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for moveCall {
            type Parameters<'a> = (
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = moveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "move(bytes32,uint256,bytes32,bool)";
            const SELECTOR: [u8; 4] = [111u8, 3u8, 68u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._challengeIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isAttack,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                moveReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `normalModeCredit(address)` and selector `0x529d6a8c`.
```solidity
function normalModeCredit(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct normalModeCreditCall(pub alloy::sol_types::private::Address);
    ///Container type for the return parameters of the [`normalModeCredit(address)`](normalModeCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct normalModeCreditReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<normalModeCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: normalModeCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for normalModeCreditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<normalModeCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: normalModeCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for normalModeCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for normalModeCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "normalModeCredit(address)";
            const SELECTOR: [u8; 4] = [82u8, 157u8, 106u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: normalModeCreditReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: normalModeCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `refundModeCredit(address)` and selector `0xc0d8bb74`.
```solidity
function refundModeCredit(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct refundModeCreditCall(pub alloy::sol_types::private::Address);
    ///Container type for the return parameters of the [`refundModeCredit(address)`](refundModeCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct refundModeCreditReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<refundModeCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: refundModeCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for refundModeCreditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<refundModeCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: refundModeCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for refundModeCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for refundModeCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "refundModeCredit(address)";
            const SELECTOR: [u8; 4] = [192u8, 216u8, 187u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: refundModeCreditReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: refundModeCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `resolutionCheckpoints(uint256)` and selector `0xa445ece6`.
```solidity
function resolutionCheckpoints(uint256) external view returns (bool initialCheckpointComplete, uint32 subgameIndex, Position leftmostPosition, address counteredBy);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolutionCheckpointsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    ///Container type for the return parameters of the [`resolutionCheckpoints(uint256)`](resolutionCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolutionCheckpointsReturn {
        #[allow(missing_docs)]
        pub initialCheckpointComplete: bool,
        #[allow(missing_docs)]
        pub subgameIndex: u32,
        #[allow(missing_docs)]
        pub leftmostPosition: <Position as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub counteredBy: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolutionCheckpointsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolutionCheckpointsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolutionCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
                Position,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                u32,
                <Position as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolutionCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolutionCheckpointsReturn) -> Self {
                    (
                        value.initialCheckpointComplete,
                        value.subgameIndex,
                        value.leftmostPosition,
                        value.counteredBy,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolutionCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialCheckpointComplete: tuple.0,
                        subgameIndex: tuple.1,
                        leftmostPosition: tuple.2,
                        counteredBy: tuple.3,
                    }
                }
            }
        }
        impl resolutionCheckpointsReturn {
            fn _tokenize(
                &self,
            ) -> <resolutionCheckpointsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.initialCheckpointComplete,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.subgameIndex),
                    <Position as alloy_sol_types::SolType>::tokenize(
                        &self.leftmostPosition,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.counteredBy,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolutionCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resolutionCheckpointsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
                Position,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolutionCheckpoints(uint256)";
            const SELECTOR: [u8; 4] = [164u8, 69u8, 236u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resolutionCheckpointsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `resolve()` and selector `0x2810e1d6`.
```solidity
function resolve() external returns (GameStatus status_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveCall;
    ///Container type for the return parameters of the [`resolve()`](resolveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveReturn {
        #[allow(missing_docs)]
        pub status_: <GameStatus as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolveCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameStatus as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolveReturn) -> Self {
                    (value.status_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { status_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameStatus as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolve()";
            const SELECTOR: [u8; 4] = [40u8, 16u8, 225u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GameStatus as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resolveReturn = r.into();
                        r.status_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: resolveReturn = r.into();
                        r.status_
                    })
            }
        }
    };
    /**Function with signature `resolveClaim(uint256,uint256)` and selector `0x03c2924d`.
```solidity
function resolveClaim(uint256 _claimIndex, uint256 _numToResolve) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveClaimCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _numToResolve: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resolveClaim(uint256,uint256)`](resolveClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveClaimReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolveClaimCall) -> Self {
                    (value._claimIndex, value._numToResolve)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimIndex: tuple.0,
                        _numToResolve: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolveClaimReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resolveClaimReturn {
            fn _tokenize(
                &self,
            ) -> <resolveClaimCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolveClaimCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resolveClaimReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolveClaim(uint256,uint256)";
            const SELECTOR: [u8; 4] = [3u8, 194u8, 146u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._numToResolve),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resolveClaimReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `resolvedAt()` and selector `0x19effeb4`.
```solidity
function resolvedAt() external view returns (Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedAtCall;
    ///Container type for the return parameters of the [`resolvedAt()`](resolvedAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedAtReturn {
        #[allow(missing_docs)]
        pub _0: <Timestamp as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolvedAtCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolvedAtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolvedAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Timestamp as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolvedAtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolvedAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolvedAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolvedAtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolvedAt()";
            const SELECTOR: [u8; 4] = [25u8, 239u8, 254u8, 180u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Timestamp as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resolvedAtReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: resolvedAtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `resolvedSubgames(uint256)` and selector `0xfe2bbeb2`.
```solidity
function resolvedSubgames(uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedSubgamesCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    ///Container type for the return parameters of the [`resolvedSubgames(uint256)`](resolvedSubgamesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedSubgamesReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolvedSubgamesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolvedSubgamesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolvedSubgamesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolvedSubgamesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolvedSubgamesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolvedSubgamesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolvedSubgamesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolvedSubgames(uint256)";
            const SELECTOR: [u8; 4] = [254u8, 43u8, 190u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resolvedSubgamesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: resolvedSubgamesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `rootClaim()` and selector `0xbcef3b55`.
```solidity
function rootClaim() external pure returns (Claim rootClaim_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootClaimCall;
    ///Container type for the return parameters of the [`rootClaim()`](rootClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootClaimReturn {
        #[allow(missing_docs)]
        pub rootClaim_: <Claim as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rootClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: rootClaimCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Claim,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rootClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rootClaimReturn) -> Self {
                    (value.rootClaim_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rootClaim_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rootClaimCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Claim as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Claim,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rootClaim()";
            const SELECTOR: [u8; 4] = [188u8, 239u8, 59u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Claim as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: rootClaimReturn = r.into();
                        r.rootClaim_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: rootClaimReturn = r.into();
                        r.rootClaim_
                    })
            }
        }
    };
    /**Function with signature `splitDepth()` and selector `0xec5e6308`.
```solidity
function splitDepth() external view returns (uint256 splitDepth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct splitDepthCall;
    ///Container type for the return parameters of the [`splitDepth()`](splitDepthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct splitDepthReturn {
        #[allow(missing_docs)]
        pub splitDepth_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<splitDepthCall> for UnderlyingRustTuple<'_> {
                fn from(value: splitDepthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for splitDepthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<splitDepthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: splitDepthReturn) -> Self {
                    (value.splitDepth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for splitDepthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { splitDepth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for splitDepthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "splitDepth()";
            const SELECTOR: [u8; 4] = [236u8, 94u8, 99u8, 8u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: splitDepthReturn = r.into();
                        r.splitDepth_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: splitDepthReturn = r.into();
                        r.splitDepth_
                    })
            }
        }
    };
    /**Function with signature `startingBlockNumber()` and selector `0x70872aa5`.
```solidity
function startingBlockNumber() external view returns (uint256 startingBlockNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberCall;
    ///Container type for the return parameters of the [`startingBlockNumber()`](startingBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberReturn {
        #[allow(missing_docs)]
        pub startingBlockNumber_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingBlockNumberReturn) -> Self {
                    (value.startingBlockNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startingBlockNumber_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingBlockNumber()";
            const SELECTOR: [u8; 4] = [112u8, 135u8, 42u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: startingBlockNumberReturn = r.into();
                        r.startingBlockNumber_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: startingBlockNumberReturn = r.into();
                        r.startingBlockNumber_
                    })
            }
        }
    };
    /**Function with signature `startingOutputRoot()` and selector `0x57da950e`.
```solidity
function startingOutputRoot() external view returns (Hash root, uint256 l2SequenceNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingOutputRootCall;
    ///Container type for the return parameters of the [`startingOutputRoot()`](startingOutputRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingOutputRootReturn {
        #[allow(missing_docs)]
        pub root: <Hash as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub l2SequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingOutputRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingOutputRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingOutputRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingOutputRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingOutputRootReturn) -> Self {
                    (value.root, value.l2SequenceNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingOutputRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        root: tuple.0,
                        l2SequenceNumber: tuple.1,
                    }
                }
            }
        }
        impl startingOutputRootReturn {
            fn _tokenize(
                &self,
            ) -> <startingOutputRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <Hash as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2SequenceNumber),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingOutputRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startingOutputRootReturn;
            type ReturnTuple<'a> = (Hash, alloy::sol_types::sol_data::Uint<256>);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingOutputRoot()";
            const SELECTOR: [u8; 4] = [87u8, 218u8, 149u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                startingOutputRootReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `startingRootHash()` and selector `0x25fc2ace`.
```solidity
function startingRootHash() external view returns (Hash startingRootHash_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingRootHashCall;
    ///Container type for the return parameters of the [`startingRootHash()`](startingRootHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingRootHashReturn {
        #[allow(missing_docs)]
        pub startingRootHash_: <Hash as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingRootHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingRootHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingRootHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startingRootHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingRootHashReturn) -> Self {
                    (value.startingRootHash_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingRootHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { startingRootHash_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingRootHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Hash as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Hash,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingRootHash()";
            const SELECTOR: [u8; 4] = [37u8, 252u8, 42u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Hash as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: startingRootHashReturn = r.into();
                        r.startingRootHash_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: startingRootHashReturn = r.into();
                        r.startingRootHash_
                    })
            }
        }
    };
    /**Function with signature `status()` and selector `0x200d2ed2`.
```solidity
function status() external view returns (GameStatus);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct statusCall;
    ///Container type for the return parameters of the [`status()`](statusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct statusReturn {
        #[allow(missing_docs)]
        pub _0: <GameStatus as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<statusCall> for UnderlyingRustTuple<'_> {
                fn from(value: statusCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for statusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameStatus as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<statusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: statusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for statusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for statusCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameStatus as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "status()";
            const SELECTOR: [u8; 4] = [32u8, 13u8, 46u8, 210u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GameStatus as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: statusReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: statusReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `step(uint256,bool,bytes,bytes)` and selector `0xd8cc1a3c`.
```solidity
function step(uint256 _claimIndex, bool _isAttack, bytes memory _stateData, bytes memory _proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stepCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _isAttack: bool,
        #[allow(missing_docs)]
        pub _stateData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _proof: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`step(uint256,bool,bytes,bytes)`](stepCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stepReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stepCall> for UnderlyingRustTuple<'_> {
                fn from(value: stepCall) -> Self {
                    (value._claimIndex, value._isAttack, value._stateData, value._proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stepCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimIndex: tuple.0,
                        _isAttack: tuple.1,
                        _stateData: tuple.2,
                        _proof: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stepReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stepReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stepReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl stepReturn {
            fn _tokenize(
                &self,
            ) -> <stepCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stepCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stepReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "step(uint256,bool,bytes,bytes)";
            const SELECTOR: [u8; 4] = [216u8, 204u8, 26u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isAttack,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._stateData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._proof,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                stepReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `subgames(uint256,uint256)` and selector `0x2ad69aeb`.
```solidity
function subgames(uint256, uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subgamesCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`subgames(uint256,uint256)`](subgamesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subgamesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<subgamesCall> for UnderlyingRustTuple<'_> {
                fn from(value: subgamesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for subgamesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<subgamesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: subgamesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for subgamesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for subgamesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "subgames(uint256,uint256)";
            const SELECTOR: [u8; 4] = [42u8, 214u8, 154u8, 235u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: subgamesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: subgamesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionCall;
    ///Container type for the return parameters of the [`version()`](versionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "version()";
            const SELECTOR: [u8; 4] = [84u8, 253u8, 77u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: versionReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: versionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `vm()` and selector `0x3a768463`.
```solidity
function vm() external view returns (address vm_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vmCall;
    ///Container type for the return parameters of the [`vm()`](vmCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vmReturn {
        #[allow(missing_docs)]
        pub vm_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<vmCall> for UnderlyingRustTuple<'_> {
                fn from(value: vmCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vmCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<vmReturn> for UnderlyingRustTuple<'_> {
                fn from(value: vmReturn) -> Self {
                    (value.vm_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vmReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { vm_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for vmCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "vm()";
            const SELECTOR: [u8; 4] = [58u8, 118u8, 132u8, 99u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: vmReturn = r.into();
                        r.vm_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: vmReturn = r.into();
                        r.vm_
                    })
            }
        }
    };
    /**Function with signature `wasRespectedGameTypeWhenCreated()` and selector `0x250e69bd`.
```solidity
function wasRespectedGameTypeWhenCreated() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasRespectedGameTypeWhenCreatedCall;
    ///Container type for the return parameters of the [`wasRespectedGameTypeWhenCreated()`](wasRespectedGameTypeWhenCreatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasRespectedGameTypeWhenCreatedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wasRespectedGameTypeWhenCreatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: wasRespectedGameTypeWhenCreatedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for wasRespectedGameTypeWhenCreatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wasRespectedGameTypeWhenCreatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: wasRespectedGameTypeWhenCreatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for wasRespectedGameTypeWhenCreatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wasRespectedGameTypeWhenCreatedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "wasRespectedGameTypeWhenCreated()";
            const SELECTOR: [u8; 4] = [37u8, 14u8, 105u8, 189u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: wasRespectedGameTypeWhenCreatedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: wasRespectedGameTypeWhenCreatedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `weth()` and selector `0x3fc8cef3`.
```solidity
function weth() external view returns (address weth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethCall;
    ///Container type for the return parameters of the [`weth()`](wethCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethReturn {
        #[allow(missing_docs)]
        pub weth_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethCall> for UnderlyingRustTuple<'_> {
                fn from(value: wethCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethReturn> for UnderlyingRustTuple<'_> {
                fn from(value: wethReturn) -> Self {
                    (value.weth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { weth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wethCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weth()";
            const SELECTOR: [u8; 4] = [63u8, 200u8, 206u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: wethReturn = r.into();
                        r.weth_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: wethReturn = r.into();
                        r.weth_
                    })
            }
        }
    };
    ///Container for all the [`IFaultDisputeGame`](self) function calls.
    #[derive(Clone)]
    pub enum IFaultDisputeGameCalls {
        #[allow(missing_docs)]
        __constructor__(__constructor__Call),
        #[allow(missing_docs)]
        absolutePrestate(absolutePrestateCall),
        #[allow(missing_docs)]
        addLocalData(addLocalDataCall),
        #[allow(missing_docs)]
        anchorStateRegistry(anchorStateRegistryCall),
        #[allow(missing_docs)]
        attack(attackCall),
        #[allow(missing_docs)]
        bondDistributionMode(bondDistributionModeCall),
        #[allow(missing_docs)]
        challengeRootL2Block(challengeRootL2BlockCall),
        #[allow(missing_docs)]
        claimCredit(claimCreditCall),
        #[allow(missing_docs)]
        claimData(claimDataCall),
        #[allow(missing_docs)]
        claimDataLen(claimDataLenCall),
        #[allow(missing_docs)]
        claims(claimsCall),
        #[allow(missing_docs)]
        clockExtension(clockExtensionCall),
        #[allow(missing_docs)]
        closeGame(closeGameCall),
        #[allow(missing_docs)]
        createdAt(createdAtCall),
        #[allow(missing_docs)]
        credit(creditCall),
        #[allow(missing_docs)]
        defend(defendCall),
        #[allow(missing_docs)]
        extraData(extraDataCall),
        #[allow(missing_docs)]
        gameCreator(gameCreatorCall),
        #[allow(missing_docs)]
        gameData(gameDataCall),
        #[allow(missing_docs)]
        gameType(gameTypeCall),
        #[allow(missing_docs)]
        getChallengerDuration(getChallengerDurationCall),
        #[allow(missing_docs)]
        getNumToResolve(getNumToResolveCall),
        #[allow(missing_docs)]
        getRequiredBond(getRequiredBondCall),
        #[allow(missing_docs)]
        hasUnlockedCredit(hasUnlockedCreditCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        l1Head(l1HeadCall),
        #[allow(missing_docs)]
        l2BlockNumber(l2BlockNumberCall),
        #[allow(missing_docs)]
        l2BlockNumberChallenged(l2BlockNumberChallengedCall),
        #[allow(missing_docs)]
        l2BlockNumberChallenger(l2BlockNumberChallengerCall),
        #[allow(missing_docs)]
        l2ChainId(l2ChainIdCall),
        #[allow(missing_docs)]
        l2SequenceNumber(l2SequenceNumberCall),
        #[allow(missing_docs)]
        maxClockDuration(maxClockDurationCall),
        #[allow(missing_docs)]
        maxGameDepth(maxGameDepthCall),
        #[allow(missing_docs)]
        r#move(moveCall),
        #[allow(missing_docs)]
        normalModeCredit(normalModeCreditCall),
        #[allow(missing_docs)]
        refundModeCredit(refundModeCreditCall),
        #[allow(missing_docs)]
        resolutionCheckpoints(resolutionCheckpointsCall),
        #[allow(missing_docs)]
        resolve(resolveCall),
        #[allow(missing_docs)]
        resolveClaim(resolveClaimCall),
        #[allow(missing_docs)]
        resolvedAt(resolvedAtCall),
        #[allow(missing_docs)]
        resolvedSubgames(resolvedSubgamesCall),
        #[allow(missing_docs)]
        rootClaim(rootClaimCall),
        #[allow(missing_docs)]
        splitDepth(splitDepthCall),
        #[allow(missing_docs)]
        startingBlockNumber(startingBlockNumberCall),
        #[allow(missing_docs)]
        startingOutputRoot(startingOutputRootCall),
        #[allow(missing_docs)]
        startingRootHash(startingRootHashCall),
        #[allow(missing_docs)]
        status(statusCall),
        #[allow(missing_docs)]
        step(stepCall),
        #[allow(missing_docs)]
        subgames(subgamesCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        vm(vmCall),
        #[allow(missing_docs)]
        wasRespectedGameTypeWhenCreated(wasRespectedGameTypeWhenCreatedCall),
        #[allow(missing_docs)]
        weth(wethCall),
    }
    impl IFaultDisputeGameCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 147u8, 81u8, 48u8],
            [3u8, 194u8, 146u8, 77u8],
            [25u8, 239u8, 254u8, 180u8],
            [32u8, 13u8, 46u8, 210u8],
            [34u8, 42u8, 191u8, 69u8],
            [37u8, 14u8, 105u8, 189u8],
            [37u8, 252u8, 42u8, 206u8],
            [40u8, 16u8, 225u8, 214u8],
            [42u8, 214u8, 154u8, 235u8],
            [48u8, 219u8, 229u8, 112u8],
            [55u8, 141u8, 212u8, 140u8],
            [55u8, 177u8, 178u8, 41u8],
            [58u8, 118u8, 132u8, 99u8],
            [62u8, 58u8, 201u8, 18u8],
            [63u8, 200u8, 206u8, 243u8],
            [71u8, 39u8, 119u8, 198u8],
            [82u8, 157u8, 106u8, 140u8],
            [84u8, 253u8, 77u8, 80u8],
            [85u8, 153u8, 198u8, 70u8],
            [87u8, 218u8, 149u8, 14u8],
            [90u8, 95u8, 162u8, 217u8],
            [92u8, 12u8, 186u8, 51u8],
            [96u8, 157u8, 51u8, 52u8],
            [96u8, 226u8, 116u8, 100u8],
            [99u8, 97u8, 80u8, 109u8],
            [107u8, 103u8, 22u8, 192u8],
            [111u8, 3u8, 68u8, 9u8],
            [112u8, 135u8, 42u8, 165u8],
            [120u8, 107u8, 132u8, 75u8],
            [123u8, 15u8, 10u8, 220u8],
            [129u8, 41u8, 252u8, 28u8],
            [137u8, 128u8, 224u8, 204u8],
            [139u8, 133u8, 144u8, 43u8],
            [141u8, 69u8, 10u8, 149u8],
            [153u8, 115u8, 94u8, 50u8],
            [164u8, 69u8, 236u8, 230u8],
            [187u8, 220u8, 2u8, 219u8],
            [188u8, 239u8, 59u8, 85u8],
            [189u8, 141u8, 169u8, 86u8],
            [192u8, 216u8, 187u8, 116u8],
            [195u8, 149u8, 225u8, 202u8],
            [198u8, 240u8, 48u8, 140u8],
            [207u8, 9u8, 224u8, 208u8],
            [213u8, 212u8, 77u8, 128u8],
            [214u8, 174u8, 60u8, 213u8],
            [216u8, 204u8, 26u8, 60u8],
            [218u8, 189u8, 57u8, 109u8],
            [236u8, 94u8, 99u8, 8u8],
            [239u8, 240u8, 245u8, 146u8],
            [248u8, 244u8, 63u8, 246u8],
            [250u8, 36u8, 247u8, 67u8],
            [250u8, 49u8, 90u8, 169u8],
            [254u8, 43u8, 190u8, 178u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(challengeRootL2Block),
            ::core::stringify!(resolveClaim),
            ::core::stringify!(resolvedAt),
            ::core::stringify!(status),
            ::core::stringify!(hasUnlockedCredit),
            ::core::stringify!(wasRespectedGameTypeWhenCreated),
            ::core::stringify!(startingRootHash),
            ::core::stringify!(resolve),
            ::core::stringify!(subgames),
            ::core::stringify!(l2BlockNumberChallenger),
            ::core::stringify!(bondDistributionMode),
            ::core::stringify!(gameCreator),
            ::core::stringify!(vm),
            ::core::stringify!(l2BlockNumberChallenged),
            ::core::stringify!(weth),
            ::core::stringify!(attack),
            ::core::stringify!(normalModeCredit),
            ::core::stringify!(version),
            ::core::stringify!(__constructor__),
            ::core::stringify!(startingOutputRoot),
            ::core::stringify!(getNumToResolve),
            ::core::stringify!(anchorStateRegistry),
            ::core::stringify!(extraData),
            ::core::stringify!(claimCredit),
            ::core::stringify!(l1Head),
            ::core::stringify!(clockExtension),
            ::core::stringify!(r#move),
            ::core::stringify!(startingBlockNumber),
            ::core::stringify!(closeGame),
            ::core::stringify!(defend),
            ::core::stringify!(initialize),
            ::core::stringify!(claimDataLen),
            ::core::stringify!(l2BlockNumber),
            ::core::stringify!(absolutePrestate),
            ::core::stringify!(l2SequenceNumber),
            ::core::stringify!(resolutionCheckpoints),
            ::core::stringify!(gameType),
            ::core::stringify!(rootClaim),
            ::core::stringify!(getChallengerDuration),
            ::core::stringify!(refundModeCredit),
            ::core::stringify!(getRequiredBond),
            ::core::stringify!(claimData),
            ::core::stringify!(createdAt),
            ::core::stringify!(credit),
            ::core::stringify!(l2ChainId),
            ::core::stringify!(step),
            ::core::stringify!(maxClockDuration),
            ::core::stringify!(splitDepth),
            ::core::stringify!(claims),
            ::core::stringify!(addLocalData),
            ::core::stringify!(gameData),
            ::core::stringify!(maxGameDepth),
            ::core::stringify!(resolvedSubgames),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <challengeRootL2BlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolvedAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <statusCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasUnlockedCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingRootHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subgamesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bondDistributionModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameCreatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <vmCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wethCall as alloy_sol_types::SolCall>::SIGNATURE,
            <attackCall as alloy_sol_types::SolCall>::SIGNATURE,
            <normalModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <__constructor__Call as alloy_sol_types::SolCall>::SIGNATURE,
            <startingOutputRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getNumToResolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <anchorStateRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <extraDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1HeadCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clockExtensionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <moveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingBlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <closeGameCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataLenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <absolutePrestateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2SequenceNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolutionCheckpointsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <rootClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getChallengerDurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <refundModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRequiredBondCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createdAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2ChainIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <stepCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxClockDurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <splitDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addLocalDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxGameDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolvedSubgamesCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IFaultDisputeGameCalls {
        const NAME: &'static str = "IFaultDisputeGameCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 53usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::__constructor__(_) => {
                    <__constructor__Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::absolutePrestate(_) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addLocalData(_) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::anchorStateRegistry(_) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::attack(_) => <attackCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bondDistributionMode(_) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeRootL2Block(_) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimCredit(_) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimData(_) => {
                    <claimDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimDataLen(_) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claims(_) => <claimsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::clockExtension(_) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeGame(_) => {
                    <closeGameCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createdAt(_) => {
                    <createdAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::credit(_) => <creditCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::defend(_) => <defendCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extraData(_) => {
                    <extraDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gameCreator(_) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gameData(_) => <gameDataCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::gameType(_) => <gameTypeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getChallengerDuration(_) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNumToResolve(_) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRequiredBond(_) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasUnlockedCredit(_) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l1Head(_) => <l1HeadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::l2BlockNumber(_) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2BlockNumberChallenged(_) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2BlockNumberChallenger(_) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2ChainId(_) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2SequenceNumber(_) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxClockDuration(_) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxGameDepth(_) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::r#move(_) => <moveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::normalModeCredit(_) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::refundModeCredit(_) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolutionCheckpoints(_) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolve(_) => <resolveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::resolveClaim(_) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolvedAt(_) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolvedSubgames(_) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rootClaim(_) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::splitDepth(_) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingBlockNumber(_) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingOutputRoot(_) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingRootHash(_) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::status(_) => <statusCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::step(_) => <stepCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::subgames(_) => <subgamesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::vm(_) => <vmCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::wasRespectedGameTypeWhenCreated(_) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::weth(_) => <wethCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<IFaultDisputeGameCalls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::wasRespectedGameTypeWhenCreated)
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn __constructor__(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <__constructor__Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::__constructor__)
                    }
                    __constructor__
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IFaultDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolvedSubgames)
                    }
                    resolvedSubgames
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<IFaultDisputeGameCalls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::wasRespectedGameTypeWhenCreated)
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn __constructor__(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <__constructor__Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::__constructor__)
                    }
                    __constructor__
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameCalls::resolvedSubgames)
                    }
                    resolvedSubgames
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::__constructor__(inner) => {
                    <__constructor__Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::absolutePrestate(inner) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addLocalData(inner) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::attack(inner) => {
                    <attackCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bondDistributionMode(inner) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengeRootL2Block(inner) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimCredit(inner) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimData(inner) => {
                    <claimDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::claimDataLen(inner) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claims(inner) => {
                    <claimsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::clockExtension(inner) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeGame(inner) => {
                    <closeGameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::createdAt(inner) => {
                    <createdAtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::credit(inner) => {
                    <creditCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::defend(inner) => {
                    <defendCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extraData(inner) => {
                    <extraDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::gameCreator(inner) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gameData(inner) => {
                    <gameDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::gameType(inner) => {
                    <gameTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getChallengerDuration(inner) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNumToResolve(inner) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRequiredBond(inner) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hasUnlockedCredit(inner) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l1Head(inner) => {
                    <l1HeadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l2BlockNumber(inner) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2BlockNumberChallenged(inner) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2BlockNumberChallenger(inner) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2ChainId(inner) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l2SequenceNumber(inner) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxClockDuration(inner) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxGameDepth(inner) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::r#move(inner) => {
                    <moveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::normalModeCredit(inner) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::refundModeCredit(inner) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolutionCheckpoints(inner) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolve(inner) => {
                    <resolveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::resolveClaim(inner) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolvedAt(inner) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::resolvedSubgames(inner) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rootClaim(inner) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::splitDepth(inner) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startingBlockNumber(inner) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingOutputRoot(inner) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingRootHash(inner) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::status(inner) => {
                    <statusCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::step(inner) => {
                    <stepCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::subgames(inner) => {
                    <subgamesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::vm(inner) => {
                    <vmCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::wasRespectedGameTypeWhenCreated(inner) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::__constructor__(inner) => {
                    <__constructor__Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::absolutePrestate(inner) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addLocalData(inner) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::attack(inner) => {
                    <attackCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bondDistributionMode(inner) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeRootL2Block(inner) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimCredit(inner) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimData(inner) => {
                    <claimDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimDataLen(inner) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claims(inner) => {
                    <claimsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::clockExtension(inner) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeGame(inner) => {
                    <closeGameCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createdAt(inner) => {
                    <createdAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::credit(inner) => {
                    <creditCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::defend(inner) => {
                    <defendCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::extraData(inner) => {
                    <extraDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameCreator(inner) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameData(inner) => {
                    <gameDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameType(inner) => {
                    <gameTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getChallengerDuration(inner) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNumToResolve(inner) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRequiredBond(inner) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasUnlockedCredit(inner) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l1Head(inner) => {
                    <l1HeadCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::l2BlockNumber(inner) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2BlockNumberChallenged(inner) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2BlockNumberChallenger(inner) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2ChainId(inner) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2SequenceNumber(inner) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxClockDuration(inner) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxGameDepth(inner) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::r#move(inner) => {
                    <moveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::normalModeCredit(inner) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::refundModeCredit(inner) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolutionCheckpoints(inner) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolve(inner) => {
                    <resolveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::resolveClaim(inner) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolvedAt(inner) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolvedSubgames(inner) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rootClaim(inner) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::splitDepth(inner) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingBlockNumber(inner) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingOutputRoot(inner) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingRootHash(inner) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::status(inner) => {
                    <statusCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::step(inner) => {
                    <stepCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::subgames(inner) => {
                    <subgamesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::vm(inner) => {
                    <vmCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::wasRespectedGameTypeWhenCreated(inner) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IFaultDisputeGame`](self) custom errors.
    #[derive(Clone)]
    pub enum IFaultDisputeGameErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AnchorRootNotFound(AnchorRootNotFound),
        #[allow(missing_docs)]
        BadExtraData(BadExtraData),
        #[allow(missing_docs)]
        BlockNumberMatches(BlockNumberMatches),
        #[allow(missing_docs)]
        BondTransferFailed(BondTransferFailed),
        #[allow(missing_docs)]
        CannotDefendRootClaim(CannotDefendRootClaim),
        #[allow(missing_docs)]
        ClaimAboveSplit(ClaimAboveSplit),
        #[allow(missing_docs)]
        ClaimAlreadyExists(ClaimAlreadyExists),
        #[allow(missing_docs)]
        ClaimAlreadyResolved(ClaimAlreadyResolved),
        #[allow(missing_docs)]
        ClockNotExpired(ClockNotExpired),
        #[allow(missing_docs)]
        ClockTimeExceeded(ClockTimeExceeded),
        #[allow(missing_docs)]
        ContentLengthMismatch(ContentLengthMismatch),
        #[allow(missing_docs)]
        DuplicateStep(DuplicateStep),
        #[allow(missing_docs)]
        EmptyItem(EmptyItem),
        #[allow(missing_docs)]
        GameDepthExceeded(GameDepthExceeded),
        #[allow(missing_docs)]
        GameNotFinalized(GameNotFinalized),
        #[allow(missing_docs)]
        GameNotInProgress(GameNotInProgress),
        #[allow(missing_docs)]
        GameNotResolved(GameNotResolved),
        #[allow(missing_docs)]
        GamePaused(GamePaused),
        #[allow(missing_docs)]
        IncorrectBondAmount(IncorrectBondAmount),
        #[allow(missing_docs)]
        InvalidBondDistributionMode(InvalidBondDistributionMode),
        #[allow(missing_docs)]
        InvalidChallengePeriod(InvalidChallengePeriod),
        #[allow(missing_docs)]
        InvalidClockExtension(InvalidClockExtension),
        #[allow(missing_docs)]
        InvalidDataRemainder(InvalidDataRemainder),
        #[allow(missing_docs)]
        InvalidDisputedClaimIndex(InvalidDisputedClaimIndex),
        #[allow(missing_docs)]
        InvalidHeader(InvalidHeader),
        #[allow(missing_docs)]
        InvalidHeaderRLP(InvalidHeaderRLP),
        #[allow(missing_docs)]
        InvalidLocalIdent(InvalidLocalIdent),
        #[allow(missing_docs)]
        InvalidOutputRootProof(InvalidOutputRootProof),
        #[allow(missing_docs)]
        InvalidParent(InvalidParent),
        #[allow(missing_docs)]
        InvalidPrestate(InvalidPrestate),
        #[allow(missing_docs)]
        InvalidSplitDepth(InvalidSplitDepth),
        #[allow(missing_docs)]
        L2BlockNumberChallenged(L2BlockNumberChallenged),
        #[allow(missing_docs)]
        MaxDepthTooLarge(MaxDepthTooLarge),
        #[allow(missing_docs)]
        NoCreditToClaim(NoCreditToClaim),
        #[allow(missing_docs)]
        OutOfOrderResolution(OutOfOrderResolution),
        #[allow(missing_docs)]
        ReservedGameType(ReservedGameType),
        #[allow(missing_docs)]
        UnexpectedList(UnexpectedList),
        #[allow(missing_docs)]
        UnexpectedRootClaim(UnexpectedRootClaim),
        #[allow(missing_docs)]
        UnexpectedString(UnexpectedString),
        #[allow(missing_docs)]
        ValidStep(ValidStep),
    }
    impl IFaultDisputeGameErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 138u8, 61u8, 244u8],
            [13u8, 193u8, 73u8, 240u8],
            [14u8, 162u8, 231u8, 82u8],
            [23u8, 191u8, 229u8, 247u8],
            [31u8, 249u8, 178u8, 228u8],
            [48u8, 20u8, 3u8, 50u8],
            [51u8, 129u8, 209u8, 20u8],
            [55u8, 154u8, 126u8, 217u8],
            [57u8, 246u8, 139u8, 56u8],
            [72u8, 81u8, 189u8, 155u8],
            [75u8, 156u8, 106u8, 190u8],
            [86u8, 245u8, 123u8, 43u8],
            [90u8, 180u8, 88u8, 251u8],
            [92u8, 85u8, 55u8, 184u8],
            [95u8, 83u8, 221u8, 152u8],
            [102u8, 201u8, 68u8, 133u8],
            [103u8, 254u8, 25u8, 80u8],
            [105u8, 101u8, 80u8, 255u8],
            [106u8, 107u8, 195u8, 178u8],
            [119u8, 223u8, 227u8, 50u8],
            [128u8, 73u8, 126u8, 59u8],
            [131u8, 230u8, 204u8, 107u8],
            [134u8, 32u8, 170u8, 25u8],
            [141u8, 119u8, 236u8, 172u8],
            [144u8, 113u8, 230u8, 175u8],
            [152u8, 36u8, 189u8, 171u8],
            [154u8, 7u8, 102u8, 70u8],
            [156u8, 192u8, 11u8, 91u8],
            [164u8, 38u8, 55u8, 188u8],
            [179u8, 75u8, 92u8, 34u8],
            [180u8, 225u8, 36u8, 51u8],
            [184u8, 237u8, 136u8, 48u8],
            [186u8, 187u8, 1u8, 221u8],
            [193u8, 5u8, 38u8, 10u8],
            [216u8, 29u8, 88u8, 59u8],
            [230u8, 44u8, 207u8, 57u8],
            [241u8, 169u8, 69u8, 129u8],
            [242u8, 68u8, 11u8, 83u8],
            [244u8, 2u8, 57u8, 219u8],
            [251u8, 78u8, 64u8, 221u8],
            [255u8, 19u8, 126u8, 101u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidBondDistributionMode),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(L2BlockNumberChallenged),
            ::core::stringify!(NoCreditToClaim),
            ::core::stringify!(UnexpectedList),
            ::core::stringify!(InvalidDisputedClaimIndex),
            ::core::stringify!(ClockTimeExceeded),
            ::core::stringify!(GamePaused),
            ::core::stringify!(ReservedGameType),
            ::core::stringify!(GameNotFinalized),
            ::core::stringify!(UnexpectedString),
            ::core::stringify!(GameDepthExceeded),
            ::core::stringify!(EmptyItem),
            ::core::stringify!(InvalidDataRemainder),
            ::core::stringify!(InvalidParent),
            ::core::stringify!(ContentLengthMismatch),
            ::core::stringify!(GameNotInProgress),
            ::core::stringify!(InvalidPrestate),
            ::core::stringify!(AnchorRootNotFound),
            ::core::stringify!(MaxDepthTooLarge),
            ::core::stringify!(ClaimAlreadyExists),
            ::core::stringify!(BondTransferFailed),
            ::core::stringify!(IncorrectBondAmount),
            ::core::stringify!(InvalidClockExtension),
            ::core::stringify!(DuplicateStep),
            ::core::stringify!(BadExtraData),
            ::core::stringify!(OutOfOrderResolution),
            ::core::stringify!(InvalidOutputRootProof),
            ::core::stringify!(CannotDefendRootClaim),
            ::core::stringify!(ClaimAboveSplit),
            ::core::stringify!(InvalidChallengePeriod),
            ::core::stringify!(BlockNumberMatches),
            ::core::stringify!(InvalidHeader),
            ::core::stringify!(GameNotResolved),
            ::core::stringify!(InvalidHeaderRLP),
            ::core::stringify!(InvalidSplitDepth),
            ::core::stringify!(ClaimAlreadyResolved),
            ::core::stringify!(ClockNotExpired),
            ::core::stringify!(UnexpectedRootClaim),
            ::core::stringify!(ValidStep),
            ::core::stringify!(InvalidLocalIdent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidBondDistributionMode as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <L2BlockNumberChallenged as alloy_sol_types::SolError>::SIGNATURE,
            <NoCreditToClaim as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedList as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ClockTimeExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <GamePaused as alloy_sol_types::SolError>::SIGNATURE,
            <ReservedGameType as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedString as alloy_sol_types::SolError>::SIGNATURE,
            <GameDepthExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyItem as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDataRemainder as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidParent as alloy_sol_types::SolError>::SIGNATURE,
            <ContentLengthMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotInProgress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidPrestate as alloy_sol_types::SolError>::SIGNATURE,
            <AnchorRootNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <MaxDepthTooLarge as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAlreadyExists as alloy_sol_types::SolError>::SIGNATURE,
            <BondTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
            <IncorrectBondAmount as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidClockExtension as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateStep as alloy_sol_types::SolError>::SIGNATURE,
            <BadExtraData as alloy_sol_types::SolError>::SIGNATURE,
            <OutOfOrderResolution as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidOutputRootProof as alloy_sol_types::SolError>::SIGNATURE,
            <CannotDefendRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAboveSplit as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChallengePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <BlockNumberMatches as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeader as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotResolved as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeaderRLP as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSplitDepth as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAlreadyResolved as alloy_sol_types::SolError>::SIGNATURE,
            <ClockNotExpired as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <ValidStep as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidLocalIdent as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IFaultDisputeGameErrors {
        const NAME: &'static str = "IFaultDisputeGameErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 41usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AnchorRootNotFound(_) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadExtraData(_) => {
                    <BadExtraData as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BlockNumberMatches(_) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BondTransferFailed(_) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotDefendRootClaim(_) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAboveSplit(_) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAlreadyExists(_) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAlreadyResolved(_) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClockNotExpired(_) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClockTimeExceeded(_) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContentLengthMismatch(_) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateStep(_) => {
                    <DuplicateStep as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyItem(_) => <EmptyItem as alloy_sol_types::SolError>::SELECTOR,
                Self::GameDepthExceeded(_) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotFinalized(_) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotInProgress(_) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotResolved(_) => {
                    <GameNotResolved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GamePaused(_) => {
                    <GamePaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IncorrectBondAmount(_) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBondDistributionMode(_) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChallengePeriod(_) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidClockExtension(_) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDataRemainder(_) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDisputedClaimIndex(_) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeader(_) => {
                    <InvalidHeader as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeaderRLP(_) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLocalIdent(_) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOutputRootProof(_) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidParent(_) => {
                    <InvalidParent as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPrestate(_) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSplitDepth(_) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::SELECTOR
                }
                Self::L2BlockNumberChallenged(_) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDepthTooLarge(_) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoCreditToClaim(_) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderResolution(_) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReservedGameType(_) => {
                    <ReservedGameType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedList(_) => {
                    <UnexpectedList as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedRootClaim(_) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedString(_) => {
                    <UnexpectedString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidStep(_) => <ValidStep as alloy_sol_types::SolError>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<IFaultDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidBondDistributionMode)
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::L2BlockNumberChallenged)
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidDisputedClaimIndex)
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IFaultDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn ReservedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ReservedGameType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ReservedGameType)
                    }
                    ReservedGameType
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IFaultDisputeGameErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IFaultDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IFaultDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidLocalIdent)
                    }
                    InvalidLocalIdent
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<IFaultDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidBondDistributionMode)
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::L2BlockNumberChallenged)
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidDisputedClaimIndex)
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn ReservedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ReservedGameType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ReservedGameType)
                    }
                    ReservedGameType
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IFaultDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IFaultDisputeGameErrors::InvalidLocalIdent)
                    }
                    InvalidLocalIdent
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AnchorRootNotFound(inner) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BlockNumberMatches(inner) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BondTransferFailed(inner) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotDefendRootClaim(inner) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAboveSplit(inner) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAlreadyExists(inner) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAlreadyResolved(inner) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClockNotExpired(inner) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClockTimeExceeded(inner) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GameDepthExceeded(inner) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotFinalized(inner) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotInProgress(inner) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotResolved(inner) => {
                    <GameNotResolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GamePaused(inner) => {
                    <GamePaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IncorrectBondAmount(inner) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBondDistributionMode(inner) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChallengePeriod(inner) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidClockExtension(inner) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHeader(inner) => {
                    <InvalidHeader as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidHeaderRLP(inner) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLocalIdent(inner) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOutputRootProof(inner) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidParent(inner) => {
                    <InvalidParent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidPrestate(inner) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSplitDepth(inner) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L2BlockNumberChallenged(inner) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoCreditToClaim(inner) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfOrderResolution(inner) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReservedGameType(inner) => {
                    <ReservedGameType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedRootClaim(inner) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedString(inner) => {
                    <UnexpectedString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidStep(inner) => {
                    <ValidStep as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AnchorRootNotFound(inner) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BlockNumberMatches(inner) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BondTransferFailed(inner) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotDefendRootClaim(inner) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAboveSplit(inner) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAlreadyExists(inner) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAlreadyResolved(inner) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClockNotExpired(inner) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClockTimeExceeded(inner) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::GameDepthExceeded(inner) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotFinalized(inner) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotInProgress(inner) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotResolved(inner) => {
                    <GameNotResolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GamePaused(inner) => {
                    <GamePaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::IncorrectBondAmount(inner) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBondDistributionMode(inner) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidChallengePeriod(inner) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidClockExtension(inner) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeader(inner) => {
                    <InvalidHeader as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeaderRLP(inner) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLocalIdent(inner) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOutputRootProof(inner) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidParent(inner) => {
                    <InvalidParent as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPrestate(inner) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSplitDepth(inner) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L2BlockNumberChallenged(inner) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoCreditToClaim(inner) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfOrderResolution(inner) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReservedGameType(inner) => {
                    <ReservedGameType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedRootClaim(inner) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedString(inner) => {
                    <UnexpectedString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidStep(inner) => {
                    <ValidStep as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IFaultDisputeGame`](self) events.
    #[derive(Clone)]
    pub enum IFaultDisputeGameEvents {
        #[allow(missing_docs)]
        GameClosed(GameClosed),
        #[allow(missing_docs)]
        Move(Move),
        #[allow(missing_docs)]
        Resolved(Resolved),
    }
    impl IFaultDisputeGameEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                94u8, 24u8, 111u8, 9u8, 185u8, 201u8, 52u8, 145u8, 241u8, 78u8, 39u8,
                126u8, 234u8, 127u8, 170u8, 93u8, 230u8, 162u8, 212u8, 189u8, 167u8,
                90u8, 121u8, 175u8, 122u8, 54u8, 132u8, 251u8, 251u8, 66u8, 218u8, 96u8,
            ],
            [
                153u8, 8u8, 234u8, 172u8, 6u8, 69u8, 223u8, 157u8, 7u8, 4u8, 208u8,
                106u8, 220u8, 158u8, 7u8, 51u8, 124u8, 149u8, 29u8, 226u8, 240u8, 107u8,
                95u8, 40u8, 54u8, 21u8, 29u8, 72u8, 213u8, 228u8, 114u8, 47u8,
            ],
            [
                155u8, 50u8, 69u8, 116u8, 14u8, 195u8, 177u8, 85u8, 9u8, 138u8, 85u8,
                190u8, 132u8, 149u8, 122u8, 77u8, 161u8, 62u8, 175u8, 127u8, 20u8, 168u8,
                188u8, 111u8, 83u8, 18u8, 108u8, 11u8, 147u8, 80u8, 242u8, 190u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(Resolved),
            ::core::stringify!(GameClosed),
            ::core::stringify!(Move),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <Resolved as alloy_sol_types::SolEvent>::SIGNATURE,
            <GameClosed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Move as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IFaultDisputeGameEvents {
        const NAME: &'static str = "IFaultDisputeGameEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<GameClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GameClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GameClosed)
                }
                Some(<Move as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Move as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Move)
                }
                Some(<Resolved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Resolved as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Resolved)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for IFaultDisputeGameEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GameClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Move(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Resolved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GameClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Move(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Resolved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IFaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`IFaultDisputeGameInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IFaultDisputeGameInstance<P, N> {
        IFaultDisputeGameInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IFaultDisputeGameInstance<P, N>>,
    > {
        IFaultDisputeGameInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        IFaultDisputeGameInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`IFaultDisputeGame`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IFaultDisputeGame`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IFaultDisputeGameInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IFaultDisputeGameInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IFaultDisputeGameInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IFaultDisputeGameInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IFaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`IFaultDisputeGameInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<IFaultDisputeGameInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IFaultDisputeGameInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IFaultDisputeGameInstance<P, N> {
            IFaultDisputeGameInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IFaultDisputeGameInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`__constructor__`] function.
        pub fn __constructor__(
            &self,
            _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, __constructor__Call, N> {
            self.call_builder(&__constructor__Call { _params })
        }
        ///Creates a new call builder for the [`absolutePrestate`] function.
        pub fn absolutePrestate(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, absolutePrestateCall, N> {
            self.call_builder(&absolutePrestateCall)
        }
        ///Creates a new call builder for the [`addLocalData`] function.
        pub fn addLocalData(
            &self,
            _ident: alloy::sol_types::private::primitives::aliases::U256,
            _execLeafIdx: alloy::sol_types::private::primitives::aliases::U256,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, addLocalDataCall, N> {
            self.call_builder(
                &addLocalDataCall {
                    _ident,
                    _execLeafIdx,
                    _partOffset,
                },
            )
        }
        ///Creates a new call builder for the [`anchorStateRegistry`] function.
        pub fn anchorStateRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, anchorStateRegistryCall, N> {
            self.call_builder(&anchorStateRegistryCall)
        }
        ///Creates a new call builder for the [`attack`] function.
        pub fn attack(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, attackCall, N> {
            self.call_builder(
                &attackCall {
                    _disputed,
                    _parentIndex,
                    _claim,
                },
            )
        }
        ///Creates a new call builder for the [`bondDistributionMode`] function.
        pub fn bondDistributionMode(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, bondDistributionModeCall, N> {
            self.call_builder(&bondDistributionModeCall)
        }
        ///Creates a new call builder for the [`challengeRootL2Block`] function.
        pub fn challengeRootL2Block(
            &self,
            _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
            _headerRLP: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, challengeRootL2BlockCall, N> {
            self.call_builder(
                &challengeRootL2BlockCall {
                    _outputRootProof,
                    _headerRLP,
                },
            )
        }
        ///Creates a new call builder for the [`claimCredit`] function.
        pub fn claimCredit(
            &self,
            _recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, claimCreditCall, N> {
            self.call_builder(&claimCreditCall { _recipient })
        }
        ///Creates a new call builder for the [`claimData`] function.
        pub fn claimData(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, claimDataCall, N> {
            self.call_builder(&claimDataCall(_0))
        }
        ///Creates a new call builder for the [`claimDataLen`] function.
        pub fn claimDataLen(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, claimDataLenCall, N> {
            self.call_builder(&claimDataLenCall)
        }
        ///Creates a new call builder for the [`claims`] function.
        pub fn claims(
            &self,
            _0: <Hash as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, claimsCall, N> {
            self.call_builder(&claimsCall(_0))
        }
        ///Creates a new call builder for the [`clockExtension`] function.
        pub fn clockExtension(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, clockExtensionCall, N> {
            self.call_builder(&clockExtensionCall)
        }
        ///Creates a new call builder for the [`closeGame`] function.
        pub fn closeGame(&self) -> alloy_contract::SolCallBuilder<&P, closeGameCall, N> {
            self.call_builder(&closeGameCall)
        }
        ///Creates a new call builder for the [`createdAt`] function.
        pub fn createdAt(&self) -> alloy_contract::SolCallBuilder<&P, createdAtCall, N> {
            self.call_builder(&createdAtCall)
        }
        ///Creates a new call builder for the [`credit`] function.
        pub fn credit(
            &self,
            _recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, creditCall, N> {
            self.call_builder(&creditCall { _recipient })
        }
        ///Creates a new call builder for the [`defend`] function.
        pub fn defend(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, defendCall, N> {
            self.call_builder(
                &defendCall {
                    _disputed,
                    _parentIndex,
                    _claim,
                },
            )
        }
        ///Creates a new call builder for the [`extraData`] function.
        pub fn extraData(&self) -> alloy_contract::SolCallBuilder<&P, extraDataCall, N> {
            self.call_builder(&extraDataCall)
        }
        ///Creates a new call builder for the [`gameCreator`] function.
        pub fn gameCreator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gameCreatorCall, N> {
            self.call_builder(&gameCreatorCall)
        }
        ///Creates a new call builder for the [`gameData`] function.
        pub fn gameData(&self) -> alloy_contract::SolCallBuilder<&P, gameDataCall, N> {
            self.call_builder(&gameDataCall)
        }
        ///Creates a new call builder for the [`gameType`] function.
        pub fn gameType(&self) -> alloy_contract::SolCallBuilder<&P, gameTypeCall, N> {
            self.call_builder(&gameTypeCall)
        }
        ///Creates a new call builder for the [`getChallengerDuration`] function.
        pub fn getChallengerDuration(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getChallengerDurationCall, N> {
            self.call_builder(
                &getChallengerDurationCall {
                    _claimIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getNumToResolve`] function.
        pub fn getNumToResolve(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getNumToResolveCall, N> {
            self.call_builder(&getNumToResolveCall { _claimIndex })
        }
        ///Creates a new call builder for the [`getRequiredBond`] function.
        pub fn getRequiredBond(
            &self,
            _position: <Position as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getRequiredBondCall, N> {
            self.call_builder(&getRequiredBondCall { _position })
        }
        ///Creates a new call builder for the [`hasUnlockedCredit`] function.
        pub fn hasUnlockedCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasUnlockedCreditCall, N> {
            self.call_builder(&hasUnlockedCreditCall(_0))
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall)
        }
        ///Creates a new call builder for the [`l1Head`] function.
        pub fn l1Head(&self) -> alloy_contract::SolCallBuilder<&P, l1HeadCall, N> {
            self.call_builder(&l1HeadCall)
        }
        ///Creates a new call builder for the [`l2BlockNumber`] function.
        pub fn l2BlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberCall, N> {
            self.call_builder(&l2BlockNumberCall)
        }
        ///Creates a new call builder for the [`l2BlockNumberChallenged`] function.
        pub fn l2BlockNumberChallenged(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberChallengedCall, N> {
            self.call_builder(&l2BlockNumberChallengedCall)
        }
        ///Creates a new call builder for the [`l2BlockNumberChallenger`] function.
        pub fn l2BlockNumberChallenger(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberChallengerCall, N> {
            self.call_builder(&l2BlockNumberChallengerCall)
        }
        ///Creates a new call builder for the [`l2ChainId`] function.
        pub fn l2ChainId(&self) -> alloy_contract::SolCallBuilder<&P, l2ChainIdCall, N> {
            self.call_builder(&l2ChainIdCall)
        }
        ///Creates a new call builder for the [`l2SequenceNumber`] function.
        pub fn l2SequenceNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2SequenceNumberCall, N> {
            self.call_builder(&l2SequenceNumberCall)
        }
        ///Creates a new call builder for the [`maxClockDuration`] function.
        pub fn maxClockDuration(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxClockDurationCall, N> {
            self.call_builder(&maxClockDurationCall)
        }
        ///Creates a new call builder for the [`maxGameDepth`] function.
        pub fn maxGameDepth(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxGameDepthCall, N> {
            self.call_builder(&maxGameDepthCall)
        }
        ///Creates a new call builder for the [`r#move`] function.
        pub fn r#move(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _challengeIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
            _isAttack: bool,
        ) -> alloy_contract::SolCallBuilder<&P, moveCall, N> {
            self.call_builder(
                &moveCall {
                    _disputed,
                    _challengeIndex,
                    _claim,
                    _isAttack,
                },
            )
        }
        ///Creates a new call builder for the [`normalModeCredit`] function.
        pub fn normalModeCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, normalModeCreditCall, N> {
            self.call_builder(&normalModeCreditCall(_0))
        }
        ///Creates a new call builder for the [`refundModeCredit`] function.
        pub fn refundModeCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, refundModeCreditCall, N> {
            self.call_builder(&refundModeCreditCall(_0))
        }
        ///Creates a new call builder for the [`resolutionCheckpoints`] function.
        pub fn resolutionCheckpoints(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolutionCheckpointsCall, N> {
            self.call_builder(&resolutionCheckpointsCall(_0))
        }
        ///Creates a new call builder for the [`resolve`] function.
        pub fn resolve(&self) -> alloy_contract::SolCallBuilder<&P, resolveCall, N> {
            self.call_builder(&resolveCall)
        }
        ///Creates a new call builder for the [`resolveClaim`] function.
        pub fn resolveClaim(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
            _numToResolve: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolveClaimCall, N> {
            self.call_builder(
                &resolveClaimCall {
                    _claimIndex,
                    _numToResolve,
                },
            )
        }
        ///Creates a new call builder for the [`resolvedAt`] function.
        pub fn resolvedAt(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, resolvedAtCall, N> {
            self.call_builder(&resolvedAtCall)
        }
        ///Creates a new call builder for the [`resolvedSubgames`] function.
        pub fn resolvedSubgames(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolvedSubgamesCall, N> {
            self.call_builder(&resolvedSubgamesCall(_0))
        }
        ///Creates a new call builder for the [`rootClaim`] function.
        pub fn rootClaim(&self) -> alloy_contract::SolCallBuilder<&P, rootClaimCall, N> {
            self.call_builder(&rootClaimCall)
        }
        ///Creates a new call builder for the [`splitDepth`] function.
        pub fn splitDepth(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, splitDepthCall, N> {
            self.call_builder(&splitDepthCall)
        }
        ///Creates a new call builder for the [`startingBlockNumber`] function.
        pub fn startingBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingBlockNumberCall, N> {
            self.call_builder(&startingBlockNumberCall)
        }
        ///Creates a new call builder for the [`startingOutputRoot`] function.
        pub fn startingOutputRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingOutputRootCall, N> {
            self.call_builder(&startingOutputRootCall)
        }
        ///Creates a new call builder for the [`startingRootHash`] function.
        pub fn startingRootHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingRootHashCall, N> {
            self.call_builder(&startingRootHashCall)
        }
        ///Creates a new call builder for the [`status`] function.
        pub fn status(&self) -> alloy_contract::SolCallBuilder<&P, statusCall, N> {
            self.call_builder(&statusCall)
        }
        ///Creates a new call builder for the [`step`] function.
        pub fn step(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
            _isAttack: bool,
            _stateData: alloy::sol_types::private::Bytes,
            _proof: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, stepCall, N> {
            self.call_builder(
                &stepCall {
                    _claimIndex,
                    _isAttack,
                    _stateData,
                    _proof,
                },
            )
        }
        ///Creates a new call builder for the [`subgames`] function.
        pub fn subgames(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, subgamesCall, N> {
            self.call_builder(&subgamesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
        ///Creates a new call builder for the [`vm`] function.
        pub fn vm(&self) -> alloy_contract::SolCallBuilder<&P, vmCall, N> {
            self.call_builder(&vmCall)
        }
        ///Creates a new call builder for the [`wasRespectedGameTypeWhenCreated`] function.
        pub fn wasRespectedGameTypeWhenCreated(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, wasRespectedGameTypeWhenCreatedCall, N> {
            self.call_builder(&wasRespectedGameTypeWhenCreatedCall)
        }
        ///Creates a new call builder for the [`weth`] function.
        pub fn weth(&self) -> alloy_contract::SolCallBuilder<&P, wethCall, N> {
            self.call_builder(&wethCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IFaultDisputeGameInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`GameClosed`] event.
        pub fn GameClosed_filter(&self) -> alloy_contract::Event<&P, GameClosed, N> {
            self.event_filter::<GameClosed>()
        }
        ///Creates a new event filter for the [`Move`] event.
        pub fn Move_filter(&self) -> alloy_contract::Event<&P, Move, N> {
            self.event_filter::<Move>()
        }
        ///Creates a new event filter for the [`Resolved`] event.
        pub fn Resolved_filter(&self) -> alloy_contract::Event<&P, Resolved, N> {
            self.event_filter::<Resolved>()
        }
    }
}
