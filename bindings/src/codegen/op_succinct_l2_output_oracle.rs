///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct OutputProposal { bytes32 outputRoot; uint128 timestamp; uint128 l2BlockNumber; }
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
struct OutputProposal { bytes32 outputRoot; uint128 timestamp; uint128 l2BlockNumber; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputProposal {
        #[allow(missing_docs)]
        pub outputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub timestamp: u128,
        #[allow(missing_docs)]
        pub l2BlockNumber: u128,
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
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            u128,
            u128,
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
        impl ::core::convert::From<OutputProposal> for UnderlyingRustTuple<'_> {
            fn from(value: OutputProposal) -> Self {
                (value.outputRoot, value.timestamp, value.l2BlockNumber)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    outputRoot: tuple.0,
                    timestamp: tuple.1,
                    l2BlockNumber: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputProposal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputProposal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputRoot),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2BlockNumber),
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
        impl alloy_sol_types::SolType for OutputProposal {
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
        impl alloy_sol_types::SolStruct for OutputProposal {
            const NAME: &'static str = "OutputProposal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputProposal(bytes32 outputRoot,uint128 timestamp,uint128 l2BlockNumber)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.outputRoot)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2BlockNumber)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputProposal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2BlockNumber,
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
                    &rust.outputRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2BlockNumber,
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
    struct OutputProposal {
        bytes32 outputRoot;
        uint128 timestamp;
        uint128 l2BlockNumber;
    }
}

interface OPSuccinctL2OutputOracle {
    struct InitParams {
        address challenger;
        address proposer;
        address owner;
        uint256 finalizationPeriodSeconds;
        uint256 l2BlockTime;
        bytes32 aggregationVkey;
        bytes32 rangeVkeyCommitment;
        bytes32 rollupConfigHash;
        bytes32 startingOutputRoot;
        uint256 startingBlockNumber;
        uint256 startingTimestamp;
        uint256 submissionInterval;
        address verifier;
        uint256 fallbackTimeout;
    }
    struct OpSuccinctConfig {
        bytes32 aggregationVkey;
        bytes32 rangeVkeyCommitment;
        bytes32 rollupConfigHash;
    }

    error L1BlockHashNotAvailable();
    error L1BlockHashNotCheckpointed();

    event DisputeGameFactorySet(address indexed disputeGameFactory);
    event Initialized(uint8 version);
    event OpSuccinctConfigDeleted(bytes32 indexed configName);
    event OpSuccinctConfigUpdated(bytes32 indexed configName, bytes32 aggregationVkey, bytes32 rangeVkeyCommitment, bytes32 rollupConfigHash);
    event OptimisticModeToggled(bool indexed enabled, uint256 finalizationPeriodSeconds);
    event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp);
    event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event ProposerUpdated(address indexed proposer, bool added);
    event SubmissionIntervalUpdated(uint256 oldSubmissionInterval, uint256 newSubmissionInterval);
    event VerifierUpdated(address indexed oldVerifier, address indexed newVerifier);

    constructor();

    function GENESIS_CONFIG_NAME() external view returns (bytes32);
    function addOpSuccinctConfig(bytes32 _configName, bytes32 _rollupConfigHash, bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment) external;
    function addProposer(address _proposer) external;
    function aggregationVkey() external view returns (bytes32);
    function approvedProposers(address) external view returns (bool);
    function challenger() external view returns (address);
    function checkpointBlockHash(uint256 _blockNumber) external;
    function computeL2Timestamp(uint256 _l2BlockNumber) external view returns (uint256);
    function deleteL2Outputs(uint256 _l2OutputIndex) external;
    function deleteOpSuccinctConfig(bytes32 _configName) external;
    function dgfProposeL2Output(bytes32 _configName, bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof, address _proverAddress) external payable returns (address _game);
    function disableOptimisticMode(uint256 _finalizationPeriodSeconds) external;
    function disputeGameFactory() external view returns (address);
    function enableOptimisticMode(uint256 _finalizationPeriodSeconds) external;
    function fallbackTimeout() external view returns (uint256);
    function finalizationPeriodSeconds() external view returns (uint256);
    function getL2Output(uint256 _l2OutputIndex) external view returns (Types.OutputProposal memory);
    function getL2OutputAfter(uint256 _l2BlockNumber) external view returns (Types.OutputProposal memory);
    function getL2OutputIndexAfter(uint256 _l2BlockNumber) external view returns (uint256);
    function historicBlockHashes(uint256) external view returns (bytes32);
    function initialize(InitParams memory _initParams) external;
    function initializerVersion() external view returns (uint8);
    function isValidOpSuccinctConfig(OpSuccinctConfig memory _config) external pure returns (bool);
    function l2BlockTime() external view returns (uint256);
    function lastProposalTimestamp() external view returns (uint256);
    function latestBlockNumber() external view returns (uint256);
    function latestOutputIndex() external view returns (uint256);
    function nextBlockNumber() external view returns (uint256);
    function nextOutputIndex() external view returns (uint256);
    function opSuccinctConfigs(bytes32) external view returns (bytes32 aggregationVkey, bytes32 rangeVkeyCommitment, bytes32 rollupConfigHash);
    function optimisticMode() external view returns (bool);
    function owner() external view returns (address);
    function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, bytes32 _l1BlockHash, uint256 _l1BlockNumber) external payable;
    function proposeL2Output(bytes32 _configName, bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof, address _proverAddress) external;
    function proposer() external view returns (address);
    function rangeVkeyCommitment() external view returns (bytes32);
    function removeProposer(address _proposer) external;
    function rollupConfigHash() external view returns (bytes32);
    function setDisputeGameFactory(address _disputeGameFactory) external;
    function startingBlockNumber() external view returns (uint256);
    function startingTimestamp() external view returns (uint256);
    function submissionInterval() external view returns (uint256);
    function transferOwnership(address _owner) external;
    function updateSubmissionInterval(uint256 _submissionInterval) external;
    function updateVerifier(address _verifier) external;
    function verifier() external view returns (address);
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "GENESIS_CONFIG_NAME",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addOpSuccinctConfig",
    "inputs": [
      {
        "name": "_configName",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_rollupConfigHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_aggregationVkey",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_rangeVkeyCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addProposer",
    "inputs": [
      {
        "name": "_proposer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "aggregationVkey",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "approvedProposers",
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
    "name": "challenger",
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
    "name": "checkpointBlockHash",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "computeL2Timestamp",
    "inputs": [
      {
        "name": "_l2BlockNumber",
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
    "name": "deleteL2Outputs",
    "inputs": [
      {
        "name": "_l2OutputIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deleteOpSuccinctConfig",
    "inputs": [
      {
        "name": "_configName",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "dgfProposeL2Output",
    "inputs": [
      {
        "name": "_configName",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_outputRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_l2BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_l1BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_proof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_proverAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "_game",
        "type": "address",
        "internalType": "contract IDisputeGame"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "disableOptimisticMode",
    "inputs": [
      {
        "name": "_finalizationPeriodSeconds",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "disputeGameFactory",
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
    "name": "enableOptimisticMode",
    "inputs": [
      {
        "name": "_finalizationPeriodSeconds",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "fallbackTimeout",
    "inputs": [],
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
    "name": "finalizationPeriodSeconds",
    "inputs": [],
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
    "name": "getL2Output",
    "inputs": [
      {
        "name": "_l2OutputIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.OutputProposal",
        "components": [
          {
            "name": "outputRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "timestamp",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "l2BlockNumber",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getL2OutputAfter",
    "inputs": [
      {
        "name": "_l2BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.OutputProposal",
        "components": [
          {
            "name": "outputRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "timestamp",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "l2BlockNumber",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getL2OutputIndexAfter",
    "inputs": [
      {
        "name": "_l2BlockNumber",
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
    "name": "historicBlockHashes",
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
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_initParams",
        "type": "tuple",
        "internalType": "struct OPSuccinctL2OutputOracle.InitParams",
        "components": [
          {
            "name": "challenger",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "proposer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "finalizationPeriodSeconds",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "l2BlockTime",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "aggregationVkey",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rangeVkeyCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rollupConfigHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "startingOutputRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "startingBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startingTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "submissionInterval",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "verifier",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "fallbackTimeout",
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
    "name": "initializerVersion",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isValidOpSuccinctConfig",
    "inputs": [
      {
        "name": "_config",
        "type": "tuple",
        "internalType": "struct OPSuccinctL2OutputOracle.OpSuccinctConfig",
        "components": [
          {
            "name": "aggregationVkey",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rangeVkeyCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rollupConfigHash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "l2BlockTime",
    "inputs": [],
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
    "name": "lastProposalTimestamp",
    "inputs": [],
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
    "name": "latestBlockNumber",
    "inputs": [],
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
    "name": "latestOutputIndex",
    "inputs": [],
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
    "name": "nextBlockNumber",
    "inputs": [],
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
    "name": "nextOutputIndex",
    "inputs": [],
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
    "name": "opSuccinctConfigs",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "aggregationVkey",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "rangeVkeyCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "rollupConfigHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "optimisticMode",
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
    "name": "owner",
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
    "name": "proposeL2Output",
    "inputs": [
      {
        "name": "_outputRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_l2BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_l1BlockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_l1BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "proposeL2Output",
    "inputs": [
      {
        "name": "_configName",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_outputRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_l2BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_l1BlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_proof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_proverAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proposer",
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
    "name": "rangeVkeyCommitment",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeProposer",
    "inputs": [
      {
        "name": "_proposer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rollupConfigHash",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setDisputeGameFactory",
    "inputs": [
      {
        "name": "_disputeGameFactory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startingBlockNumber",
    "inputs": [],
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
    "name": "startingTimestamp",
    "inputs": [],
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
    "name": "submissionInterval",
    "inputs": [],
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
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateSubmissionInterval",
    "inputs": [
      {
        "name": "_submissionInterval",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateVerifier",
    "inputs": [
      {
        "name": "_verifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifier",
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
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "DisputeGameFactorySet",
    "inputs": [
      {
        "name": "disputeGameFactory",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OpSuccinctConfigDeleted",
    "inputs": [
      {
        "name": "configName",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OpSuccinctConfigUpdated",
    "inputs": [
      {
        "name": "configName",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "aggregationVkey",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "rangeVkeyCommitment",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "rollupConfigHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OptimisticModeToggled",
    "inputs": [
      {
        "name": "enabled",
        "type": "bool",
        "indexed": true,
        "internalType": "bool"
      },
      {
        "name": "finalizationPeriodSeconds",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OutputProposed",
    "inputs": [
      {
        "name": "outputRoot",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "l2OutputIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "l2BlockNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "l1Timestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OutputsDeleted",
    "inputs": [
      {
        "name": "prevNextOutputIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newNextOutputIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposerUpdated",
    "inputs": [
      {
        "name": "proposer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "added",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubmissionIntervalUpdated",
    "inputs": [
      {
        "name": "oldSubmissionInterval",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newSubmissionInterval",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "VerifierUpdated",
    "inputs": [
      {
        "name": "oldVerifier",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newVerifier",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "L1BlockHashNotAvailable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "L1BlockHashNotCheckpointed",
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
pub mod OPSuccinctL2OutputOracle {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040523480156200001157600080fd5b50620000226200002860201b60201c565b620001d3565b600060019054906101000a900460ff16156200007b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401620000729062000176565b60405180910390fd5b60ff801660008054906101000a900460ff1660ff161015620000ed5760ff6000806101000a81548160ff021916908360ff1602179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860ff604051620000e49190620001b6565b60405180910390a15b565b600082825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320696e69746960008201527f616c697a696e6700000000000000000000000000000000000000000000000000602082015250565b60006200015e602783620000ef565b91506200016b8262000100565b604082019050919050565b6000602082019050818103600083015262000191816200014f565b9050919050565b600060ff82169050919050565b620001b08162000198565b82525050565b6000602082019050620001cd6000830184620001a5565b92915050565b61513f80620001e36000396000f3fe6080604052600436106102885760003560e01c8063887862721161015a578063ce5db8d6116100c1578063e1a41bcf1161007a578063e1a41bcf146109e2578063e40b7a1214610a0d578063ec5b2e3a14610a36578063f2b4e61714610a5f578063f2fde38b14610a8a578063f72f606d14610ab357610288565b8063ce5db8d6146108aa578063cf8e5cf0146108d5578063d1de856c14610912578063d46512761461094f578063dcec33481461098c578063e0c2f935146109b757610288565b8063a196b52511610113578063a196b52514610788578063a25ae557146107c5578063a4ee9d7b14610802578063a8e4fb901461082b578063b03cd41814610856578063c32e4e3e1461087f57610288565b8063887862721461069957806389c44cbb146106c45780638da5cb5b146106ed57806393991af31461071857806397fc007c146107435780639aaab6481461076c57610288565b80634ab309ac116101fe5780636abcf563116101b75780636abcf563146105805780636d9a1c8b146105ab57806370872aa5146105d65780637a41a035146106015780637f006420146106315780637f01ea681461066e57610288565b80634ab309ac1461046c578063534db0e21461049557806354fd4d50146104c057806360caf7a0146104eb57806369f16eec146105165780636a56620b1461054157610288565b8063336c9e8111610250578063336c9e811461035e5780633419d2c2146103875780634277bc06146103b05780634599c788146103db57806347c37e9c1461040657806349185e061461042f57610288565b806309d632d31461028d5780631e856800146102b65780632b31841e146102df5780632b7ac3f31461030a5780632c69796114610335575b600080fd5b34801561029957600080fd5b506102b460048036038101906102af9190613276565b610ade565b005b3480156102c257600080fd5b506102dd60048036038101906102d891906132d9565b610c18565b005b3480156102eb57600080fd5b506102f4610c76565b604051610301919061331f565b60405180910390f35b34801561031657600080fd5b5061031f610c7c565b60405161032c9190613349565b60405180910390f35b34801561034157600080fd5b5061035c600480360381019061035791906132d9565b610ca2565b005b34801561036a57600080fd5b50610385600480360381019061038091906132d9565b610de2565b005b34801561039357600080fd5b506103ae60048036038101906103a99190613276565b610efa565b005b3480156103bc57600080fd5b506103c5611011565b6040516103d29190613373565b60405180910390f35b3480156103e757600080fd5b506103f0611017565b6040516103fd9190613373565b60405180910390f35b34801561041257600080fd5b5061042d600480360381019061042891906133ba565b611098565b005b34801561043b57600080fd5b5061045660048036038101906104519190613516565b6112d0565b604051610463919061355e565b60405180910390f35b34801561047857600080fd5b50610493600480360381019061048e91906132d9565b61130a565b005b3480156104a157600080fd5b506104aa611449565b6040516104b79190613349565b60405180910390f35b3480156104cc57600080fd5b506104d561146f565b6040516104e29190613601565b60405180910390f35b3480156104f757600080fd5b506105006114a8565b60405161050d919061355e565b60405180910390f35b34801561052257600080fd5b5061052b6114bb565b6040516105389190613373565b60405180910390f35b34801561054d57600080fd5b5061056860048036038101906105639190613623565b6114d4565b60405161057793929190613650565b60405180910390f35b34801561058c57600080fd5b506105956114fe565b6040516105a29190613373565b60405180910390f35b3480156105b757600080fd5b506105c061150b565b6040516105cd919061331f565b60405180910390f35b3480156105e257600080fd5b506105eb611511565b6040516105f89190613373565b60405180910390f35b61061b60048036038101906106169190613741565b611517565b6040516106289190613849565b60405180910390f35b34801561063d57600080fd5b50610658600480360381019061065391906132d9565b611707565b6040516106659190613373565b60405180910390f35b34801561067a57600080fd5b5061068361184e565b6040516106909190613880565b60405180910390f35b3480156106a557600080fd5b506106ae611853565b6040516106bb9190613373565b60405180910390f35b3480156106d057600080fd5b506106eb60048036038101906106e691906132d9565b611859565b005b3480156106f957600080fd5b50610702611a57565b60405161070f9190613349565b60405180910390f35b34801561072457600080fd5b5061072d611a7d565b60405161073a9190613373565b60405180910390f35b34801561074f57600080fd5b5061076a60048036038101906107659190613276565b611a83565b005b6107866004803603810190610781919061389b565b611bd3565b005b34801561079457600080fd5b506107af60048036038101906107aa91906132d9565b611f63565b6040516107bc919061331f565b60405180910390f35b3480156107d157600080fd5b506107ec60048036038101906107e791906132d9565b611f7b565b6040516107f9919061397e565b60405180910390f35b34801561080e57600080fd5b5061082960048036038101906108249190613741565b612055565b005b34801561083757600080fd5b506108406126c6565b60405161084d9190613349565b60405180910390f35b34801561086257600080fd5b5061087d60048036038101906108789190613276565b6126ec565b005b34801561088b57600080fd5b50610894612826565b6040516108a1919061331f565b60405180910390f35b3480156108b657600080fd5b506108bf61282c565b6040516108cc9190613373565b60405180910390f35b3480156108e157600080fd5b506108fc60048036038101906108f791906132d9565b612832565b604051610909919061397e565b60405180910390f35b34801561091e57600080fd5b50610939600480360381019061093491906132d9565b612914565b6040516109469190613373565b60405180910390f35b34801561095b57600080fd5b5061097660048036038101906109719190613276565b612945565b604051610983919061355e565b60405180910390f35b34801561099857600080fd5b506109a1612965565b6040516109ae9190613373565b60405180910390f35b3480156109c357600080fd5b506109cc612981565b6040516109d99190613373565b60405180910390f35b3480156109ee57600080fd5b506109f7612a02565b604051610a049190613373565b60405180910390f35b348015610a1957600080fd5b50610a346004803603810190610a2f9190613ae7565b612a08565b005b348015610a4257600080fd5b50610a5d6004803603810190610a589190613623565b612f34565b005b348015610a6b57600080fd5b50610a74613022565b604051610a819190613349565b60405180910390f35b348015610a9657600080fd5b50610ab16004803603810190610aac9190613276565b613048565b005b348015610abf57600080fd5b50610ac8613198565b604051610ad5919061331f565b60405180910390f35b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610b6e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b6590613b87565b60405180910390fd5b6000600e60008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff0219169083151502179055508073ffffffffffffffffffffffffffffffffffffffff167f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a6000604051610c0d919061355e565b60405180910390a250565b6000814090506000801b8103610c5a576040517f84c0686400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80600f6000848152602001908152602001600020819055505050565b600a5481565b600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d32576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d2990613b87565b60405180910390fd5b601060009054906101000a900460ff1615610d82576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d7990613c19565b60405180910390fd5b806008819055506001601060006101000a81548160ff021916908315150217905550600115157f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a82604051610dd79190613373565b60405180910390a250565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610e72576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e6990613b87565b60405180910390fd5b60008111610eb5576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610eac90613cab565b60405180910390fd5b7fc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b260045482604051610ee8929190613ccb565b60405180910390a18060048190555050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f8a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f8190613b87565b60405180910390fd5b80601360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508073ffffffffffffffffffffffffffffffffffffffff167f73702180ce348e07b058846d1745c99987ae6c741ff97ec28d4539530ef1e8f160405160405180910390a250565b60115481565b6000806003805490501461108f57600360016003805490506110399190613d23565b8154811061104a57611049613d57565b5b906000526020600020906002020160010160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16611093565b6001545b905090565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611128576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161111f90613b87565b60405180910390fd5b6000801b840361116d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161116490613df8565b60405180910390fd5b6111b16012600086815260200190815260200160002060405180606001604052908160008201548152602001600182015481526020016002820154815250506112d0565b156111f1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016111e890613e8a565b60405180910390fd5b60006040518060600160405280848152602001838152602001858152509050611219816112d0565b611258576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161124f90613f1c565b60405180910390fd5b8060126000878152602001908152602001600020600082015181600001556020820151816001015560408201518160020155905050847fea0123c726a665cb0ab5691444f929a7056c7a7709c60c0587829e8046b8d5148484876040516112c193929190613650565b60405180910390a25050505050565b60008060001b8260000151141580156112f057506000801b826020015114155b801561130357506000801b826040015114155b9050919050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461139a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161139190613b87565b60405180910390fd5b601060009054906101000a900460ff166113e9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016113e090613fae565b60405180910390fd5b806008819055506000601060006101000a81548160ff021916908315150217905550600015157f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a8260405161143e9190613373565b60405180910390a250565b600660009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6040518060400160405280600681526020017f76332e302e30000000000000000000000000000000000000000000000000000081525081565b601060009054906101000a900460ff1681565b600060016003805490506114cf9190613d23565b905090565b60126020528060005260406000206000915090508060000154908060010154908060020154905083565b6000600380549050905090565b600c5481565b60015481565b6000601060009054906101000a900460ff1615611569576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161156090613c19565b60405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036115fa576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016115f190614040565b60405180910390fd5b6001601360146101000a81548160ff021916908315150217905550601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166382ecf2f6346006898989888e8b604051602001611670959493929190614131565b6040516020818303038152906040526040518563ffffffff1660e01b815260040161169d93929190614238565b60206040518083038185885af11580156116bb573d6000803e3d6000fd5b50505050506040513d601f19601f820116820180604052508101906116e091906142b4565b90506000601360146101000a81548160ff0219169083151502179055509695505050505050565b6000611711611017565b821115611753576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161174a90614379565b60405180910390fd5b60006003805490501161179b576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161179290614431565b60405180910390fd5b60008060038054905090505b80821015611844576000600282846117bf9190614451565b6117c991906144d6565b905084600382815481106117e0576117df613d57565b5b906000526020600020906002020160010160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16101561183a576001816118339190614451565b925061183e565b8091505b506117a7565b8192505050919050565b600381565b60025481565b600660009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118e9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118e090614579565b60405180910390fd5b6000811161192c576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119239061460b565b60405180910390fd5b6003805490508110611973576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161196a906146c3565b60405180910390fd5b6008546003828154811061198a57611989613d57565b5b906000526020600020906002020160010160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16426119d59190613d23565b10611a15576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a0c9061477b565b60405180910390fd5b6000611a1f6114fe565b90508160035581817f4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b660405160405180910390a35050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60055481565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611b13576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611b0a90613b87565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff16600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff167f0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc60405160405180910390a380600b60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b601060009054906101000a900460ff16611c22576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c1990613fae565b60405180910390fd5b600e60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff1680611cc35750600e60008073ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff165b611d02576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cf99061480d565b60405180910390fd5b611d0a612965565b8314611d4b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d42906148c5565b60405180910390fd5b42611d5584612914565b10611d95576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d8c90614957565b60405180910390fd5b6000801b8403611dda576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611dd1906149e9565b60405180910390fd5b6000801b8214611e285781814014611e27576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e1e90614aa1565b60405180910390fd5b5b82611e316114fe565b857fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e242604051611e619190613373565b60405180910390a460036040518060600160405280868152602001426fffffffffffffffffffffffffffffffff168152602001856fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff160217905550505050505050565b600f6020528060005260406000206000915090505481565b611f836131bc565b60038281548110611f9757611f96613d57565b5b9060005260206000209060020201604051806060016040529081600082015481526020016001820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016001820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250509050919050565b601060009054906101000a900460ff16156120a5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161209c90613c19565b60405180910390fd5b600e60003273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16806121465750600e60008073ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff165b806121645750601154612157612981565b426121629190613d23565b115b6121a3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161219a9061480d565b60405180910390fd5b6121ab612965565b8410156121ed576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016121e490614b59565b60405180910390fd5b426121f785612914565b10612237576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161222e90614957565b60405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146122e157601360149054906101000a900460ff166122dc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016122d390614c37565b60405180910390fd5b612332565b601360149054906101000a900460ff1615612331576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161232890614d15565b60405180910390fd5b5b6000801b8503612377576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161236e906149e9565b60405180910390fd5b600060126000888152602001908152602001600020604051806060016040529081600082015481526020016001820154815260200160028201548152505090506123c0816112d0565b6123ff576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016123f690614da7565b60405180910390fd5b6000600f60008681526020019081526020016000205490506000801b8103612453576040517f22aa3a9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006040518060e0016040528083815260200160036124706114bb565b8154811061248157612480613d57565b5b906000526020600020906002020160000154815260200189815260200188815260200184604001518152602001846020015181526020018573ffffffffffffffffffffffffffffffffffffffff168152509050600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166341493c608460000151836040516020016125289190614e73565b604051602081830303815290604052886040518463ffffffff1660e01b815260040161255693929190614e8e565b60006040518083038186803b15801561256e57600080fd5b505afa158015612582573d6000803e3d6000fd5b505050508661258f6114fe565b897fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2426040516125bf9190613373565b60405180910390a4600360405180606001604052808a8152602001426fffffffffffffffffffffffffffffffff168152602001896fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055505050505050505050505050565b600760009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461277c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161277390613b87565b60405180910390fd5b6001600e60008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff0219169083151502179055508073ffffffffffffffffffffffffffffffffffffffff167f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a600160405161281b919061355e565b60405180910390a250565b60095481565b60085481565b61283a6131bc565b600361284583611707565b8154811061285657612855613d57565b5b9060005260206000209060020201604051806060016040529081600082015481526020016001820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016001820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250509050919050565b6000600554600154836129279190613d23565b6129319190614ed3565b60025461293e9190614451565b9050919050565b600e6020528060005260406000206000915054906101000a900460ff1681565b6000600454612972611017565b61297c9190614451565b905090565b600080600380549050146129f957600360016003805490506129a39190613d23565b815481106129b4576129b3613d57565b5b906000526020600020906002020160010160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166129fd565b6002545b905090565b60045481565b6003600060019054906101000a900460ff16158015612a3957508060ff1660008054906101000a900460ff1660ff16105b612a78576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612a6f90614f9f565b60405180910390fd5b806000806101000a81548160ff021916908360ff1602179055506001600060016101000a81548160ff021916908315150217905550600082610160015111612af5576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612aec90613cab565b60405180910390fd5b6000826080015111612b3c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612b3390615031565b60405180910390fd5b428261014001511115612b84576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612b7b906150e9565b60405180910390fd5b8161016001516004819055508160800151600581905550600060038054905003612cc4576003604051806060016040528084610100015181526020018461014001516fffffffffffffffffffffffffffffffff1681526020018461012001516fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555050508161012001516001819055508161014001516002819055505b8160000151600660006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555081606001516008819055506001600e6000846020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff021916908315150217905550816101a0015160118190555060405180606001604052808360a0015181526020018360c0015181526020018360e00151815250601260007fae8304f40f7123e0c87b97f8a600e94ff3a3a25be588fc66b8a3717c8959ce778152602001908152602001600020600082015181600001556020820151816001015560408201518160020155905050816101800151600b60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508160400151600d60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506000601360146101000a81548160ff0219169083151502179055506000601360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060008060016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249881604051612f289190613880565b60405180910390a15050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612fc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612fbb90613b87565b60405180910390fd5b60126000828152602001908152602001600020600080820160009055600182016000905560028201600090555050807f4432b02a2fcbed48d94e8d72723e155c6690e4b7f39afa41a2a8ff8c0aa425da60405160405180910390a250565b601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146130d8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016130cf90613b87565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff16600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a380600d60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b7fae8304f40f7123e0c87b97f8a600e94ff3a3a25be588fc66b8a3717c8959ce7781565b60405180606001604052806000801916815260200160006fffffffffffffffffffffffffffffffff16815260200160006fffffffffffffffffffffffffffffffff1681525090565b6000604051905090565b600080fd5b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b600061324382613218565b9050919050565b61325381613238565b811461325e57600080fd5b50565b6000813590506132708161324a565b92915050565b60006020828403121561328c5761328b61320e565b5b600061329a84828501613261565b91505092915050565b6000819050919050565b6132b6816132a3565b81146132c157600080fd5b50565b6000813590506132d3816132ad565b92915050565b6000602082840312156132ef576132ee61320e565b5b60006132fd848285016132c4565b91505092915050565b6000819050919050565b61331981613306565b82525050565b60006020820190506133346000830184613310565b92915050565b61334381613238565b82525050565b600060208201905061335e600083018461333a565b92915050565b61336d816132a3565b82525050565b60006020820190506133886000830184613364565b92915050565b61339781613306565b81146133a257600080fd5b50565b6000813590506133b48161338e565b92915050565b600080600080608085870312156133d4576133d361320e565b5b60006133e2878288016133a5565b94505060206133f3878288016133a5565b9350506040613404878288016133a5565b9250506060613415878288016133a5565b91505092959194509250565b600080fd5b6000601f19601f8301169050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b61346f82613426565b810181811067ffffffffffffffff8211171561348e5761348d613437565b5b80604052505050565b60006134a1613204565b90506134ad8282613466565b919050565b6000606082840312156134c8576134c7613421565b5b6134d26060613497565b905060006134e2848285016133a5565b60008301525060206134f6848285016133a5565b602083015250604061350a848285016133a5565b60408301525092915050565b60006060828403121561352c5761352b61320e565b5b600061353a848285016134b2565b91505092915050565b60008115159050919050565b61355881613543565b82525050565b6000602082019050613573600083018461354f565b92915050565b600081519050919050565b600082825260208201905092915050565b60005b838110156135b3578082015181840152602081019050613598565b838111156135c2576000848401525b50505050565b60006135d382613579565b6135dd8185613584565b93506135ed818560208601613595565b6135f681613426565b840191505092915050565b6000602082019050818103600083015261361b81846135c8565b905092915050565b6000602082840312156136395761363861320e565b5b6000613647848285016133a5565b91505092915050565b60006060820190506136656000830186613310565b6136726020830185613310565b61367f6040830184613310565b949350505050565b600080fd5b600080fd5b600067ffffffffffffffff8211156136ac576136ab613437565b5b6136b582613426565b9050602081019050919050565b82818337600083830152505050565b60006136e46136df84613691565b613497565b905082815260208101848484011115613700576136ff61368c565b5b61370b8482856136c2565b509392505050565b600082601f83011261372857613727613687565b5b81356137388482602086016136d1565b91505092915050565b60008060008060008060c0878903121561375e5761375d61320e565b5b600061376c89828a016133a5565b965050602061377d89828a016133a5565b955050604061378e89828a016132c4565b945050606061379f89828a016132c4565b935050608087013567ffffffffffffffff8111156137c0576137bf613213565b5b6137cc89828a01613713565b92505060a06137dd89828a01613261565b9150509295509295509295565b6000819050919050565b600061380f61380a61380584613218565b6137ea565b613218565b9050919050565b6000613821826137f4565b9050919050565b600061383382613816565b9050919050565b61384381613828565b82525050565b600060208201905061385e600083018461383a565b92915050565b600060ff82169050919050565b61387a81613864565b82525050565b60006020820190506138956000830184613871565b92915050565b600080600080608085870312156138b5576138b461320e565b5b60006138c3878288016133a5565b94505060206138d4878288016132c4565b93505060406138e5878288016133a5565b92505060606138f6878288016132c4565b91505092959194509250565b61390b81613306565b82525050565b60006fffffffffffffffffffffffffffffffff82169050919050565b61393681613911565b82525050565b6060820160008201516139526000850182613902565b506020820151613965602085018261392d565b506040820151613978604085018261392d565b50505050565b6000606082019050613993600083018461393c565b92915050565b60006101c082840312156139b0576139af613421565b5b6139bb6101c0613497565b905060006139cb84828501613261565b60008301525060206139df84828501613261565b60208301525060406139f384828501613261565b6040830152506060613a07848285016132c4565b6060830152506080613a1b848285016132c4565b60808301525060a0613a2f848285016133a5565b60a08301525060c0613a43848285016133a5565b60c08301525060e0613a57848285016133a5565b60e083015250610100613a6c848285016133a5565b61010083015250610120613a82848285016132c4565b61012083015250610140613a98848285016132c4565b61014083015250610160613aae848285016132c4565b61016083015250610180613ac484828501613261565b610180830152506101a0613ada848285016132c4565b6101a08301525092915050565b60006101c08284031215613afe57613afd61320e565b5b6000613b0c84828501613999565b91505092915050565b7f4c324f75747075744f7261636c653a2063616c6c6572206973206e6f7420746860008201527f65206f776e657200000000000000000000000000000000000000000000000000602082015250565b6000613b71602783613584565b9150613b7c82613b15565b604082019050919050565b60006020820190508181036000830152613ba081613b64565b9050919050565b7f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f64652060008201527f697320656e61626c656400000000000000000000000000000000000000000000602082015250565b6000613c03602a83613584565b9150613c0e82613ba7565b604082019050919050565b60006020820190508181036000830152613c3281613bf6565b9050919050565b7f4c324f75747075744f7261636c653a207375626d697373696f6e20696e74657260008201527f76616c206d7573742062652067726561746572207468616e2030000000000000602082015250565b6000613c95603a83613584565b9150613ca082613c39565b604082019050919050565b60006020820190508181036000830152613cc481613c88565b9050919050565b6000604082019050613ce06000830185613364565b613ced6020830184613364565b9392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000613d2e826132a3565b9150613d39836132a3565b925082821015613d4c57613d4b613cf4565b5b828203905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4c324f75747075744f7261636c653a20636f6e666967206e616d652063616e6e60008201527f6f7420626520656d707479000000000000000000000000000000000000000000602082015250565b6000613de2602b83613584565b9150613ded82613d86565b604082019050919050565b60006020820190508181036000830152613e1181613dd5565b9050919050565b7f4c324f75747075744f7261636c653a20636f6e66696720616c7265616479206560008201527f7869737473000000000000000000000000000000000000000000000000000000602082015250565b6000613e74602583613584565b9150613e7f82613e18565b604082019050919050565b60006020820190508181036000830152613ea381613e67565b9050919050565b7f4c324f75747075744f7261636c653a20696e76616c6964204f5020537563636960008201527f6e637420636f6e66696775726174696f6e20706172616d657465727300000000602082015250565b6000613f06603c83613584565b9150613f1182613eaa565b604082019050919050565b60006020820190508181036000830152613f3581613ef9565b9050919050565b7f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f64652060008201527f6973206e6f7420656e61626c6564000000000000000000000000000000000000602082015250565b6000613f98602e83613584565b9150613fa382613f3c565b604082019050919050565b60006020820190508181036000830152613fc781613f8b565b9050919050565b7f4c324f75747075744f7261636c653a20646973707574652067616d652066616360008201527f746f7279206973206e6f74207365740000000000000000000000000000000000602082015250565b600061402a602f83613584565b915061403582613fce565b604082019050919050565b600060208201905081810360008301526140598161401d565b9050919050565b6000819050919050565b61407b614076826132a3565b614060565b82525050565b60008160601b9050919050565b600061409982614081565b9050919050565b60006140ab8261408e565b9050919050565b6140c36140be82613238565b6140a0565b82525050565b6000819050919050565b6140e46140df82613306565b6140c9565b82525050565b600081519050919050565b600081905092915050565b600061410b826140ea565b61411581856140f5565b9350614125818560208601613595565b80840191505092915050565b600061413d828861406a565b60208201915061414d828761406a565b60208201915061415d82866140b2565b60148201915061416d82856140d3565b60208201915061417d8284614100565b91508190509695505050505050565b600063ffffffff82169050919050565b60006141b76141b26141ad8461418c565b6137ea565b61418c565b9050919050565b6141c78161419c565b82525050565b60006141d882613306565b9050919050565b6141e8816141cd565b82525050565b600082825260208201905092915050565b600061420a826140ea565b61421481856141ee565b9350614224818560208601613595565b61422d81613426565b840191505092915050565b600060608201905061424d60008301866141be565b61425a60208301856141df565b818103604083015261426c81846141ff565b9050949350505050565b600061428182613238565b9050919050565b61429181614276565b811461429c57600080fd5b50565b6000815190506142ae81614288565b92915050565b6000602082840312156142ca576142c961320e565b5b60006142d88482850161429f565b91505092915050565b7f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560008201527f7420666f72206120626c6f636b207468617420686173206e6f74206265656e2060208201527f70726f706f736564000000000000000000000000000000000000000000000000604082015250565b6000614363604883613584565b915061436e826142e1565b606082019050919050565b6000602082019050818103600083015261439281614356565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560008201527f74206173206e6f206f7574707574732068617665206265656e2070726f706f7360208201527f6564207965740000000000000000000000000000000000000000000000000000604082015250565b600061441b604683613584565b915061442682614399565b606082019050919050565b6000602082019050818103600083015261444a8161440e565b9050919050565b600061445c826132a3565b9150614467836132a3565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0382111561449c5761449b613cf4565b5b828201905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60006144e1826132a3565b91506144ec836132a3565b9250826144fc576144fb6144a7565b5b828204905092915050565b7f4c324f75747075744f7261636c653a206f6e6c7920746865206368616c6c656e60008201527f67657220616464726573732063616e2064656c657465206f7574707574730000602082015250565b6000614563603e83613584565b915061456e82614507565b604082019050919050565b6000602082019050818103600083015261459281614556565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c65746520676560008201527f6e65736973206f75747075740000000000000000000000000000000000000000602082015250565b60006145f5602c83613584565b915061460082614599565b604082019050919050565b60006020820190508181036000830152614624816145e8565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560008201527f747075747320616674657220746865206c6174657374206f757470757420696e60208201527f6465780000000000000000000000000000000000000000000000000000000000604082015250565b60006146ad604383613584565b91506146b88261462b565b606082019050919050565b600060208201905081810360008301526146dc816146a0565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560008201527f74707574732074686174206861766520616c7265616479206265656e2066696e60208201527f616c697a65640000000000000000000000000000000000000000000000000000604082015250565b6000614765604683613584565b9150614770826146e3565b606082019050919050565b6000602082019050818103600083015261479481614758565b9050919050565b7f4c324f75747075744f7261636c653a206f6e6c7920617070726f76656420707260008201527f6f706f736572732063616e2070726f706f7365206e6577206f75747075747300602082015250565b60006147f7603f83613584565b91506148028261479b565b604082019050919050565b60006020820190508181036000830152614826816147ea565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360008201527f7420626520657175616c20746f206e65787420657870656374656420626c6f6360208201527f6b206e756d626572000000000000000000000000000000000000000000000000604082015250565b60006148af604883613584565b91506148ba8261482d565b606082019050919050565b600060208201905081810360008301526148de816148a2565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f757470757420696e207468652066757475726500000000000000000000602082015250565b6000614941603683613584565b915061494c826148e5565b604082019050919050565b6000602082019050818103600083015261497081614934565b9050919050565b7f4c324f75747075744f7261636c653a204c32206f75747075742070726f706f7360008201527f616c2063616e6e6f7420626520746865207a65726f2068617368000000000000602082015250565b60006149d3603a83613584565b91506149de82614977565b604082019050919050565b60006020820190508181036000830152614a02816149c6565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206861736820646f65732060008201527f6e6f74206d61746368207468652068617368206174207468652065787065637460208201527f6564206865696768740000000000000000000000000000000000000000000000604082015250565b6000614a8b604983613584565b9150614a9682614a09565b606082019050919050565b60006020820190508181036000830152614aba81614a7e565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360008201527f742062652067726561746572207468616e206f7220657175616c20746f206e6560208201527f787420657870656374656420626c6f636b206e756d6265720000000000000000604082015250565b6000614b43605883613584565b9150614b4e82614ac1565b606082019050919050565b60006020820190508181036000830152614b7281614b36565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f75747075742066726f6d206f757473696465204469737075746547616d60208201527f65466163746f72792e637265617465207768696c65206469737075746547616d60408201527f65466163746f7279206973207365740000000000000000000000000000000000606082015250565b6000614c21606f83613584565b9150614c2c82614b79565b608082019050919050565b60006020820190508181036000830152614c5081614c14565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f75747075742066726f6d20696e73696465204469737075746547616d6560208201527f466163746f72792e63726561746520776974686f75742073657474696e67206460408201527f69737075746547616d65466163746f7279000000000000000000000000000000606082015250565b6000614cff607183613584565b9150614d0a82614c57565b608082019050919050565b60006020820190508181036000830152614d2e81614cf2565b9050919050565b7f4c324f75747075744f7261636c653a20696e76616c6964204f5020537563636960008201527f6e637420636f6e66696775726174696f6e000000000000000000000000000000602082015250565b6000614d91603183613584565b9150614d9c82614d35565b604082019050919050565b60006020820190508181036000830152614dc081614d84565b9050919050565b614dd0816132a3565b82525050565b614ddf81613238565b82525050565b60e082016000820151614dfb6000850182613902565b506020820151614e0e6020850182613902565b506040820151614e216040850182613902565b506060820151614e346060850182614dc7565b506080820151614e476080850182613902565b5060a0820151614e5a60a0850182613902565b5060c0820151614e6d60c0850182614dd6565b50505050565b600060e082019050614e886000830184614de5565b92915050565b6000606082019050614ea36000830186613310565b8181036020830152614eb581856141ff565b90508181036040830152614ec981846141ff565b9050949350505050565b6000614ede826132a3565b9150614ee9836132a3565b9250817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0483118215151615614f2257614f21613cf4565b5b828202905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160008201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b6000614f89602e83613584565b9150614f9482614f2d565b604082019050919050565b60006020820190508181036000830152614fb881614f7c565b9050919050565b7f4c324f75747075744f7261636c653a204c3220626c6f636b2074696d65206d7560008201527f73742062652067726561746572207468616e2030000000000000000000000000602082015250565b600061501b603483613584565b915061502682614fbf565b604082019050919050565b6000602082019050818103600083015261504a8161500e565b9050919050565b7f4c324f75747075744f7261636c653a207374617274696e67204c322074696d6560008201527f7374616d70206d757374206265206c657373207468616e2063757272656e742060208201527f74696d6500000000000000000000000000000000000000000000000000000000604082015250565b60006150d3604483613584565b91506150de82615051565b606082019050919050565b60006020820190508181036000830152615102816150c6565b905091905056fea2646970667358221220f1b1fc1306af82a760d2c9e40e19b4cba6fb0476008a2b853da237a3e6c072ad64736f6c634300080f0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\"b\0\0(` \x1B` \x1CV[b\0\x01\xD3V[`\0`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0{W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0r\x90b\0\x01vV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16`\0\x80T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10\x15b\0\0\xEDW`\xFF`\0\x80a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\xFF`@Qb\0\0\xE4\x91\x90b\0\x01\xB6V[`@Q\x80\x91\x03\x90\xA1[V[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is initi`\0\x82\x01R\x7Falizing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0b\0\x01^`'\x83b\0\0\xEFV[\x91Pb\0\x01k\x82b\0\x01\0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Rb\0\x01\x91\x81b\0\x01OV[\x90P\x91\x90PV[`\0`\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xB0\x81b\0\x01\x98V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x01\xCD`\0\x83\x01\x84b\0\x01\xA5V[\x92\x91PPV[aQ?\x80b\0\x01\xE3`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x88W`\x005`\xE0\x1C\x80c\x88xbr\x11a\x01ZW\x80c\xCE]\xB8\xD6\x11a\0\xC1W\x80c\xE1\xA4\x1B\xCF\x11a\0zW\x80c\xE1\xA4\x1B\xCF\x14a\t\xE2W\x80c\xE4\x0Bz\x12\x14a\n\rW\x80c\xEC[.:\x14a\n6W\x80c\xF2\xB4\xE6\x17\x14a\n_W\x80c\xF2\xFD\xE3\x8B\x14a\n\x8AW\x80c\xF7/`m\x14a\n\xB3Wa\x02\x88V[\x80c\xCE]\xB8\xD6\x14a\x08\xAAW\x80c\xCF\x8E\\\xF0\x14a\x08\xD5W\x80c\xD1\xDE\x85l\x14a\t\x12W\x80c\xD4e\x12v\x14a\tOW\x80c\xDC\xEC3H\x14a\t\x8CW\x80c\xE0\xC2\xF95\x14a\t\xB7Wa\x02\x88V[\x80c\xA1\x96\xB5%\x11a\x01\x13W\x80c\xA1\x96\xB5%\x14a\x07\x88W\x80c\xA2Z\xE5W\x14a\x07\xC5W\x80c\xA4\xEE\x9D{\x14a\x08\x02W\x80c\xA8\xE4\xFB\x90\x14a\x08+W\x80c\xB0<\xD4\x18\x14a\x08VW\x80c\xC3.N>\x14a\x08\x7FWa\x02\x88V[\x80c\x88xbr\x14a\x06\x99W\x80c\x89\xC4L\xBB\x14a\x06\xC4W\x80c\x8D\xA5\xCB[\x14a\x06\xEDW\x80c\x93\x99\x1A\xF3\x14a\x07\x18W\x80c\x97\xFC\0|\x14a\x07CW\x80c\x9A\xAA\xB6H\x14a\x07lWa\x02\x88V[\x80cJ\xB3\t\xAC\x11a\x01\xFEW\x80cj\xBC\xF5c\x11a\x01\xB7W\x80cj\xBC\xF5c\x14a\x05\x80W\x80cm\x9A\x1C\x8B\x14a\x05\xABW\x80cp\x87*\xA5\x14a\x05\xD6W\x80czA\xA05\x14a\x06\x01W\x80c\x7F\0d \x14a\x061W\x80c\x7F\x01\xEAh\x14a\x06nWa\x02\x88V[\x80cJ\xB3\t\xAC\x14a\x04lW\x80cSM\xB0\xE2\x14a\x04\x95W\x80cT\xFDMP\x14a\x04\xC0W\x80c`\xCA\xF7\xA0\x14a\x04\xEBW\x80ci\xF1n\xEC\x14a\x05\x16W\x80cjVb\x0B\x14a\x05AWa\x02\x88V[\x80c3l\x9E\x81\x11a\x02PW\x80c3l\x9E\x81\x14a\x03^W\x80c4\x19\xD2\xC2\x14a\x03\x87W\x80cBw\xBC\x06\x14a\x03\xB0W\x80cE\x99\xC7\x88\x14a\x03\xDBW\x80cG\xC3~\x9C\x14a\x04\x06W\x80cI\x18^\x06\x14a\x04/Wa\x02\x88V[\x80c\t\xD62\xD3\x14a\x02\x8DW\x80c\x1E\x85h\0\x14a\x02\xB6W\x80c+1\x84\x1E\x14a\x02\xDFW\x80c+z\xC3\xF3\x14a\x03\nW\x80c,iya\x14a\x035W[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xB4`\x04\x806\x03\x81\x01\x90a\x02\xAF\x91\x90a2vV[a\n\xDEV[\0[4\x80\x15a\x02\xC2W`\0\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a2\xD9V[a\x0C\x18V[\0[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x02\xF4a\x0CvV[`@Qa\x03\x01\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03\x1Fa\x0C|V[`@Qa\x03,\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03AW`\0\x80\xFD[Pa\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a2\xD9V[a\x0C\xA2V[\0[4\x80\x15a\x03jW`\0\x80\xFD[Pa\x03\x85`\x04\x806\x03\x81\x01\x90a\x03\x80\x91\x90a2\xD9V[a\r\xE2V[\0[4\x80\x15a\x03\x93W`\0\x80\xFD[Pa\x03\xAE`\x04\x806\x03\x81\x01\x90a\x03\xA9\x91\x90a2vV[a\x0E\xFAV[\0[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x03\xC5a\x10\x11V[`@Qa\x03\xD2\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE7W`\0\x80\xFD[Pa\x03\xF0a\x10\x17V[`@Qa\x03\xFD\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x12W`\0\x80\xFD[Pa\x04-`\x04\x806\x03\x81\x01\x90a\x04(\x91\x90a3\xBAV[a\x10\x98V[\0[4\x80\x15a\x04;W`\0\x80\xFD[Pa\x04V`\x04\x806\x03\x81\x01\x90a\x04Q\x91\x90a5\x16V[a\x12\xD0V[`@Qa\x04c\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90a2\xD9V[a\x13\nV[\0[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x04\xAAa\x14IV[`@Qa\x04\xB7\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xCCW`\0\x80\xFD[Pa\x04\xD5a\x14oV[`@Qa\x04\xE2\x91\x90a6\x01V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xF7W`\0\x80\xFD[Pa\x05\0a\x14\xA8V[`@Qa\x05\r\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x05+a\x14\xBBV[`@Qa\x058\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05MW`\0\x80\xFD[Pa\x05h`\x04\x806\x03\x81\x01\x90a\x05c\x91\x90a6#V[a\x14\xD4V[`@Qa\x05w\x93\x92\x91\x90a6PV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x8CW`\0\x80\xFD[Pa\x05\x95a\x14\xFEV[`@Qa\x05\xA2\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x05\xC0a\x15\x0BV[`@Qa\x05\xCD\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xE2W`\0\x80\xFD[Pa\x05\xEBa\x15\x11V[`@Qa\x05\xF8\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1B`\x04\x806\x03\x81\x01\x90a\x06\x16\x91\x90a7AV[a\x15\x17V[`@Qa\x06(\x91\x90a8IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06=W`\0\x80\xFD[Pa\x06X`\x04\x806\x03\x81\x01\x90a\x06S\x91\x90a2\xD9V[a\x17\x07V[`@Qa\x06e\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x06\x83a\x18NV[`@Qa\x06\x90\x91\x90a8\x80V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xA5W`\0\x80\xFD[Pa\x06\xAEa\x18SV[`@Qa\x06\xBB\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xD0W`\0\x80\xFD[Pa\x06\xEB`\x04\x806\x03\x81\x01\x90a\x06\xE6\x91\x90a2\xD9V[a\x18YV[\0[4\x80\x15a\x06\xF9W`\0\x80\xFD[Pa\x07\x02a\x1AWV[`@Qa\x07\x0F\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07$W`\0\x80\xFD[Pa\x07-a\x1A}V[`@Qa\x07:\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07OW`\0\x80\xFD[Pa\x07j`\x04\x806\x03\x81\x01\x90a\x07e\x91\x90a2vV[a\x1A\x83V[\0[a\x07\x86`\x04\x806\x03\x81\x01\x90a\x07\x81\x91\x90a8\x9BV[a\x1B\xD3V[\0[4\x80\x15a\x07\x94W`\0\x80\xFD[Pa\x07\xAF`\x04\x806\x03\x81\x01\x90a\x07\xAA\x91\x90a2\xD9V[a\x1FcV[`@Qa\x07\xBC\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\xD1W`\0\x80\xFD[Pa\x07\xEC`\x04\x806\x03\x81\x01\x90a\x07\xE7\x91\x90a2\xD9V[a\x1F{V[`@Qa\x07\xF9\x91\x90a9~V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\x0EW`\0\x80\xFD[Pa\x08)`\x04\x806\x03\x81\x01\x90a\x08$\x91\x90a7AV[a UV[\0[4\x80\x15a\x087W`\0\x80\xFD[Pa\x08@a&\xC6V[`@Qa\x08M\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08bW`\0\x80\xFD[Pa\x08}`\x04\x806\x03\x81\x01\x90a\x08x\x91\x90a2vV[a&\xECV[\0[4\x80\x15a\x08\x8BW`\0\x80\xFD[Pa\x08\x94a(&V[`@Qa\x08\xA1\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xB6W`\0\x80\xFD[Pa\x08\xBFa(,V[`@Qa\x08\xCC\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xE1W`\0\x80\xFD[Pa\x08\xFC`\x04\x806\x03\x81\x01\x90a\x08\xF7\x91\x90a2\xD9V[a(2V[`@Qa\t\t\x91\x90a9~V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x1EW`\0\x80\xFD[Pa\t9`\x04\x806\x03\x81\x01\x90a\t4\x91\x90a2\xD9V[a)\x14V[`@Qa\tF\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t[W`\0\x80\xFD[Pa\tv`\x04\x806\x03\x81\x01\x90a\tq\x91\x90a2vV[a)EV[`@Qa\t\x83\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x98W`\0\x80\xFD[Pa\t\xA1a)eV[`@Qa\t\xAE\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xC3W`\0\x80\xFD[Pa\t\xCCa)\x81V[`@Qa\t\xD9\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xEEW`\0\x80\xFD[Pa\t\xF7a*\x02V[`@Qa\n\x04\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x19W`\0\x80\xFD[Pa\n4`\x04\x806\x03\x81\x01\x90a\n/\x91\x90a:\xE7V[a*\x08V[\0[4\x80\x15a\nBW`\0\x80\xFD[Pa\n]`\x04\x806\x03\x81\x01\x90a\nX\x91\x90a6#V[a/4V[\0[4\x80\x15a\nkW`\0\x80\xFD[Pa\nta0\"V[`@Qa\n\x81\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x96W`\0\x80\xFD[Pa\n\xB1`\x04\x806\x03\x81\x01\x90a\n\xAC\x91\x90a2vV[a0HV[\0[4\x80\x15a\n\xBFW`\0\x80\xFD[Pa\n\xC8a1\x98V[`@Qa\n\xD5\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Be\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0`\x0E`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F]\xF3\x8D9^\xDC\x15\xB6i\xD6FV\x9B\xD0\x15Q3\x95\x07\x0B[M\xEB\x8A\x160\n\xBB\x06\r\x1BZ`\0`@Qa\x0C\r\x91\x90a5^V[`@Q\x80\x91\x03\x90\xA2PV[`\0\x81@\x90P`\0\x80\x1B\x81\x03a\x0CZW`@Q\x7F\x84\xC0hd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x0F`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPPPV[`\nT\x81V[`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r)\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\r\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ry\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x81\x90UP`\x01`\x10`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x15\x15\x7F\x1F\\\x87/\x1E\xA9<W\xE41\x12\xEAD\x9E\xE1\x9E\xF5uD\x88\xB8v'\xB4\xC5$V\xB0\xE5\xA4\x10\x9A\x82`@Qa\r\xD7\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA2PV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0ErW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ei\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x11a\x0E\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xAC\x90a<\xABV[`@Q\x80\x91\x03\x90\xFD[\x7F\xC1\xBF\x9A\xBF\xB5~\xA0\x1E\xD9\xEC\xB4\xF4^\x9C\xEF\xA7\xBAD\xB2\xE6w\x8C<\xE7(\x14\t\x99\x9F\x1A\xF1\xB2`\x04T\x82`@Qa\x0E\xE8\x92\x91\x90a<\xCBV[`@Q\x80\x91\x03\x90\xA1\x80`\x04\x81\x90UPPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x81\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80`\x13`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fsp!\x80\xCE4\x8E\x07\xB0X\x84m\x17E\xC9\x99\x87\xAElt\x1F\xF9~\xC2\x8DE9S\x0E\xF1\xE8\xF1`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x11T\x81V[`\0\x80`\x03\x80T\x90P\x14a\x10\x8FW`\x03`\x01`\x03\x80T\x90Pa\x109\x91\x90a=#V[\x81T\x81\x10a\x10JWa\x10Ia=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10\x93V[`\x01T[\x90P\x90V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11(W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x1F\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x84\x03a\x11mW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11d\x90a=\xF8V[`@Q\x80\x91\x03\x90\xFD[a\x11\xB1`\x12`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPPa\x12\xD0V[\x15a\x11\xF1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xE8\x90a>\x8AV[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x85\x81RP\x90Pa\x12\x19\x81a\x12\xD0V[a\x12XW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12O\x90a?\x1CV[`@Q\x80\x91\x03\x90\xFD[\x80`\x12`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U\x90PP\x84\x7F\xEA\x01#\xC7&\xA6e\xCB\n\xB5i\x14D\xF9)\xA7\x05lzw\t\xC6\x0C\x05\x87\x82\x9E\x80F\xB8\xD5\x14\x84\x84\x87`@Qa\x12\xC1\x93\x92\x91\x90a6PV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x80`\0\x1B\x82`\0\x01Q\x14\x15\x80\x15a\x12\xF0WP`\0\x80\x1B\x82` \x01Q\x14\x15[\x80\x15a\x13\x03WP`\0\x80\x1B\x82`@\x01Q\x14\x15[\x90P\x91\x90PV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x91\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x13\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xE0\x90a?\xAEV[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x81\x90UP`\0`\x10`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x15\x15\x7F\x1F\\\x87/\x1E\xA9<W\xE41\x12\xEAD\x9E\xE1\x9E\xF5uD\x88\xB8v'\xB4\xC5$V\xB0\xE5\xA4\x10\x9A\x82`@Qa\x14>\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA2PV[`\x06`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fv3.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x01`\x03\x80T\x90Pa\x14\xCF\x91\x90a=#V[\x90P\x90V[`\x12` R\x80`\0R`@`\0 `\0\x91P\x90P\x80`\0\x01T\x90\x80`\x01\x01T\x90\x80`\x02\x01T\x90P\x83V[`\0`\x03\x80T\x90P\x90P\x90V[`\x0CT\x81V[`\x01T\x81V[`\0`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15`\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xF1\x90a@@V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x82\xEC\xF2\xF64`\x06\x89\x89\x89\x88\x8E\x8B`@Q` \x01a\x16p\x95\x94\x93\x92\x91\x90aA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x9D\x93\x92\x91\x90aB8V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x16\xBBW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE0\x91\x90aB\xB4V[\x90P`\0`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x96\x95PPPPPPV[`\0a\x17\x11a\x10\x17V[\x82\x11\x15a\x17SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17J\x90aCyV[`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x80T\x90P\x11a\x17\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x92\x90aD1V[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\x03\x80T\x90P\x90P[\x80\x82\x10\x15a\x18DW`\0`\x02\x82\x84a\x17\xBF\x91\x90aDQV[a\x17\xC9\x91\x90aD\xD6V[\x90P\x84`\x03\x82\x81T\x81\x10a\x17\xE0Wa\x17\xDFa=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x18:W`\x01\x81a\x183\x91\x90aDQV[\x92Pa\x18>V[\x80\x91P[Pa\x17\xA7V[\x81\x92PPP\x91\x90PV[`\x03\x81V[`\x02T\x81V[`\x06`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xE0\x90aEyV[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x11a\x19,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19#\x90aF\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80T\x90P\x81\x10a\x19sW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19j\x90aF\xC3V[`@Q\x80\x91\x03\x90\xFD[`\x08T`\x03\x82\x81T\x81\x10a\x19\x8AWa\x19\x89a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x19\xD5\x91\x90a=#V[\x10a\x1A\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x0C\x90aG{V[`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x1Fa\x14\xFEV[\x90P\x81`\x03U\x81\x81\x7FN\xE3z\xC2\xC7\x86\xEC\x85\xE8u\x92\xD3\xC5\xC8\xA1\xDDf\xF8Im\xDA?\x12]\x9E\xA8\xCA_ev)\xB6`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x05T\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02CT\x9A\x92\xB2A/z<\xAFz.V\xD6[\x88!\xB9\x13E6?\xAA_W\x19S\x84\x06_\xCC`@Q`@Q\x80\x91\x03\x90\xA3\x80`\x0B`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1C\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x19\x90a?\xAEV[`@Q\x80\x91\x03\x90\xFD[`\x0E`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1C\xC3WP`\x0E`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[a\x1D\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xF9\x90aH\rV[`@Q\x80\x91\x03\x90\xFD[a\x1D\na)eV[\x83\x14a\x1DKW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DB\x90aH\xC5V[`@Q\x80\x91\x03\x90\xFD[Ba\x1DU\x84a)\x14V[\x10a\x1D\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x8C\x90aIWV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x84\x03a\x1D\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xD1\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x82\x14a\x1E(W\x81\x81@\x14a\x1E'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x1E\x90aJ\xA1V[`@Q\x80\x91\x03\x90\xFD[[\x82a\x1E1a\x14\xFEV[\x85\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa\x1Ea\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA4`\x03`@Q\x80``\x01`@R\x80\x86\x81R` \x01Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[`\x0F` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[a\x1F\x83a1\xBCV[`\x03\x82\x81T\x81\x10a\x1F\x97Wa\x1F\x96a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a \xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x9C\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[`\x0E`\x002s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a!FWP`\x0E`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x80a!dWP`\x11Ta!Wa)\x81V[Ba!b\x91\x90a=#V[\x11[a!\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x9A\x90aH\rV[`@Q\x80\x91\x03\x90\xFD[a!\xABa)eV[\x84\x10\x15a!\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\xE4\x90aKYV[`@Q\x80\x91\x03\x90\xFD[Ba!\xF7\x85a)\x14V[\x10a\"7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\".\x90aIWV[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\"\xE1W`\x13`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\"\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xD3\x90aL7V[`@Q\x80\x91\x03\x90\xFD[a#2V[`\x13`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a#1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#(\x90aM\x15V[`@Q\x80\x91\x03\x90\xFD[[`\0\x80\x1B\x85\x03a#wW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#n\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[`\0`\x12`\0\x88\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90Pa#\xC0\x81a\x12\xD0V[a#\xFFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xF6\x90aM\xA7V[`@Q\x80\x91\x03\x90\xFD[`\0`\x0F`\0\x86\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x03a$SW`@Q\x7F\"\xAA:\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xE0\x01`@R\x80\x83\x81R` \x01`\x03a$pa\x14\xBBV[\x81T\x81\x10a$\x81Wa$\x80a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x84`@\x01Q\x81R` \x01\x84` \x01Q\x81R` \x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cAI<`\x84`\0\x01Q\x83`@Q` \x01a%(\x91\x90aNsV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x88`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%V\x93\x92\x91\x90aN\x8EV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a%nW`\0\x80\xFD[PZ\xFA\x15\x80\x15a%\x82W=`\0\x80>=`\0\xFD[PPPP\x86a%\x8Fa\x14\xFEV[\x89\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa%\xBF\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA4`\x03`@Q\x80``\x01`@R\x80\x8A\x81R` \x01Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPPPPPPV[`\x07`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'|W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a's\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0E`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F]\xF3\x8D9^\xDC\x15\xB6i\xD6FV\x9B\xD0\x15Q3\x95\x07\x0B[M\xEB\x8A\x160\n\xBB\x06\r\x1BZ`\x01`@Qa(\x1B\x91\x90a5^V[`@Q\x80\x91\x03\x90\xA2PV[`\tT\x81V[`\x08T\x81V[a(:a1\xBCV[`\x03a(E\x83a\x17\x07V[\x81T\x81\x10a(VWa(Ua=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\0`\x05T`\x01T\x83a)'\x91\x90a=#V[a)1\x91\x90aN\xD3V[`\x02Ta)>\x91\x90aDQV[\x90P\x91\x90PV[`\x0E` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x04Ta)ra\x10\x17V[a)|\x91\x90aDQV[\x90P\x90V[`\0\x80`\x03\x80T\x90P\x14a)\xF9W`\x03`\x01`\x03\x80T\x90Pa)\xA3\x91\x90a=#V[\x81T\x81\x10a)\xB4Wa)\xB3a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xFDV[`\x02T[\x90P\x90V[`\x04T\x81V[`\x03`\0`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a*9WP\x80`\xFF\x16`\0\x80T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[a*xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*o\x90aO\x9FV[`@Q\x80\x91\x03\x90\xFD[\x80`\0\x80a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\x01`\0`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x82a\x01`\x01Q\x11a*\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xEC\x90a<\xABV[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x80\x01Q\x11a+<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+3\x90aP1V[`@Q\x80\x91\x03\x90\xFD[B\x82a\x01@\x01Q\x11\x15a+\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+{\x90aP\xE9V[`@Q\x80\x91\x03\x90\xFD[\x81a\x01`\x01Q`\x04\x81\x90UP\x81`\x80\x01Q`\x05\x81\x90UP`\0`\x03\x80T\x90P\x03a,\xC4W`\x03`@Q\x80``\x01`@R\x80\x84a\x01\0\x01Q\x81R` \x01\x84a\x01@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84a\x01 \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP\x81a\x01 \x01Q`\x01\x81\x90UP\x81a\x01@\x01Q`\x02\x81\x90UP[\x81`\0\x01Q`\x06`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81``\x01Q`\x08\x81\x90UP`\x01`\x0E`\0\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81a\x01\xA0\x01Q`\x11\x81\x90UP`@Q\x80``\x01`@R\x80\x83`\xA0\x01Q\x81R` \x01\x83`\xC0\x01Q\x81R` \x01\x83`\xE0\x01Q\x81RP`\x12`\0\x7F\xAE\x83\x04\xF4\x0Fq#\xE0\xC8{\x97\xF8\xA6\0\xE9O\xF3\xA3\xA2[\xE5\x88\xFCf\xB8\xA3q|\x89Y\xCEw\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U\x90PP\x81a\x01\x80\x01Q`\x0B`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`@\x01Q`\r`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x13`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0\x80`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x81`@Qa/(\x91\x90a8\x80V[`@Q\x80\x91\x03\x90\xA1PPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xBB\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x12`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0\x90U`\x01\x82\x01`\0\x90U`\x02\x82\x01`\0\x90UPP\x80\x7FD2\xB0*/\xCB\xEDH\xD9N\x8Drr>\x15\\f\x90\xE4\xB7\xF3\x9A\xFAA\xA2\xA8\xFF\x8C\n\xA4%\xDA`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\xCF\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3\x80`\r`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[\x7F\xAE\x83\x04\xF4\x0Fq#\xE0\xC8{\x97\xF8\xA6\0\xE9O\xF3\xA3\xA2[\xE5\x88\xFCf\xB8\xA3q|\x89Y\xCEw\x81V[`@Q\x80``\x01`@R\x80`\0\x80\x19\x16\x81R` \x01`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a2C\x82a2\x18V[\x90P\x91\x90PV[a2S\x81a28V[\x81\x14a2^W`\0\x80\xFD[PV[`\0\x815\x90Pa2p\x81a2JV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2\x8CWa2\x8Ba2\x0EV[[`\0a2\x9A\x84\x82\x85\x01a2aV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a2\xB6\x81a2\xA3V[\x81\x14a2\xC1W`\0\x80\xFD[PV[`\0\x815\x90Pa2\xD3\x81a2\xADV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2\xEFWa2\xEEa2\x0EV[[`\0a2\xFD\x84\x82\x85\x01a2\xC4V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a3\x19\x81a3\x06V[\x82RPPV[`\0` \x82\x01\x90Pa34`\0\x83\x01\x84a3\x10V[\x92\x91PPV[a3C\x81a28V[\x82RPPV[`\0` \x82\x01\x90Pa3^`\0\x83\x01\x84a3:V[\x92\x91PPV[a3m\x81a2\xA3V[\x82RPPV[`\0` \x82\x01\x90Pa3\x88`\0\x83\x01\x84a3dV[\x92\x91PPV[a3\x97\x81a3\x06V[\x81\x14a3\xA2W`\0\x80\xFD[PV[`\0\x815\x90Pa3\xB4\x81a3\x8EV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3\xD4Wa3\xD3a2\x0EV[[`\0a3\xE2\x87\x82\x88\x01a3\xA5V[\x94PP` a3\xF3\x87\x82\x88\x01a3\xA5V[\x93PP`@a4\x04\x87\x82\x88\x01a3\xA5V[\x92PP``a4\x15\x87\x82\x88\x01a3\xA5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a4o\x82a4&V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\x8EWa4\x8Da47V[[\x80`@RPPPV[`\0a4\xA1a2\x04V[\x90Pa4\xAD\x82\x82a4fV[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a4\xC8Wa4\xC7a4!V[[a4\xD2``a4\x97V[\x90P`\0a4\xE2\x84\x82\x85\x01a3\xA5V[`\0\x83\x01RP` a4\xF6\x84\x82\x85\x01a3\xA5V[` \x83\x01RP`@a5\n\x84\x82\x85\x01a3\xA5V[`@\x83\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5,Wa5+a2\x0EV[[`\0a5:\x84\x82\x85\x01a4\xB2V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a5X\x81a5CV[\x82RPPV[`\0` \x82\x01\x90Pa5s`\0\x83\x01\x84a5OV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a5\xB3W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa5\x98V[\x83\x81\x11\x15a5\xC2W`\0\x84\x84\x01R[PPPPV[`\0a5\xD3\x82a5yV[a5\xDD\x81\x85a5\x84V[\x93Pa5\xED\x81\x85` \x86\x01a5\x95V[a5\xF6\x81a4&V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra6\x1B\x81\x84a5\xC8V[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a69Wa68a2\x0EV[[`\0a6G\x84\x82\x85\x01a3\xA5V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa6e`\0\x83\x01\x86a3\x10V[a6r` \x83\x01\x85a3\x10V[a6\x7F`@\x83\x01\x84a3\x10V[\x94\x93PPPPV[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\xACWa6\xABa47V[[a6\xB5\x82a4&V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a6\xE4a6\xDF\x84a6\x91V[a4\x97V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a7\0Wa6\xFFa6\x8CV[[a7\x0B\x84\x82\x85a6\xC2V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a7(Wa7'a6\x87V[[\x815a78\x84\x82` \x86\x01a6\xD1V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a7^Wa7]a2\x0EV[[`\0a7l\x89\x82\x8A\x01a3\xA5V[\x96PP` a7}\x89\x82\x8A\x01a3\xA5V[\x95PP`@a7\x8E\x89\x82\x8A\x01a2\xC4V[\x94PP``a7\x9F\x89\x82\x8A\x01a2\xC4V[\x93PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xC0Wa7\xBFa2\x13V[[a7\xCC\x89\x82\x8A\x01a7\x13V[\x92PP`\xA0a7\xDD\x89\x82\x8A\x01a2aV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81\x90P\x91\x90PV[`\0a8\x0Fa8\na8\x05\x84a2\x18V[a7\xEAV[a2\x18V[\x90P\x91\x90PV[`\0a8!\x82a7\xF4V[\x90P\x91\x90PV[`\0a83\x82a8\x16V[\x90P\x91\x90PV[a8C\x81a8(V[\x82RPPV[`\0` \x82\x01\x90Pa8^`\0\x83\x01\x84a8:V[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a8z\x81a8dV[\x82RPPV[`\0` \x82\x01\x90Pa8\x95`\0\x83\x01\x84a8qV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a8\xB5Wa8\xB4a2\x0EV[[`\0a8\xC3\x87\x82\x88\x01a3\xA5V[\x94PP` a8\xD4\x87\x82\x88\x01a2\xC4V[\x93PP`@a8\xE5\x87\x82\x88\x01a3\xA5V[\x92PP``a8\xF6\x87\x82\x88\x01a2\xC4V[\x91PP\x92\x95\x91\x94P\x92PV[a9\x0B\x81a3\x06V[\x82RPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a96\x81a9\x11V[\x82RPPV[``\x82\x01`\0\x82\x01Qa9R`\0\x85\x01\x82a9\x02V[P` \x82\x01Qa9e` \x85\x01\x82a9-V[P`@\x82\x01Qa9x`@\x85\x01\x82a9-V[PPPPV[`\0``\x82\x01\x90Pa9\x93`\0\x83\x01\x84a9<V[\x92\x91PPV[`\0a\x01\xC0\x82\x84\x03\x12\x15a9\xB0Wa9\xAFa4!V[[a9\xBBa\x01\xC0a4\x97V[\x90P`\0a9\xCB\x84\x82\x85\x01a2aV[`\0\x83\x01RP` a9\xDF\x84\x82\x85\x01a2aV[` \x83\x01RP`@a9\xF3\x84\x82\x85\x01a2aV[`@\x83\x01RP``a:\x07\x84\x82\x85\x01a2\xC4V[``\x83\x01RP`\x80a:\x1B\x84\x82\x85\x01a2\xC4V[`\x80\x83\x01RP`\xA0a:/\x84\x82\x85\x01a3\xA5V[`\xA0\x83\x01RP`\xC0a:C\x84\x82\x85\x01a3\xA5V[`\xC0\x83\x01RP`\xE0a:W\x84\x82\x85\x01a3\xA5V[`\xE0\x83\x01RPa\x01\0a:l\x84\x82\x85\x01a3\xA5V[a\x01\0\x83\x01RPa\x01 a:\x82\x84\x82\x85\x01a2\xC4V[a\x01 \x83\x01RPa\x01@a:\x98\x84\x82\x85\x01a2\xC4V[a\x01@\x83\x01RPa\x01`a:\xAE\x84\x82\x85\x01a2\xC4V[a\x01`\x83\x01RPa\x01\x80a:\xC4\x84\x82\x85\x01a2aV[a\x01\x80\x83\x01RPa\x01\xA0a:\xDA\x84\x82\x85\x01a2\xC4V[a\x01\xA0\x83\x01RP\x92\x91PPV[`\0a\x01\xC0\x82\x84\x03\x12\x15a:\xFEWa:\xFDa2\x0EV[[`\0a;\x0C\x84\x82\x85\x01a9\x99V[\x91PP\x92\x91PPV[\x7FL2OutputOracle: caller is not th`\0\x82\x01R\x7Fe owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a;q`'\x83a5\x84V[\x91Pa;|\x82a;\x15V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra;\xA0\x81a;dV[\x90P\x91\x90PV[\x7FL2OutputOracle: optimistic mode `\0\x82\x01R\x7Fis enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a<\x03`*\x83a5\x84V[\x91Pa<\x0E\x82a;\xA7V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<2\x81a;\xF6V[\x90P\x91\x90PV[\x7FL2OutputOracle: submission inter`\0\x82\x01R\x7Fval must be greater than 0\0\0\0\0\0\0` \x82\x01RPV[`\0a<\x95`:\x83a5\x84V[\x91Pa<\xA0\x82a<9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<\xC4\x81a<\x88V[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa<\xE0`\0\x83\x01\x85a3dV[a<\xED` \x83\x01\x84a3dV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a=.\x82a2\xA3V[\x91Pa=9\x83a2\xA3V[\x92P\x82\x82\x10\x15a=LWa=Ka<\xF4V[[\x82\x82\x03\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FL2OutputOracle: config name cann`\0\x82\x01R\x7Fot be empty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a=\xE2`+\x83a5\x84V[\x91Pa=\xED\x82a=\x86V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\x11\x81a=\xD5V[\x90P\x91\x90PV[\x7FL2OutputOracle: config already e`\0\x82\x01R\x7Fxists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a>t`%\x83a5\x84V[\x91Pa>\x7F\x82a>\x18V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\xA3\x81a>gV[\x90P\x91\x90PV[\x7FL2OutputOracle: invalid OP Succi`\0\x82\x01R\x7Fnct configuration parameters\0\0\0\0` \x82\x01RPV[`\0a?\x06`<\x83a5\x84V[\x91Pa?\x11\x82a>\xAAV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra?5\x81a>\xF9V[\x90P\x91\x90PV[\x7FL2OutputOracle: optimistic mode `\0\x82\x01R\x7Fis not enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a?\x98`.\x83a5\x84V[\x91Pa?\xA3\x82a?<V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra?\xC7\x81a?\x8BV[\x90P\x91\x90PV[\x7FL2OutputOracle: dispute game fac`\0\x82\x01R\x7Ftory is not set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a@*`/\x83a5\x84V[\x91Pa@5\x82a?\xCEV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra@Y\x81a@\x1DV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a@{a@v\x82a2\xA3V[a@`V[\x82RPPV[`\0\x81``\x1B\x90P\x91\x90PV[`\0a@\x99\x82a@\x81V[\x90P\x91\x90PV[`\0a@\xAB\x82a@\x8EV[\x90P\x91\x90PV[a@\xC3a@\xBE\x82a28V[a@\xA0V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a@\xE4a@\xDF\x82a3\x06V[a@\xC9V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0aA\x0B\x82a@\xEAV[aA\x15\x81\x85a@\xF5V[\x93PaA%\x81\x85` \x86\x01a5\x95V[\x80\x84\x01\x91PP\x92\x91PPV[`\0aA=\x82\x88a@jV[` \x82\x01\x91PaAM\x82\x87a@jV[` \x82\x01\x91PaA]\x82\x86a@\xB2V[`\x14\x82\x01\x91PaAm\x82\x85a@\xD3V[` \x82\x01\x91PaA}\x82\x84aA\0V[\x91P\x81\x90P\x96\x95PPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0aA\xB7aA\xB2aA\xAD\x84aA\x8CV[a7\xEAV[aA\x8CV[\x90P\x91\x90PV[aA\xC7\x81aA\x9CV[\x82RPPV[`\0aA\xD8\x82a3\x06V[\x90P\x91\x90PV[aA\xE8\x81aA\xCDV[\x82RPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0aB\n\x82a@\xEAV[aB\x14\x81\x85aA\xEEV[\x93PaB$\x81\x85` \x86\x01a5\x95V[aB-\x81a4&V[\x84\x01\x91PP\x92\x91PPV[`\0``\x82\x01\x90PaBM`\0\x83\x01\x86aA\xBEV[aBZ` \x83\x01\x85aA\xDFV[\x81\x81\x03`@\x83\x01RaBl\x81\x84aA\xFFV[\x90P\x94\x93PPPPV[`\0aB\x81\x82a28V[\x90P\x91\x90PV[aB\x91\x81aBvV[\x81\x14aB\x9CW`\0\x80\xFD[PV[`\0\x81Q\x90PaB\xAE\x81aB\x88V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aB\xCAWaB\xC9a2\x0EV[[`\0aB\xD8\x84\x82\x85\x01aB\x9FV[\x91PP\x92\x91PPV[\x7FL2OutputOracle: cannot get outpu`\0\x82\x01R\x7Ft for a block that has not been ` \x82\x01R\x7Fproposed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aCc`H\x83a5\x84V[\x91PaCn\x82aB\xE1V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaC\x92\x81aCVV[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot get outpu`\0\x82\x01R\x7Ft as no outputs have been propos` \x82\x01R\x7Fed yet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aD\x1B`F\x83a5\x84V[\x91PaD&\x82aC\x99V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaDJ\x81aD\x0EV[\x90P\x91\x90PV[`\0aD\\\x82a2\xA3V[\x91PaDg\x83a2\xA3V[\x92P\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x11\x15aD\x9CWaD\x9Ba<\xF4V[[\x82\x82\x01\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0aD\xE1\x82a2\xA3V[\x91PaD\xEC\x83a2\xA3V[\x92P\x82aD\xFCWaD\xFBaD\xA7V[[\x82\x82\x04\x90P\x92\x91PPV[\x7FL2OutputOracle: only the challen`\0\x82\x01R\x7Fger address can delete outputs\0\0` \x82\x01RPV[`\0aEc`>\x83a5\x84V[\x91PaEn\x82aE\x07V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaE\x92\x81aEVV[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ge`\0\x82\x01R\x7Fnesis output\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aE\xF5`,\x83a5\x84V[\x91PaF\0\x82aE\x99V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaF$\x81aE\xE8V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ou`\0\x82\x01R\x7Ftputs after the latest output in` \x82\x01R\x7Fdex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aF\xAD`C\x83a5\x84V[\x91PaF\xB8\x82aF+V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaF\xDC\x81aF\xA0V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ou`\0\x82\x01R\x7Ftputs that have already been fin` \x82\x01R\x7Falized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aGe`F\x83a5\x84V[\x91PaGp\x82aF\xE3V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaG\x94\x81aGXV[\x90P\x91\x90PV[\x7FL2OutputOracle: only approved pr`\0\x82\x01R\x7Foposers can propose new outputs\0` \x82\x01RPV[`\0aG\xF7`?\x83a5\x84V[\x91PaH\x02\x82aG\x9BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaH&\x81aG\xEAV[\x90P\x91\x90PV[\x7FL2OutputOracle: block number mus`\0\x82\x01R\x7Ft be equal to next expected bloc` \x82\x01R\x7Fk number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aH\xAF`H\x83a5\x84V[\x91PaH\xBA\x82aH-V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaH\xDE\x81aH\xA2V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output in the future\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aIA`6\x83a5\x84V[\x91PaIL\x82aH\xE5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaIp\x81aI4V[\x90P\x91\x90PV[\x7FL2OutputOracle: L2 output propos`\0\x82\x01R\x7Fal cannot be the zero hash\0\0\0\0\0\0` \x82\x01RPV[`\0aI\xD3`:\x83a5\x84V[\x91PaI\xDE\x82aIwV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\x02\x81aI\xC6V[\x90P\x91\x90PV[\x7FL2OutputOracle: block hash does `\0\x82\x01R\x7Fnot match the hash at the expect` \x82\x01R\x7Fed height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aJ\x8B`I\x83a5\x84V[\x91PaJ\x96\x82aJ\tV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\xBA\x81aJ~V[\x90P\x91\x90PV[\x7FL2OutputOracle: block number mus`\0\x82\x01R\x7Ft be greater than or equal to ne` \x82\x01R\x7Fxt expected block number\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aKC`X\x83a5\x84V[\x91PaKN\x82aJ\xC1V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaKr\x81aK6V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output from outside DisputeGam` \x82\x01R\x7FeFactory.create while disputeGam`@\x82\x01R\x7FeFactory is set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[`\0aL!`o\x83a5\x84V[\x91PaL,\x82aKyV[`\x80\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaLP\x81aL\x14V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output from inside DisputeGame` \x82\x01R\x7FFactory.create without setting d`@\x82\x01R\x7FisputeGameFactory\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[`\0aL\xFF`q\x83a5\x84V[\x91PaM\n\x82aLWV[`\x80\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaM.\x81aL\xF2V[\x90P\x91\x90PV[\x7FL2OutputOracle: invalid OP Succi`\0\x82\x01R\x7Fnct configuration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aM\x91`1\x83a5\x84V[\x91PaM\x9C\x82aM5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaM\xC0\x81aM\x84V[\x90P\x91\x90PV[aM\xD0\x81a2\xA3V[\x82RPPV[aM\xDF\x81a28V[\x82RPPV[`\xE0\x82\x01`\0\x82\x01QaM\xFB`\0\x85\x01\x82a9\x02V[P` \x82\x01QaN\x0E` \x85\x01\x82a9\x02V[P`@\x82\x01QaN!`@\x85\x01\x82a9\x02V[P``\x82\x01QaN4``\x85\x01\x82aM\xC7V[P`\x80\x82\x01QaNG`\x80\x85\x01\x82a9\x02V[P`\xA0\x82\x01QaNZ`\xA0\x85\x01\x82a9\x02V[P`\xC0\x82\x01QaNm`\xC0\x85\x01\x82aM\xD6V[PPPPV[`\0`\xE0\x82\x01\x90PaN\x88`\0\x83\x01\x84aM\xE5V[\x92\x91PPV[`\0``\x82\x01\x90PaN\xA3`\0\x83\x01\x86a3\x10V[\x81\x81\x03` \x83\x01RaN\xB5\x81\x85aA\xFFV[\x90P\x81\x81\x03`@\x83\x01RaN\xC9\x81\x84aA\xFFV[\x90P\x94\x93PPPPV[`\0aN\xDE\x82a2\xA3V[\x91PaN\xE9\x83a2\xA3V[\x92P\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aO\"WaO!a<\xF4V[[\x82\x82\x02\x90P\x92\x91PPV[\x7FInitializable: contract is alrea`\0\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aO\x89`.\x83a5\x84V[\x91PaO\x94\x82aO-V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaO\xB8\x81aO|V[\x90P\x91\x90PV[\x7FL2OutputOracle: L2 block time mu`\0\x82\x01R\x7Fst be greater than 0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aP\x1B`4\x83a5\x84V[\x91PaP&\x82aO\xBFV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaPJ\x81aP\x0EV[\x90P\x91\x90PV[\x7FL2OutputOracle: starting L2 time`\0\x82\x01R\x7Fstamp must be less than current ` \x82\x01R\x7Ftime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aP\xD3`D\x83a5\x84V[\x91PaP\xDE\x82aPQV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaQ\x02\x81aP\xC6V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xF1\xB1\xFC\x13\x06\xAF\x82\xA7`\xD2\xC9\xE4\x0E\x19\xB4\xCB\xA6\xFB\x04v\0\x8A+\x85=\xA27\xA3\xE6\xC0r\xADdsolcC\0\x08\x0F\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102885760003560e01c8063887862721161015a578063ce5db8d6116100c1578063e1a41bcf1161007a578063e1a41bcf146109e2578063e40b7a1214610a0d578063ec5b2e3a14610a36578063f2b4e61714610a5f578063f2fde38b14610a8a578063f72f606d14610ab357610288565b8063ce5db8d6146108aa578063cf8e5cf0146108d5578063d1de856c14610912578063d46512761461094f578063dcec33481461098c578063e0c2f935146109b757610288565b8063a196b52511610113578063a196b52514610788578063a25ae557146107c5578063a4ee9d7b14610802578063a8e4fb901461082b578063b03cd41814610856578063c32e4e3e1461087f57610288565b8063887862721461069957806389c44cbb146106c45780638da5cb5b146106ed57806393991af31461071857806397fc007c146107435780639aaab6481461076c57610288565b80634ab309ac116101fe5780636abcf563116101b75780636abcf563146105805780636d9a1c8b146105ab57806370872aa5146105d65780637a41a035146106015780637f006420146106315780637f01ea681461066e57610288565b80634ab309ac1461046c578063534db0e21461049557806354fd4d50146104c057806360caf7a0146104eb57806369f16eec146105165780636a56620b1461054157610288565b8063336c9e8111610250578063336c9e811461035e5780633419d2c2146103875780634277bc06146103b05780634599c788146103db57806347c37e9c1461040657806349185e061461042f57610288565b806309d632d31461028d5780631e856800146102b65780632b31841e146102df5780632b7ac3f31461030a5780632c69796114610335575b600080fd5b34801561029957600080fd5b506102b460048036038101906102af9190613276565b610ade565b005b3480156102c257600080fd5b506102dd60048036038101906102d891906132d9565b610c18565b005b3480156102eb57600080fd5b506102f4610c76565b604051610301919061331f565b60405180910390f35b34801561031657600080fd5b5061031f610c7c565b60405161032c9190613349565b60405180910390f35b34801561034157600080fd5b5061035c600480360381019061035791906132d9565b610ca2565b005b34801561036a57600080fd5b50610385600480360381019061038091906132d9565b610de2565b005b34801561039357600080fd5b506103ae60048036038101906103a99190613276565b610efa565b005b3480156103bc57600080fd5b506103c5611011565b6040516103d29190613373565b60405180910390f35b3480156103e757600080fd5b506103f0611017565b6040516103fd9190613373565b60405180910390f35b34801561041257600080fd5b5061042d600480360381019061042891906133ba565b611098565b005b34801561043b57600080fd5b5061045660048036038101906104519190613516565b6112d0565b604051610463919061355e565b60405180910390f35b34801561047857600080fd5b50610493600480360381019061048e91906132d9565b61130a565b005b3480156104a157600080fd5b506104aa611449565b6040516104b79190613349565b60405180910390f35b3480156104cc57600080fd5b506104d561146f565b6040516104e29190613601565b60405180910390f35b3480156104f757600080fd5b506105006114a8565b60405161050d919061355e565b60405180910390f35b34801561052257600080fd5b5061052b6114bb565b6040516105389190613373565b60405180910390f35b34801561054d57600080fd5b5061056860048036038101906105639190613623565b6114d4565b60405161057793929190613650565b60405180910390f35b34801561058c57600080fd5b506105956114fe565b6040516105a29190613373565b60405180910390f35b3480156105b757600080fd5b506105c061150b565b6040516105cd919061331f565b60405180910390f35b3480156105e257600080fd5b506105eb611511565b6040516105f89190613373565b60405180910390f35b61061b60048036038101906106169190613741565b611517565b6040516106289190613849565b60405180910390f35b34801561063d57600080fd5b50610658600480360381019061065391906132d9565b611707565b6040516106659190613373565b60405180910390f35b34801561067a57600080fd5b5061068361184e565b6040516106909190613880565b60405180910390f35b3480156106a557600080fd5b506106ae611853565b6040516106bb9190613373565b60405180910390f35b3480156106d057600080fd5b506106eb60048036038101906106e691906132d9565b611859565b005b3480156106f957600080fd5b50610702611a57565b60405161070f9190613349565b60405180910390f35b34801561072457600080fd5b5061072d611a7d565b60405161073a9190613373565b60405180910390f35b34801561074f57600080fd5b5061076a60048036038101906107659190613276565b611a83565b005b6107866004803603810190610781919061389b565b611bd3565b005b34801561079457600080fd5b506107af60048036038101906107aa91906132d9565b611f63565b6040516107bc919061331f565b60405180910390f35b3480156107d157600080fd5b506107ec60048036038101906107e791906132d9565b611f7b565b6040516107f9919061397e565b60405180910390f35b34801561080e57600080fd5b5061082960048036038101906108249190613741565b612055565b005b34801561083757600080fd5b506108406126c6565b60405161084d9190613349565b60405180910390f35b34801561086257600080fd5b5061087d60048036038101906108789190613276565b6126ec565b005b34801561088b57600080fd5b50610894612826565b6040516108a1919061331f565b60405180910390f35b3480156108b657600080fd5b506108bf61282c565b6040516108cc9190613373565b60405180910390f35b3480156108e157600080fd5b506108fc60048036038101906108f791906132d9565b612832565b604051610909919061397e565b60405180910390f35b34801561091e57600080fd5b50610939600480360381019061093491906132d9565b612914565b6040516109469190613373565b60405180910390f35b34801561095b57600080fd5b5061097660048036038101906109719190613276565b612945565b604051610983919061355e565b60405180910390f35b34801561099857600080fd5b506109a1612965565b6040516109ae9190613373565b60405180910390f35b3480156109c357600080fd5b506109cc612981565b6040516109d99190613373565b60405180910390f35b3480156109ee57600080fd5b506109f7612a02565b604051610a049190613373565b60405180910390f35b348015610a1957600080fd5b50610a346004803603810190610a2f9190613ae7565b612a08565b005b348015610a4257600080fd5b50610a5d6004803603810190610a589190613623565b612f34565b005b348015610a6b57600080fd5b50610a74613022565b604051610a819190613349565b60405180910390f35b348015610a9657600080fd5b50610ab16004803603810190610aac9190613276565b613048565b005b348015610abf57600080fd5b50610ac8613198565b604051610ad5919061331f565b60405180910390f35b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610b6e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b6590613b87565b60405180910390fd5b6000600e60008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff0219169083151502179055508073ffffffffffffffffffffffffffffffffffffffff167f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a6000604051610c0d919061355e565b60405180910390a250565b6000814090506000801b8103610c5a576040517f84c0686400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80600f6000848152602001908152602001600020819055505050565b600a5481565b600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d32576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d2990613b87565b60405180910390fd5b601060009054906101000a900460ff1615610d82576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d7990613c19565b60405180910390fd5b806008819055506001601060006101000a81548160ff021916908315150217905550600115157f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a82604051610dd79190613373565b60405180910390a250565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610e72576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e6990613b87565b60405180910390fd5b60008111610eb5576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610eac90613cab565b60405180910390fd5b7fc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b260045482604051610ee8929190613ccb565b60405180910390a18060048190555050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f8a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f8190613b87565b60405180910390fd5b80601360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508073ffffffffffffffffffffffffffffffffffffffff167f73702180ce348e07b058846d1745c99987ae6c741ff97ec28d4539530ef1e8f160405160405180910390a250565b60115481565b6000806003805490501461108f57600360016003805490506110399190613d23565b8154811061104a57611049613d57565b5b906000526020600020906002020160010160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16611093565b6001545b905090565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611128576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161111f90613b87565b60405180910390fd5b6000801b840361116d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161116490613df8565b60405180910390fd5b6111b16012600086815260200190815260200160002060405180606001604052908160008201548152602001600182015481526020016002820154815250506112d0565b156111f1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016111e890613e8a565b60405180910390fd5b60006040518060600160405280848152602001838152602001858152509050611219816112d0565b611258576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161124f90613f1c565b60405180910390fd5b8060126000878152602001908152602001600020600082015181600001556020820151816001015560408201518160020155905050847fea0123c726a665cb0ab5691444f929a7056c7a7709c60c0587829e8046b8d5148484876040516112c193929190613650565b60405180910390a25050505050565b60008060001b8260000151141580156112f057506000801b826020015114155b801561130357506000801b826040015114155b9050919050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461139a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161139190613b87565b60405180910390fd5b601060009054906101000a900460ff166113e9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016113e090613fae565b60405180910390fd5b806008819055506000601060006101000a81548160ff021916908315150217905550600015157f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a8260405161143e9190613373565b60405180910390a250565b600660009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6040518060400160405280600681526020017f76332e302e30000000000000000000000000000000000000000000000000000081525081565b601060009054906101000a900460ff1681565b600060016003805490506114cf9190613d23565b905090565b60126020528060005260406000206000915090508060000154908060010154908060020154905083565b6000600380549050905090565b600c5481565b60015481565b6000601060009054906101000a900460ff1615611569576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161156090613c19565b60405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036115fa576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016115f190614040565b60405180910390fd5b6001601360146101000a81548160ff021916908315150217905550601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166382ecf2f6346006898989888e8b604051602001611670959493929190614131565b6040516020818303038152906040526040518563ffffffff1660e01b815260040161169d93929190614238565b60206040518083038185885af11580156116bb573d6000803e3d6000fd5b50505050506040513d601f19601f820116820180604052508101906116e091906142b4565b90506000601360146101000a81548160ff0219169083151502179055509695505050505050565b6000611711611017565b821115611753576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161174a90614379565b60405180910390fd5b60006003805490501161179b576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161179290614431565b60405180910390fd5b60008060038054905090505b80821015611844576000600282846117bf9190614451565b6117c991906144d6565b905084600382815481106117e0576117df613d57565b5b906000526020600020906002020160010160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16101561183a576001816118339190614451565b925061183e565b8091505b506117a7565b8192505050919050565b600381565b60025481565b600660009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118e9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118e090614579565b60405180910390fd5b6000811161192c576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119239061460b565b60405180910390fd5b6003805490508110611973576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161196a906146c3565b60405180910390fd5b6008546003828154811061198a57611989613d57565b5b906000526020600020906002020160010160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16426119d59190613d23565b10611a15576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a0c9061477b565b60405180910390fd5b6000611a1f6114fe565b90508160035581817f4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b660405160405180910390a35050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60055481565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611b13576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611b0a90613b87565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff16600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff167f0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc60405160405180910390a380600b60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b601060009054906101000a900460ff16611c22576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c1990613fae565b60405180910390fd5b600e60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff1680611cc35750600e60008073ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff165b611d02576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cf99061480d565b60405180910390fd5b611d0a612965565b8314611d4b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d42906148c5565b60405180910390fd5b42611d5584612914565b10611d95576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d8c90614957565b60405180910390fd5b6000801b8403611dda576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611dd1906149e9565b60405180910390fd5b6000801b8214611e285781814014611e27576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e1e90614aa1565b60405180910390fd5b5b82611e316114fe565b857fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e242604051611e619190613373565b60405180910390a460036040518060600160405280868152602001426fffffffffffffffffffffffffffffffff168152602001856fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff160217905550505050505050565b600f6020528060005260406000206000915090505481565b611f836131bc565b60038281548110611f9757611f96613d57565b5b9060005260206000209060020201604051806060016040529081600082015481526020016001820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016001820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250509050919050565b601060009054906101000a900460ff16156120a5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161209c90613c19565b60405180910390fd5b600e60003273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16806121465750600e60008073ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff165b806121645750601154612157612981565b426121629190613d23565b115b6121a3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161219a9061480d565b60405180910390fd5b6121ab612965565b8410156121ed576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016121e490614b59565b60405180910390fd5b426121f785612914565b10612237576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161222e90614957565b60405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146122e157601360149054906101000a900460ff166122dc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016122d390614c37565b60405180910390fd5b612332565b601360149054906101000a900460ff1615612331576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161232890614d15565b60405180910390fd5b5b6000801b8503612377576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161236e906149e9565b60405180910390fd5b600060126000888152602001908152602001600020604051806060016040529081600082015481526020016001820154815260200160028201548152505090506123c0816112d0565b6123ff576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016123f690614da7565b60405180910390fd5b6000600f60008681526020019081526020016000205490506000801b8103612453576040517f22aa3a9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006040518060e0016040528083815260200160036124706114bb565b8154811061248157612480613d57565b5b906000526020600020906002020160000154815260200189815260200188815260200184604001518152602001846020015181526020018573ffffffffffffffffffffffffffffffffffffffff168152509050600b60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166341493c608460000151836040516020016125289190614e73565b604051602081830303815290604052886040518463ffffffff1660e01b815260040161255693929190614e8e565b60006040518083038186803b15801561256e57600080fd5b505afa158015612582573d6000803e3d6000fd5b505050508661258f6114fe565b897fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2426040516125bf9190613373565b60405180910390a4600360405180606001604052808a8152602001426fffffffffffffffffffffffffffffffff168152602001896fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055505050505050505050505050565b600760009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461277c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161277390613b87565b60405180910390fd5b6001600e60008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff0219169083151502179055508073ffffffffffffffffffffffffffffffffffffffff167f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a600160405161281b919061355e565b60405180910390a250565b60095481565b60085481565b61283a6131bc565b600361284583611707565b8154811061285657612855613d57565b5b9060005260206000209060020201604051806060016040529081600082015481526020016001820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016001820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250509050919050565b6000600554600154836129279190613d23565b6129319190614ed3565b60025461293e9190614451565b9050919050565b600e6020528060005260406000206000915054906101000a900460ff1681565b6000600454612972611017565b61297c9190614451565b905090565b600080600380549050146129f957600360016003805490506129a39190613d23565b815481106129b4576129b3613d57565b5b906000526020600020906002020160010160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166129fd565b6002545b905090565b60045481565b6003600060019054906101000a900460ff16158015612a3957508060ff1660008054906101000a900460ff1660ff16105b612a78576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612a6f90614f9f565b60405180910390fd5b806000806101000a81548160ff021916908360ff1602179055506001600060016101000a81548160ff021916908315150217905550600082610160015111612af5576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612aec90613cab565b60405180910390fd5b6000826080015111612b3c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612b3390615031565b60405180910390fd5b428261014001511115612b84576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612b7b906150e9565b60405180910390fd5b8161016001516004819055508160800151600581905550600060038054905003612cc4576003604051806060016040528084610100015181526020018461014001516fffffffffffffffffffffffffffffffff1681526020018461012001516fffffffffffffffffffffffffffffffff1681525090806001815401808255809150506001900390600052602060002090600202016000909190919091506000820151816000015560208201518160010160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060408201518160010160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555050508161012001516001819055508161014001516002819055505b8160000151600660006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555081606001516008819055506001600e6000846020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff021916908315150217905550816101a0015160118190555060405180606001604052808360a0015181526020018360c0015181526020018360e00151815250601260007fae8304f40f7123e0c87b97f8a600e94ff3a3a25be588fc66b8a3717c8959ce778152602001908152602001600020600082015181600001556020820151816001015560408201518160020155905050816101800151600b60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508160400151600d60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506000601360146101000a81548160ff0219169083151502179055506000601360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060008060016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249881604051612f289190613880565b60405180910390a15050565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612fc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612fbb90613b87565b60405180910390fd5b60126000828152602001908152602001600020600080820160009055600182016000905560028201600090555050807f4432b02a2fcbed48d94e8d72723e155c6690e4b7f39afa41a2a8ff8c0aa425da60405160405180910390a250565b601360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146130d8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016130cf90613b87565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff16600d60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a380600d60006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b7fae8304f40f7123e0c87b97f8a600e94ff3a3a25be588fc66b8a3717c8959ce7781565b60405180606001604052806000801916815260200160006fffffffffffffffffffffffffffffffff16815260200160006fffffffffffffffffffffffffffffffff1681525090565b6000604051905090565b600080fd5b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b600061324382613218565b9050919050565b61325381613238565b811461325e57600080fd5b50565b6000813590506132708161324a565b92915050565b60006020828403121561328c5761328b61320e565b5b600061329a84828501613261565b91505092915050565b6000819050919050565b6132b6816132a3565b81146132c157600080fd5b50565b6000813590506132d3816132ad565b92915050565b6000602082840312156132ef576132ee61320e565b5b60006132fd848285016132c4565b91505092915050565b6000819050919050565b61331981613306565b82525050565b60006020820190506133346000830184613310565b92915050565b61334381613238565b82525050565b600060208201905061335e600083018461333a565b92915050565b61336d816132a3565b82525050565b60006020820190506133886000830184613364565b92915050565b61339781613306565b81146133a257600080fd5b50565b6000813590506133b48161338e565b92915050565b600080600080608085870312156133d4576133d361320e565b5b60006133e2878288016133a5565b94505060206133f3878288016133a5565b9350506040613404878288016133a5565b9250506060613415878288016133a5565b91505092959194509250565b600080fd5b6000601f19601f8301169050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b61346f82613426565b810181811067ffffffffffffffff8211171561348e5761348d613437565b5b80604052505050565b60006134a1613204565b90506134ad8282613466565b919050565b6000606082840312156134c8576134c7613421565b5b6134d26060613497565b905060006134e2848285016133a5565b60008301525060206134f6848285016133a5565b602083015250604061350a848285016133a5565b60408301525092915050565b60006060828403121561352c5761352b61320e565b5b600061353a848285016134b2565b91505092915050565b60008115159050919050565b61355881613543565b82525050565b6000602082019050613573600083018461354f565b92915050565b600081519050919050565b600082825260208201905092915050565b60005b838110156135b3578082015181840152602081019050613598565b838111156135c2576000848401525b50505050565b60006135d382613579565b6135dd8185613584565b93506135ed818560208601613595565b6135f681613426565b840191505092915050565b6000602082019050818103600083015261361b81846135c8565b905092915050565b6000602082840312156136395761363861320e565b5b6000613647848285016133a5565b91505092915050565b60006060820190506136656000830186613310565b6136726020830185613310565b61367f6040830184613310565b949350505050565b600080fd5b600080fd5b600067ffffffffffffffff8211156136ac576136ab613437565b5b6136b582613426565b9050602081019050919050565b82818337600083830152505050565b60006136e46136df84613691565b613497565b905082815260208101848484011115613700576136ff61368c565b5b61370b8482856136c2565b509392505050565b600082601f83011261372857613727613687565b5b81356137388482602086016136d1565b91505092915050565b60008060008060008060c0878903121561375e5761375d61320e565b5b600061376c89828a016133a5565b965050602061377d89828a016133a5565b955050604061378e89828a016132c4565b945050606061379f89828a016132c4565b935050608087013567ffffffffffffffff8111156137c0576137bf613213565b5b6137cc89828a01613713565b92505060a06137dd89828a01613261565b9150509295509295509295565b6000819050919050565b600061380f61380a61380584613218565b6137ea565b613218565b9050919050565b6000613821826137f4565b9050919050565b600061383382613816565b9050919050565b61384381613828565b82525050565b600060208201905061385e600083018461383a565b92915050565b600060ff82169050919050565b61387a81613864565b82525050565b60006020820190506138956000830184613871565b92915050565b600080600080608085870312156138b5576138b461320e565b5b60006138c3878288016133a5565b94505060206138d4878288016132c4565b93505060406138e5878288016133a5565b92505060606138f6878288016132c4565b91505092959194509250565b61390b81613306565b82525050565b60006fffffffffffffffffffffffffffffffff82169050919050565b61393681613911565b82525050565b6060820160008201516139526000850182613902565b506020820151613965602085018261392d565b506040820151613978604085018261392d565b50505050565b6000606082019050613993600083018461393c565b92915050565b60006101c082840312156139b0576139af613421565b5b6139bb6101c0613497565b905060006139cb84828501613261565b60008301525060206139df84828501613261565b60208301525060406139f384828501613261565b6040830152506060613a07848285016132c4565b6060830152506080613a1b848285016132c4565b60808301525060a0613a2f848285016133a5565b60a08301525060c0613a43848285016133a5565b60c08301525060e0613a57848285016133a5565b60e083015250610100613a6c848285016133a5565b61010083015250610120613a82848285016132c4565b61012083015250610140613a98848285016132c4565b61014083015250610160613aae848285016132c4565b61016083015250610180613ac484828501613261565b610180830152506101a0613ada848285016132c4565b6101a08301525092915050565b60006101c08284031215613afe57613afd61320e565b5b6000613b0c84828501613999565b91505092915050565b7f4c324f75747075744f7261636c653a2063616c6c6572206973206e6f7420746860008201527f65206f776e657200000000000000000000000000000000000000000000000000602082015250565b6000613b71602783613584565b9150613b7c82613b15565b604082019050919050565b60006020820190508181036000830152613ba081613b64565b9050919050565b7f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f64652060008201527f697320656e61626c656400000000000000000000000000000000000000000000602082015250565b6000613c03602a83613584565b9150613c0e82613ba7565b604082019050919050565b60006020820190508181036000830152613c3281613bf6565b9050919050565b7f4c324f75747075744f7261636c653a207375626d697373696f6e20696e74657260008201527f76616c206d7573742062652067726561746572207468616e2030000000000000602082015250565b6000613c95603a83613584565b9150613ca082613c39565b604082019050919050565b60006020820190508181036000830152613cc481613c88565b9050919050565b6000604082019050613ce06000830185613364565b613ced6020830184613364565b9392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000613d2e826132a3565b9150613d39836132a3565b925082821015613d4c57613d4b613cf4565b5b828203905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4c324f75747075744f7261636c653a20636f6e666967206e616d652063616e6e60008201527f6f7420626520656d707479000000000000000000000000000000000000000000602082015250565b6000613de2602b83613584565b9150613ded82613d86565b604082019050919050565b60006020820190508181036000830152613e1181613dd5565b9050919050565b7f4c324f75747075744f7261636c653a20636f6e66696720616c7265616479206560008201527f7869737473000000000000000000000000000000000000000000000000000000602082015250565b6000613e74602583613584565b9150613e7f82613e18565b604082019050919050565b60006020820190508181036000830152613ea381613e67565b9050919050565b7f4c324f75747075744f7261636c653a20696e76616c6964204f5020537563636960008201527f6e637420636f6e66696775726174696f6e20706172616d657465727300000000602082015250565b6000613f06603c83613584565b9150613f1182613eaa565b604082019050919050565b60006020820190508181036000830152613f3581613ef9565b9050919050565b7f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f64652060008201527f6973206e6f7420656e61626c6564000000000000000000000000000000000000602082015250565b6000613f98602e83613584565b9150613fa382613f3c565b604082019050919050565b60006020820190508181036000830152613fc781613f8b565b9050919050565b7f4c324f75747075744f7261636c653a20646973707574652067616d652066616360008201527f746f7279206973206e6f74207365740000000000000000000000000000000000602082015250565b600061402a602f83613584565b915061403582613fce565b604082019050919050565b600060208201905081810360008301526140598161401d565b9050919050565b6000819050919050565b61407b614076826132a3565b614060565b82525050565b60008160601b9050919050565b600061409982614081565b9050919050565b60006140ab8261408e565b9050919050565b6140c36140be82613238565b6140a0565b82525050565b6000819050919050565b6140e46140df82613306565b6140c9565b82525050565b600081519050919050565b600081905092915050565b600061410b826140ea565b61411581856140f5565b9350614125818560208601613595565b80840191505092915050565b600061413d828861406a565b60208201915061414d828761406a565b60208201915061415d82866140b2565b60148201915061416d82856140d3565b60208201915061417d8284614100565b91508190509695505050505050565b600063ffffffff82169050919050565b60006141b76141b26141ad8461418c565b6137ea565b61418c565b9050919050565b6141c78161419c565b82525050565b60006141d882613306565b9050919050565b6141e8816141cd565b82525050565b600082825260208201905092915050565b600061420a826140ea565b61421481856141ee565b9350614224818560208601613595565b61422d81613426565b840191505092915050565b600060608201905061424d60008301866141be565b61425a60208301856141df565b818103604083015261426c81846141ff565b9050949350505050565b600061428182613238565b9050919050565b61429181614276565b811461429c57600080fd5b50565b6000815190506142ae81614288565b92915050565b6000602082840312156142ca576142c961320e565b5b60006142d88482850161429f565b91505092915050565b7f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560008201527f7420666f72206120626c6f636b207468617420686173206e6f74206265656e2060208201527f70726f706f736564000000000000000000000000000000000000000000000000604082015250565b6000614363604883613584565b915061436e826142e1565b606082019050919050565b6000602082019050818103600083015261439281614356565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560008201527f74206173206e6f206f7574707574732068617665206265656e2070726f706f7360208201527f6564207965740000000000000000000000000000000000000000000000000000604082015250565b600061441b604683613584565b915061442682614399565b606082019050919050565b6000602082019050818103600083015261444a8161440e565b9050919050565b600061445c826132a3565b9150614467836132a3565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0382111561449c5761449b613cf4565b5b828201905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60006144e1826132a3565b91506144ec836132a3565b9250826144fc576144fb6144a7565b5b828204905092915050565b7f4c324f75747075744f7261636c653a206f6e6c7920746865206368616c6c656e60008201527f67657220616464726573732063616e2064656c657465206f7574707574730000602082015250565b6000614563603e83613584565b915061456e82614507565b604082019050919050565b6000602082019050818103600083015261459281614556565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c65746520676560008201527f6e65736973206f75747075740000000000000000000000000000000000000000602082015250565b60006145f5602c83613584565b915061460082614599565b604082019050919050565b60006020820190508181036000830152614624816145e8565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560008201527f747075747320616674657220746865206c6174657374206f757470757420696e60208201527f6465780000000000000000000000000000000000000000000000000000000000604082015250565b60006146ad604383613584565b91506146b88261462b565b606082019050919050565b600060208201905081810360008301526146dc816146a0565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560008201527f74707574732074686174206861766520616c7265616479206265656e2066696e60208201527f616c697a65640000000000000000000000000000000000000000000000000000604082015250565b6000614765604683613584565b9150614770826146e3565b606082019050919050565b6000602082019050818103600083015261479481614758565b9050919050565b7f4c324f75747075744f7261636c653a206f6e6c7920617070726f76656420707260008201527f6f706f736572732063616e2070726f706f7365206e6577206f75747075747300602082015250565b60006147f7603f83613584565b91506148028261479b565b604082019050919050565b60006020820190508181036000830152614826816147ea565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360008201527f7420626520657175616c20746f206e65787420657870656374656420626c6f6360208201527f6b206e756d626572000000000000000000000000000000000000000000000000604082015250565b60006148af604883613584565b91506148ba8261482d565b606082019050919050565b600060208201905081810360008301526148de816148a2565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f757470757420696e207468652066757475726500000000000000000000602082015250565b6000614941603683613584565b915061494c826148e5565b604082019050919050565b6000602082019050818103600083015261497081614934565b9050919050565b7f4c324f75747075744f7261636c653a204c32206f75747075742070726f706f7360008201527f616c2063616e6e6f7420626520746865207a65726f2068617368000000000000602082015250565b60006149d3603a83613584565b91506149de82614977565b604082019050919050565b60006020820190508181036000830152614a02816149c6565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206861736820646f65732060008201527f6e6f74206d61746368207468652068617368206174207468652065787065637460208201527f6564206865696768740000000000000000000000000000000000000000000000604082015250565b6000614a8b604983613584565b9150614a9682614a09565b606082019050919050565b60006020820190508181036000830152614aba81614a7e565b9050919050565b7f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360008201527f742062652067726561746572207468616e206f7220657175616c20746f206e6560208201527f787420657870656374656420626c6f636b206e756d6265720000000000000000604082015250565b6000614b43605883613584565b9150614b4e82614ac1565b606082019050919050565b60006020820190508181036000830152614b7281614b36565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f75747075742066726f6d206f757473696465204469737075746547616d60208201527f65466163746f72792e637265617465207768696c65206469737075746547616d60408201527f65466163746f7279206973207365740000000000000000000000000000000000606082015250565b6000614c21606f83613584565b9150614c2c82614b79565b608082019050919050565b60006020820190508181036000830152614c5081614c14565b9050919050565b7f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60008201527f32206f75747075742066726f6d20696e73696465204469737075746547616d6560208201527f466163746f72792e63726561746520776974686f75742073657474696e67206460408201527f69737075746547616d65466163746f7279000000000000000000000000000000606082015250565b6000614cff607183613584565b9150614d0a82614c57565b608082019050919050565b60006020820190508181036000830152614d2e81614cf2565b9050919050565b7f4c324f75747075744f7261636c653a20696e76616c6964204f5020537563636960008201527f6e637420636f6e66696775726174696f6e000000000000000000000000000000602082015250565b6000614d91603183613584565b9150614d9c82614d35565b604082019050919050565b60006020820190508181036000830152614dc081614d84565b9050919050565b614dd0816132a3565b82525050565b614ddf81613238565b82525050565b60e082016000820151614dfb6000850182613902565b506020820151614e0e6020850182613902565b506040820151614e216040850182613902565b506060820151614e346060850182614dc7565b506080820151614e476080850182613902565b5060a0820151614e5a60a0850182613902565b5060c0820151614e6d60c0850182614dd6565b50505050565b600060e082019050614e886000830184614de5565b92915050565b6000606082019050614ea36000830186613310565b8181036020830152614eb581856141ff565b90508181036040830152614ec981846141ff565b9050949350505050565b6000614ede826132a3565b9150614ee9836132a3565b9250817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0483118215151615614f2257614f21613cf4565b5b828202905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160008201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b6000614f89602e83613584565b9150614f9482614f2d565b604082019050919050565b60006020820190508181036000830152614fb881614f7c565b9050919050565b7f4c324f75747075744f7261636c653a204c3220626c6f636b2074696d65206d7560008201527f73742062652067726561746572207468616e2030000000000000000000000000602082015250565b600061501b603483613584565b915061502682614fbf565b604082019050919050565b6000602082019050818103600083015261504a8161500e565b9050919050565b7f4c324f75747075744f7261636c653a207374617274696e67204c322074696d6560008201527f7374616d70206d757374206265206c657373207468616e2063757272656e742060208201527f74696d6500000000000000000000000000000000000000000000000000000000604082015250565b60006150d3604483613584565b91506150de82615051565b606082019050919050565b60006020820190508181036000830152615102816150c6565b905091905056fea2646970667358221220f1b1fc1306af82a760d2c9e40e19b4cba6fb0476008a2b853da237a3e6c072ad64736f6c634300080f0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x88W`\x005`\xE0\x1C\x80c\x88xbr\x11a\x01ZW\x80c\xCE]\xB8\xD6\x11a\0\xC1W\x80c\xE1\xA4\x1B\xCF\x11a\0zW\x80c\xE1\xA4\x1B\xCF\x14a\t\xE2W\x80c\xE4\x0Bz\x12\x14a\n\rW\x80c\xEC[.:\x14a\n6W\x80c\xF2\xB4\xE6\x17\x14a\n_W\x80c\xF2\xFD\xE3\x8B\x14a\n\x8AW\x80c\xF7/`m\x14a\n\xB3Wa\x02\x88V[\x80c\xCE]\xB8\xD6\x14a\x08\xAAW\x80c\xCF\x8E\\\xF0\x14a\x08\xD5W\x80c\xD1\xDE\x85l\x14a\t\x12W\x80c\xD4e\x12v\x14a\tOW\x80c\xDC\xEC3H\x14a\t\x8CW\x80c\xE0\xC2\xF95\x14a\t\xB7Wa\x02\x88V[\x80c\xA1\x96\xB5%\x11a\x01\x13W\x80c\xA1\x96\xB5%\x14a\x07\x88W\x80c\xA2Z\xE5W\x14a\x07\xC5W\x80c\xA4\xEE\x9D{\x14a\x08\x02W\x80c\xA8\xE4\xFB\x90\x14a\x08+W\x80c\xB0<\xD4\x18\x14a\x08VW\x80c\xC3.N>\x14a\x08\x7FWa\x02\x88V[\x80c\x88xbr\x14a\x06\x99W\x80c\x89\xC4L\xBB\x14a\x06\xC4W\x80c\x8D\xA5\xCB[\x14a\x06\xEDW\x80c\x93\x99\x1A\xF3\x14a\x07\x18W\x80c\x97\xFC\0|\x14a\x07CW\x80c\x9A\xAA\xB6H\x14a\x07lWa\x02\x88V[\x80cJ\xB3\t\xAC\x11a\x01\xFEW\x80cj\xBC\xF5c\x11a\x01\xB7W\x80cj\xBC\xF5c\x14a\x05\x80W\x80cm\x9A\x1C\x8B\x14a\x05\xABW\x80cp\x87*\xA5\x14a\x05\xD6W\x80czA\xA05\x14a\x06\x01W\x80c\x7F\0d \x14a\x061W\x80c\x7F\x01\xEAh\x14a\x06nWa\x02\x88V[\x80cJ\xB3\t\xAC\x14a\x04lW\x80cSM\xB0\xE2\x14a\x04\x95W\x80cT\xFDMP\x14a\x04\xC0W\x80c`\xCA\xF7\xA0\x14a\x04\xEBW\x80ci\xF1n\xEC\x14a\x05\x16W\x80cjVb\x0B\x14a\x05AWa\x02\x88V[\x80c3l\x9E\x81\x11a\x02PW\x80c3l\x9E\x81\x14a\x03^W\x80c4\x19\xD2\xC2\x14a\x03\x87W\x80cBw\xBC\x06\x14a\x03\xB0W\x80cE\x99\xC7\x88\x14a\x03\xDBW\x80cG\xC3~\x9C\x14a\x04\x06W\x80cI\x18^\x06\x14a\x04/Wa\x02\x88V[\x80c\t\xD62\xD3\x14a\x02\x8DW\x80c\x1E\x85h\0\x14a\x02\xB6W\x80c+1\x84\x1E\x14a\x02\xDFW\x80c+z\xC3\xF3\x14a\x03\nW\x80c,iya\x14a\x035W[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xB4`\x04\x806\x03\x81\x01\x90a\x02\xAF\x91\x90a2vV[a\n\xDEV[\0[4\x80\x15a\x02\xC2W`\0\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a2\xD9V[a\x0C\x18V[\0[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x02\xF4a\x0CvV[`@Qa\x03\x01\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03\x1Fa\x0C|V[`@Qa\x03,\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03AW`\0\x80\xFD[Pa\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a2\xD9V[a\x0C\xA2V[\0[4\x80\x15a\x03jW`\0\x80\xFD[Pa\x03\x85`\x04\x806\x03\x81\x01\x90a\x03\x80\x91\x90a2\xD9V[a\r\xE2V[\0[4\x80\x15a\x03\x93W`\0\x80\xFD[Pa\x03\xAE`\x04\x806\x03\x81\x01\x90a\x03\xA9\x91\x90a2vV[a\x0E\xFAV[\0[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x03\xC5a\x10\x11V[`@Qa\x03\xD2\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE7W`\0\x80\xFD[Pa\x03\xF0a\x10\x17V[`@Qa\x03\xFD\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x12W`\0\x80\xFD[Pa\x04-`\x04\x806\x03\x81\x01\x90a\x04(\x91\x90a3\xBAV[a\x10\x98V[\0[4\x80\x15a\x04;W`\0\x80\xFD[Pa\x04V`\x04\x806\x03\x81\x01\x90a\x04Q\x91\x90a5\x16V[a\x12\xD0V[`@Qa\x04c\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04xW`\0\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90a2\xD9V[a\x13\nV[\0[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x04\xAAa\x14IV[`@Qa\x04\xB7\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xCCW`\0\x80\xFD[Pa\x04\xD5a\x14oV[`@Qa\x04\xE2\x91\x90a6\x01V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xF7W`\0\x80\xFD[Pa\x05\0a\x14\xA8V[`@Qa\x05\r\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x05+a\x14\xBBV[`@Qa\x058\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05MW`\0\x80\xFD[Pa\x05h`\x04\x806\x03\x81\x01\x90a\x05c\x91\x90a6#V[a\x14\xD4V[`@Qa\x05w\x93\x92\x91\x90a6PV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x8CW`\0\x80\xFD[Pa\x05\x95a\x14\xFEV[`@Qa\x05\xA2\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x05\xC0a\x15\x0BV[`@Qa\x05\xCD\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xE2W`\0\x80\xFD[Pa\x05\xEBa\x15\x11V[`@Qa\x05\xF8\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1B`\x04\x806\x03\x81\x01\x90a\x06\x16\x91\x90a7AV[a\x15\x17V[`@Qa\x06(\x91\x90a8IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06=W`\0\x80\xFD[Pa\x06X`\x04\x806\x03\x81\x01\x90a\x06S\x91\x90a2\xD9V[a\x17\x07V[`@Qa\x06e\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x06\x83a\x18NV[`@Qa\x06\x90\x91\x90a8\x80V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xA5W`\0\x80\xFD[Pa\x06\xAEa\x18SV[`@Qa\x06\xBB\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xD0W`\0\x80\xFD[Pa\x06\xEB`\x04\x806\x03\x81\x01\x90a\x06\xE6\x91\x90a2\xD9V[a\x18YV[\0[4\x80\x15a\x06\xF9W`\0\x80\xFD[Pa\x07\x02a\x1AWV[`@Qa\x07\x0F\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07$W`\0\x80\xFD[Pa\x07-a\x1A}V[`@Qa\x07:\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07OW`\0\x80\xFD[Pa\x07j`\x04\x806\x03\x81\x01\x90a\x07e\x91\x90a2vV[a\x1A\x83V[\0[a\x07\x86`\x04\x806\x03\x81\x01\x90a\x07\x81\x91\x90a8\x9BV[a\x1B\xD3V[\0[4\x80\x15a\x07\x94W`\0\x80\xFD[Pa\x07\xAF`\x04\x806\x03\x81\x01\x90a\x07\xAA\x91\x90a2\xD9V[a\x1FcV[`@Qa\x07\xBC\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\xD1W`\0\x80\xFD[Pa\x07\xEC`\x04\x806\x03\x81\x01\x90a\x07\xE7\x91\x90a2\xD9V[a\x1F{V[`@Qa\x07\xF9\x91\x90a9~V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\x0EW`\0\x80\xFD[Pa\x08)`\x04\x806\x03\x81\x01\x90a\x08$\x91\x90a7AV[a UV[\0[4\x80\x15a\x087W`\0\x80\xFD[Pa\x08@a&\xC6V[`@Qa\x08M\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08bW`\0\x80\xFD[Pa\x08}`\x04\x806\x03\x81\x01\x90a\x08x\x91\x90a2vV[a&\xECV[\0[4\x80\x15a\x08\x8BW`\0\x80\xFD[Pa\x08\x94a(&V[`@Qa\x08\xA1\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xB6W`\0\x80\xFD[Pa\x08\xBFa(,V[`@Qa\x08\xCC\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xE1W`\0\x80\xFD[Pa\x08\xFC`\x04\x806\x03\x81\x01\x90a\x08\xF7\x91\x90a2\xD9V[a(2V[`@Qa\t\t\x91\x90a9~V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x1EW`\0\x80\xFD[Pa\t9`\x04\x806\x03\x81\x01\x90a\t4\x91\x90a2\xD9V[a)\x14V[`@Qa\tF\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t[W`\0\x80\xFD[Pa\tv`\x04\x806\x03\x81\x01\x90a\tq\x91\x90a2vV[a)EV[`@Qa\t\x83\x91\x90a5^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x98W`\0\x80\xFD[Pa\t\xA1a)eV[`@Qa\t\xAE\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xC3W`\0\x80\xFD[Pa\t\xCCa)\x81V[`@Qa\t\xD9\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xEEW`\0\x80\xFD[Pa\t\xF7a*\x02V[`@Qa\n\x04\x91\x90a3sV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x19W`\0\x80\xFD[Pa\n4`\x04\x806\x03\x81\x01\x90a\n/\x91\x90a:\xE7V[a*\x08V[\0[4\x80\x15a\nBW`\0\x80\xFD[Pa\n]`\x04\x806\x03\x81\x01\x90a\nX\x91\x90a6#V[a/4V[\0[4\x80\x15a\nkW`\0\x80\xFD[Pa\nta0\"V[`@Qa\n\x81\x91\x90a3IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x96W`\0\x80\xFD[Pa\n\xB1`\x04\x806\x03\x81\x01\x90a\n\xAC\x91\x90a2vV[a0HV[\0[4\x80\x15a\n\xBFW`\0\x80\xFD[Pa\n\xC8a1\x98V[`@Qa\n\xD5\x91\x90a3\x1FV[`@Q\x80\x91\x03\x90\xF3[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Be\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0`\x0E`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F]\xF3\x8D9^\xDC\x15\xB6i\xD6FV\x9B\xD0\x15Q3\x95\x07\x0B[M\xEB\x8A\x160\n\xBB\x06\r\x1BZ`\0`@Qa\x0C\r\x91\x90a5^V[`@Q\x80\x91\x03\x90\xA2PV[`\0\x81@\x90P`\0\x80\x1B\x81\x03a\x0CZW`@Q\x7F\x84\xC0hd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x0F`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPPPV[`\nT\x81V[`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r)\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\r\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ry\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x81\x90UP`\x01`\x10`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x15\x15\x7F\x1F\\\x87/\x1E\xA9<W\xE41\x12\xEAD\x9E\xE1\x9E\xF5uD\x88\xB8v'\xB4\xC5$V\xB0\xE5\xA4\x10\x9A\x82`@Qa\r\xD7\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA2PV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0ErW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ei\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x11a\x0E\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xAC\x90a<\xABV[`@Q\x80\x91\x03\x90\xFD[\x7F\xC1\xBF\x9A\xBF\xB5~\xA0\x1E\xD9\xEC\xB4\xF4^\x9C\xEF\xA7\xBAD\xB2\xE6w\x8C<\xE7(\x14\t\x99\x9F\x1A\xF1\xB2`\x04T\x82`@Qa\x0E\xE8\x92\x91\x90a<\xCBV[`@Q\x80\x91\x03\x90\xA1\x80`\x04\x81\x90UPPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x81\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80`\x13`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fsp!\x80\xCE4\x8E\x07\xB0X\x84m\x17E\xC9\x99\x87\xAElt\x1F\xF9~\xC2\x8DE9S\x0E\xF1\xE8\xF1`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x11T\x81V[`\0\x80`\x03\x80T\x90P\x14a\x10\x8FW`\x03`\x01`\x03\x80T\x90Pa\x109\x91\x90a=#V[\x81T\x81\x10a\x10JWa\x10Ia=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10\x93V[`\x01T[\x90P\x90V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11(W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x1F\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x84\x03a\x11mW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11d\x90a=\xF8V[`@Q\x80\x91\x03\x90\xFD[a\x11\xB1`\x12`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPPa\x12\xD0V[\x15a\x11\xF1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xE8\x90a>\x8AV[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x01\x85\x81RP\x90Pa\x12\x19\x81a\x12\xD0V[a\x12XW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12O\x90a?\x1CV[`@Q\x80\x91\x03\x90\xFD[\x80`\x12`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U\x90PP\x84\x7F\xEA\x01#\xC7&\xA6e\xCB\n\xB5i\x14D\xF9)\xA7\x05lzw\t\xC6\x0C\x05\x87\x82\x9E\x80F\xB8\xD5\x14\x84\x84\x87`@Qa\x12\xC1\x93\x92\x91\x90a6PV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x80`\0\x1B\x82`\0\x01Q\x14\x15\x80\x15a\x12\xF0WP`\0\x80\x1B\x82` \x01Q\x14\x15[\x80\x15a\x13\x03WP`\0\x80\x1B\x82`@\x01Q\x14\x15[\x90P\x91\x90PV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x91\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x13\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xE0\x90a?\xAEV[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x81\x90UP`\0`\x10`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x15\x15\x7F\x1F\\\x87/\x1E\xA9<W\xE41\x12\xEAD\x9E\xE1\x9E\xF5uD\x88\xB8v'\xB4\xC5$V\xB0\xE5\xA4\x10\x9A\x82`@Qa\x14>\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA2PV[`\x06`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fv3.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x01`\x03\x80T\x90Pa\x14\xCF\x91\x90a=#V[\x90P\x90V[`\x12` R\x80`\0R`@`\0 `\0\x91P\x90P\x80`\0\x01T\x90\x80`\x01\x01T\x90\x80`\x02\x01T\x90P\x83V[`\0`\x03\x80T\x90P\x90P\x90V[`\x0CT\x81V[`\x01T\x81V[`\0`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15`\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xF1\x90a@@V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x82\xEC\xF2\xF64`\x06\x89\x89\x89\x88\x8E\x8B`@Q` \x01a\x16p\x95\x94\x93\x92\x91\x90aA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x9D\x93\x92\x91\x90aB8V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x16\xBBW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE0\x91\x90aB\xB4V[\x90P`\0`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x96\x95PPPPPPV[`\0a\x17\x11a\x10\x17V[\x82\x11\x15a\x17SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17J\x90aCyV[`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x80T\x90P\x11a\x17\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x92\x90aD1V[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\x03\x80T\x90P\x90P[\x80\x82\x10\x15a\x18DW`\0`\x02\x82\x84a\x17\xBF\x91\x90aDQV[a\x17\xC9\x91\x90aD\xD6V[\x90P\x84`\x03\x82\x81T\x81\x10a\x17\xE0Wa\x17\xDFa=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x18:W`\x01\x81a\x183\x91\x90aDQV[\x92Pa\x18>V[\x80\x91P[Pa\x17\xA7V[\x81\x92PPP\x91\x90PV[`\x03\x81V[`\x02T\x81V[`\x06`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xE0\x90aEyV[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x11a\x19,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19#\x90aF\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80T\x90P\x81\x10a\x19sW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19j\x90aF\xC3V[`@Q\x80\x91\x03\x90\xFD[`\x08T`\x03\x82\x81T\x81\x10a\x19\x8AWa\x19\x89a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x19\xD5\x91\x90a=#V[\x10a\x1A\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x0C\x90aG{V[`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x1Fa\x14\xFEV[\x90P\x81`\x03U\x81\x81\x7FN\xE3z\xC2\xC7\x86\xEC\x85\xE8u\x92\xD3\xC5\xC8\xA1\xDDf\xF8Im\xDA?\x12]\x9E\xA8\xCA_ev)\xB6`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x05T\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02CT\x9A\x92\xB2A/z<\xAFz.V\xD6[\x88!\xB9\x13E6?\xAA_W\x19S\x84\x06_\xCC`@Q`@Q\x80\x91\x03\x90\xA3\x80`\x0B`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1C\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x19\x90a?\xAEV[`@Q\x80\x91\x03\x90\xFD[`\x0E`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1C\xC3WP`\x0E`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[a\x1D\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xF9\x90aH\rV[`@Q\x80\x91\x03\x90\xFD[a\x1D\na)eV[\x83\x14a\x1DKW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DB\x90aH\xC5V[`@Q\x80\x91\x03\x90\xFD[Ba\x1DU\x84a)\x14V[\x10a\x1D\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x8C\x90aIWV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x84\x03a\x1D\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xD1\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x1B\x82\x14a\x1E(W\x81\x81@\x14a\x1E'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x1E\x90aJ\xA1V[`@Q\x80\x91\x03\x90\xFD[[\x82a\x1E1a\x14\xFEV[\x85\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa\x1Ea\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA4`\x03`@Q\x80``\x01`@R\x80\x86\x81R` \x01Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[`\x0F` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[a\x1F\x83a1\xBCV[`\x03\x82\x81T\x81\x10a\x1F\x97Wa\x1F\x96a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a \xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x9C\x90a<\x19V[`@Q\x80\x91\x03\x90\xFD[`\x0E`\x002s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a!FWP`\x0E`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x80a!dWP`\x11Ta!Wa)\x81V[Ba!b\x91\x90a=#V[\x11[a!\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x9A\x90aH\rV[`@Q\x80\x91\x03\x90\xFD[a!\xABa)eV[\x84\x10\x15a!\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\xE4\x90aKYV[`@Q\x80\x91\x03\x90\xFD[Ba!\xF7\x85a)\x14V[\x10a\"7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\".\x90aIWV[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\"\xE1W`\x13`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\"\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xD3\x90aL7V[`@Q\x80\x91\x03\x90\xFD[a#2V[`\x13`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a#1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#(\x90aM\x15V[`@Q\x80\x91\x03\x90\xFD[[`\0\x80\x1B\x85\x03a#wW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#n\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[`\0`\x12`\0\x88\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90Pa#\xC0\x81a\x12\xD0V[a#\xFFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xF6\x90aM\xA7V[`@Q\x80\x91\x03\x90\xFD[`\0`\x0F`\0\x86\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x03a$SW`@Q\x7F\"\xAA:\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xE0\x01`@R\x80\x83\x81R` \x01`\x03a$pa\x14\xBBV[\x81T\x81\x10a$\x81Wa$\x80a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x84`@\x01Q\x81R` \x01\x84` \x01Q\x81R` \x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P`\x0B`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cAI<`\x84`\0\x01Q\x83`@Q` \x01a%(\x91\x90aNsV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x88`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%V\x93\x92\x91\x90aN\x8EV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a%nW`\0\x80\xFD[PZ\xFA\x15\x80\x15a%\x82W=`\0\x80>=`\0\xFD[PPPP\x86a%\x8Fa\x14\xFEV[\x89\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa%\xBF\x91\x90a3sV[`@Q\x80\x91\x03\x90\xA4`\x03`@Q\x80``\x01`@R\x80\x8A\x81R` \x01Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPPPPPPV[`\x07`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'|W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a's\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0E`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F]\xF3\x8D9^\xDC\x15\xB6i\xD6FV\x9B\xD0\x15Q3\x95\x07\x0B[M\xEB\x8A\x160\n\xBB\x06\r\x1BZ`\x01`@Qa(\x1B\x91\x90a5^V[`@Q\x80\x91\x03\x90\xA2PV[`\tT\x81V[`\x08T\x81V[a(:a1\xBCV[`\x03a(E\x83a\x17\x07V[\x81T\x81\x10a(VWa(Ua=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80``\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\0`\x05T`\x01T\x83a)'\x91\x90a=#V[a)1\x91\x90aN\xD3V[`\x02Ta)>\x91\x90aDQV[\x90P\x91\x90PV[`\x0E` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x04Ta)ra\x10\x17V[a)|\x91\x90aDQV[\x90P\x90V[`\0\x80`\x03\x80T\x90P\x14a)\xF9W`\x03`\x01`\x03\x80T\x90Pa)\xA3\x91\x90a=#V[\x81T\x81\x10a)\xB4Wa)\xB3a=WV[[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xFDV[`\x02T[\x90P\x90V[`\x04T\x81V[`\x03`\0`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a*9WP\x80`\xFF\x16`\0\x80T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[a*xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*o\x90aO\x9FV[`@Q\x80\x91\x03\x90\xFD[\x80`\0\x80a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\x01`\0`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x82a\x01`\x01Q\x11a*\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xEC\x90a<\xABV[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x80\x01Q\x11a+<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+3\x90aP1V[`@Q\x80\x91\x03\x90\xFD[B\x82a\x01@\x01Q\x11\x15a+\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+{\x90aP\xE9V[`@Q\x80\x91\x03\x90\xFD[\x81a\x01`\x01Q`\x04\x81\x90UP\x81`\x80\x01Q`\x05\x81\x90UP`\0`\x03\x80T\x90P\x03a,\xC4W`\x03`@Q\x80``\x01`@R\x80\x84a\x01\0\x01Q\x81R` \x01\x84a\x01@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84a\x01 \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP\x81a\x01 \x01Q`\x01\x81\x90UP\x81a\x01@\x01Q`\x02\x81\x90UP[\x81`\0\x01Q`\x06`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81``\x01Q`\x08\x81\x90UP`\x01`\x0E`\0\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81a\x01\xA0\x01Q`\x11\x81\x90UP`@Q\x80``\x01`@R\x80\x83`\xA0\x01Q\x81R` \x01\x83`\xC0\x01Q\x81R` \x01\x83`\xE0\x01Q\x81RP`\x12`\0\x7F\xAE\x83\x04\xF4\x0Fq#\xE0\xC8{\x97\xF8\xA6\0\xE9O\xF3\xA3\xA2[\xE5\x88\xFCf\xB8\xA3q|\x89Y\xCEw\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U\x90PP\x81a\x01\x80\x01Q`\x0B`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`@\x01Q`\r`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x13`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x13`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0\x80`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x81`@Qa/(\x91\x90a8\x80V[`@Q\x80\x91\x03\x90\xA1PPV[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xBB\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[`\x12`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0\x90U`\x01\x82\x01`\0\x90U`\x02\x82\x01`\0\x90UPP\x80\x7FD2\xB0*/\xCB\xEDH\xD9N\x8Drr>\x15\\f\x90\xE4\xB7\xF3\x9A\xFAA\xA2\xA8\xFF\x8C\n\xA4%\xDA`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x13`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\xCF\x90a;\x87V[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\r`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3\x80`\r`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[\x7F\xAE\x83\x04\xF4\x0Fq#\xE0\xC8{\x97\xF8\xA6\0\xE9O\xF3\xA3\xA2[\xE5\x88\xFCf\xB8\xA3q|\x89Y\xCEw\x81V[`@Q\x80``\x01`@R\x80`\0\x80\x19\x16\x81R` \x01`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a2C\x82a2\x18V[\x90P\x91\x90PV[a2S\x81a28V[\x81\x14a2^W`\0\x80\xFD[PV[`\0\x815\x90Pa2p\x81a2JV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2\x8CWa2\x8Ba2\x0EV[[`\0a2\x9A\x84\x82\x85\x01a2aV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a2\xB6\x81a2\xA3V[\x81\x14a2\xC1W`\0\x80\xFD[PV[`\0\x815\x90Pa2\xD3\x81a2\xADV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2\xEFWa2\xEEa2\x0EV[[`\0a2\xFD\x84\x82\x85\x01a2\xC4V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a3\x19\x81a3\x06V[\x82RPPV[`\0` \x82\x01\x90Pa34`\0\x83\x01\x84a3\x10V[\x92\x91PPV[a3C\x81a28V[\x82RPPV[`\0` \x82\x01\x90Pa3^`\0\x83\x01\x84a3:V[\x92\x91PPV[a3m\x81a2\xA3V[\x82RPPV[`\0` \x82\x01\x90Pa3\x88`\0\x83\x01\x84a3dV[\x92\x91PPV[a3\x97\x81a3\x06V[\x81\x14a3\xA2W`\0\x80\xFD[PV[`\0\x815\x90Pa3\xB4\x81a3\x8EV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3\xD4Wa3\xD3a2\x0EV[[`\0a3\xE2\x87\x82\x88\x01a3\xA5V[\x94PP` a3\xF3\x87\x82\x88\x01a3\xA5V[\x93PP`@a4\x04\x87\x82\x88\x01a3\xA5V[\x92PP``a4\x15\x87\x82\x88\x01a3\xA5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a4o\x82a4&V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\x8EWa4\x8Da47V[[\x80`@RPPPV[`\0a4\xA1a2\x04V[\x90Pa4\xAD\x82\x82a4fV[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a4\xC8Wa4\xC7a4!V[[a4\xD2``a4\x97V[\x90P`\0a4\xE2\x84\x82\x85\x01a3\xA5V[`\0\x83\x01RP` a4\xF6\x84\x82\x85\x01a3\xA5V[` \x83\x01RP`@a5\n\x84\x82\x85\x01a3\xA5V[`@\x83\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5,Wa5+a2\x0EV[[`\0a5:\x84\x82\x85\x01a4\xB2V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a5X\x81a5CV[\x82RPPV[`\0` \x82\x01\x90Pa5s`\0\x83\x01\x84a5OV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a5\xB3W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa5\x98V[\x83\x81\x11\x15a5\xC2W`\0\x84\x84\x01R[PPPPV[`\0a5\xD3\x82a5yV[a5\xDD\x81\x85a5\x84V[\x93Pa5\xED\x81\x85` \x86\x01a5\x95V[a5\xF6\x81a4&V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra6\x1B\x81\x84a5\xC8V[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a69Wa68a2\x0EV[[`\0a6G\x84\x82\x85\x01a3\xA5V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa6e`\0\x83\x01\x86a3\x10V[a6r` \x83\x01\x85a3\x10V[a6\x7F`@\x83\x01\x84a3\x10V[\x94\x93PPPPV[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\xACWa6\xABa47V[[a6\xB5\x82a4&V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a6\xE4a6\xDF\x84a6\x91V[a4\x97V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a7\0Wa6\xFFa6\x8CV[[a7\x0B\x84\x82\x85a6\xC2V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a7(Wa7'a6\x87V[[\x815a78\x84\x82` \x86\x01a6\xD1V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a7^Wa7]a2\x0EV[[`\0a7l\x89\x82\x8A\x01a3\xA5V[\x96PP` a7}\x89\x82\x8A\x01a3\xA5V[\x95PP`@a7\x8E\x89\x82\x8A\x01a2\xC4V[\x94PP``a7\x9F\x89\x82\x8A\x01a2\xC4V[\x93PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xC0Wa7\xBFa2\x13V[[a7\xCC\x89\x82\x8A\x01a7\x13V[\x92PP`\xA0a7\xDD\x89\x82\x8A\x01a2aV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81\x90P\x91\x90PV[`\0a8\x0Fa8\na8\x05\x84a2\x18V[a7\xEAV[a2\x18V[\x90P\x91\x90PV[`\0a8!\x82a7\xF4V[\x90P\x91\x90PV[`\0a83\x82a8\x16V[\x90P\x91\x90PV[a8C\x81a8(V[\x82RPPV[`\0` \x82\x01\x90Pa8^`\0\x83\x01\x84a8:V[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a8z\x81a8dV[\x82RPPV[`\0` \x82\x01\x90Pa8\x95`\0\x83\x01\x84a8qV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a8\xB5Wa8\xB4a2\x0EV[[`\0a8\xC3\x87\x82\x88\x01a3\xA5V[\x94PP` a8\xD4\x87\x82\x88\x01a2\xC4V[\x93PP`@a8\xE5\x87\x82\x88\x01a3\xA5V[\x92PP``a8\xF6\x87\x82\x88\x01a2\xC4V[\x91PP\x92\x95\x91\x94P\x92PV[a9\x0B\x81a3\x06V[\x82RPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a96\x81a9\x11V[\x82RPPV[``\x82\x01`\0\x82\x01Qa9R`\0\x85\x01\x82a9\x02V[P` \x82\x01Qa9e` \x85\x01\x82a9-V[P`@\x82\x01Qa9x`@\x85\x01\x82a9-V[PPPPV[`\0``\x82\x01\x90Pa9\x93`\0\x83\x01\x84a9<V[\x92\x91PPV[`\0a\x01\xC0\x82\x84\x03\x12\x15a9\xB0Wa9\xAFa4!V[[a9\xBBa\x01\xC0a4\x97V[\x90P`\0a9\xCB\x84\x82\x85\x01a2aV[`\0\x83\x01RP` a9\xDF\x84\x82\x85\x01a2aV[` \x83\x01RP`@a9\xF3\x84\x82\x85\x01a2aV[`@\x83\x01RP``a:\x07\x84\x82\x85\x01a2\xC4V[``\x83\x01RP`\x80a:\x1B\x84\x82\x85\x01a2\xC4V[`\x80\x83\x01RP`\xA0a:/\x84\x82\x85\x01a3\xA5V[`\xA0\x83\x01RP`\xC0a:C\x84\x82\x85\x01a3\xA5V[`\xC0\x83\x01RP`\xE0a:W\x84\x82\x85\x01a3\xA5V[`\xE0\x83\x01RPa\x01\0a:l\x84\x82\x85\x01a3\xA5V[a\x01\0\x83\x01RPa\x01 a:\x82\x84\x82\x85\x01a2\xC4V[a\x01 \x83\x01RPa\x01@a:\x98\x84\x82\x85\x01a2\xC4V[a\x01@\x83\x01RPa\x01`a:\xAE\x84\x82\x85\x01a2\xC4V[a\x01`\x83\x01RPa\x01\x80a:\xC4\x84\x82\x85\x01a2aV[a\x01\x80\x83\x01RPa\x01\xA0a:\xDA\x84\x82\x85\x01a2\xC4V[a\x01\xA0\x83\x01RP\x92\x91PPV[`\0a\x01\xC0\x82\x84\x03\x12\x15a:\xFEWa:\xFDa2\x0EV[[`\0a;\x0C\x84\x82\x85\x01a9\x99V[\x91PP\x92\x91PPV[\x7FL2OutputOracle: caller is not th`\0\x82\x01R\x7Fe owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a;q`'\x83a5\x84V[\x91Pa;|\x82a;\x15V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra;\xA0\x81a;dV[\x90P\x91\x90PV[\x7FL2OutputOracle: optimistic mode `\0\x82\x01R\x7Fis enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a<\x03`*\x83a5\x84V[\x91Pa<\x0E\x82a;\xA7V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<2\x81a;\xF6V[\x90P\x91\x90PV[\x7FL2OutputOracle: submission inter`\0\x82\x01R\x7Fval must be greater than 0\0\0\0\0\0\0` \x82\x01RPV[`\0a<\x95`:\x83a5\x84V[\x91Pa<\xA0\x82a<9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<\xC4\x81a<\x88V[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa<\xE0`\0\x83\x01\x85a3dV[a<\xED` \x83\x01\x84a3dV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a=.\x82a2\xA3V[\x91Pa=9\x83a2\xA3V[\x92P\x82\x82\x10\x15a=LWa=Ka<\xF4V[[\x82\x82\x03\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FL2OutputOracle: config name cann`\0\x82\x01R\x7Fot be empty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a=\xE2`+\x83a5\x84V[\x91Pa=\xED\x82a=\x86V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\x11\x81a=\xD5V[\x90P\x91\x90PV[\x7FL2OutputOracle: config already e`\0\x82\x01R\x7Fxists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a>t`%\x83a5\x84V[\x91Pa>\x7F\x82a>\x18V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\xA3\x81a>gV[\x90P\x91\x90PV[\x7FL2OutputOracle: invalid OP Succi`\0\x82\x01R\x7Fnct configuration parameters\0\0\0\0` \x82\x01RPV[`\0a?\x06`<\x83a5\x84V[\x91Pa?\x11\x82a>\xAAV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra?5\x81a>\xF9V[\x90P\x91\x90PV[\x7FL2OutputOracle: optimistic mode `\0\x82\x01R\x7Fis not enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a?\x98`.\x83a5\x84V[\x91Pa?\xA3\x82a?<V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra?\xC7\x81a?\x8BV[\x90P\x91\x90PV[\x7FL2OutputOracle: dispute game fac`\0\x82\x01R\x7Ftory is not set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a@*`/\x83a5\x84V[\x91Pa@5\x82a?\xCEV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra@Y\x81a@\x1DV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a@{a@v\x82a2\xA3V[a@`V[\x82RPPV[`\0\x81``\x1B\x90P\x91\x90PV[`\0a@\x99\x82a@\x81V[\x90P\x91\x90PV[`\0a@\xAB\x82a@\x8EV[\x90P\x91\x90PV[a@\xC3a@\xBE\x82a28V[a@\xA0V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a@\xE4a@\xDF\x82a3\x06V[a@\xC9V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0aA\x0B\x82a@\xEAV[aA\x15\x81\x85a@\xF5V[\x93PaA%\x81\x85` \x86\x01a5\x95V[\x80\x84\x01\x91PP\x92\x91PPV[`\0aA=\x82\x88a@jV[` \x82\x01\x91PaAM\x82\x87a@jV[` \x82\x01\x91PaA]\x82\x86a@\xB2V[`\x14\x82\x01\x91PaAm\x82\x85a@\xD3V[` \x82\x01\x91PaA}\x82\x84aA\0V[\x91P\x81\x90P\x96\x95PPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0aA\xB7aA\xB2aA\xAD\x84aA\x8CV[a7\xEAV[aA\x8CV[\x90P\x91\x90PV[aA\xC7\x81aA\x9CV[\x82RPPV[`\0aA\xD8\x82a3\x06V[\x90P\x91\x90PV[aA\xE8\x81aA\xCDV[\x82RPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0aB\n\x82a@\xEAV[aB\x14\x81\x85aA\xEEV[\x93PaB$\x81\x85` \x86\x01a5\x95V[aB-\x81a4&V[\x84\x01\x91PP\x92\x91PPV[`\0``\x82\x01\x90PaBM`\0\x83\x01\x86aA\xBEV[aBZ` \x83\x01\x85aA\xDFV[\x81\x81\x03`@\x83\x01RaBl\x81\x84aA\xFFV[\x90P\x94\x93PPPPV[`\0aB\x81\x82a28V[\x90P\x91\x90PV[aB\x91\x81aBvV[\x81\x14aB\x9CW`\0\x80\xFD[PV[`\0\x81Q\x90PaB\xAE\x81aB\x88V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aB\xCAWaB\xC9a2\x0EV[[`\0aB\xD8\x84\x82\x85\x01aB\x9FV[\x91PP\x92\x91PPV[\x7FL2OutputOracle: cannot get outpu`\0\x82\x01R\x7Ft for a block that has not been ` \x82\x01R\x7Fproposed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aCc`H\x83a5\x84V[\x91PaCn\x82aB\xE1V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaC\x92\x81aCVV[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot get outpu`\0\x82\x01R\x7Ft as no outputs have been propos` \x82\x01R\x7Fed yet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aD\x1B`F\x83a5\x84V[\x91PaD&\x82aC\x99V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaDJ\x81aD\x0EV[\x90P\x91\x90PV[`\0aD\\\x82a2\xA3V[\x91PaDg\x83a2\xA3V[\x92P\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x11\x15aD\x9CWaD\x9Ba<\xF4V[[\x82\x82\x01\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0aD\xE1\x82a2\xA3V[\x91PaD\xEC\x83a2\xA3V[\x92P\x82aD\xFCWaD\xFBaD\xA7V[[\x82\x82\x04\x90P\x92\x91PPV[\x7FL2OutputOracle: only the challen`\0\x82\x01R\x7Fger address can delete outputs\0\0` \x82\x01RPV[`\0aEc`>\x83a5\x84V[\x91PaEn\x82aE\x07V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaE\x92\x81aEVV[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ge`\0\x82\x01R\x7Fnesis output\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aE\xF5`,\x83a5\x84V[\x91PaF\0\x82aE\x99V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaF$\x81aE\xE8V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ou`\0\x82\x01R\x7Ftputs after the latest output in` \x82\x01R\x7Fdex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aF\xAD`C\x83a5\x84V[\x91PaF\xB8\x82aF+V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaF\xDC\x81aF\xA0V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot delete ou`\0\x82\x01R\x7Ftputs that have already been fin` \x82\x01R\x7Falized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aGe`F\x83a5\x84V[\x91PaGp\x82aF\xE3V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaG\x94\x81aGXV[\x90P\x91\x90PV[\x7FL2OutputOracle: only approved pr`\0\x82\x01R\x7Foposers can propose new outputs\0` \x82\x01RPV[`\0aG\xF7`?\x83a5\x84V[\x91PaH\x02\x82aG\x9BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaH&\x81aG\xEAV[\x90P\x91\x90PV[\x7FL2OutputOracle: block number mus`\0\x82\x01R\x7Ft be equal to next expected bloc` \x82\x01R\x7Fk number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aH\xAF`H\x83a5\x84V[\x91PaH\xBA\x82aH-V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaH\xDE\x81aH\xA2V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output in the future\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aIA`6\x83a5\x84V[\x91PaIL\x82aH\xE5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaIp\x81aI4V[\x90P\x91\x90PV[\x7FL2OutputOracle: L2 output propos`\0\x82\x01R\x7Fal cannot be the zero hash\0\0\0\0\0\0` \x82\x01RPV[`\0aI\xD3`:\x83a5\x84V[\x91PaI\xDE\x82aIwV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\x02\x81aI\xC6V[\x90P\x91\x90PV[\x7FL2OutputOracle: block hash does `\0\x82\x01R\x7Fnot match the hash at the expect` \x82\x01R\x7Fed height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aJ\x8B`I\x83a5\x84V[\x91PaJ\x96\x82aJ\tV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\xBA\x81aJ~V[\x90P\x91\x90PV[\x7FL2OutputOracle: block number mus`\0\x82\x01R\x7Ft be greater than or equal to ne` \x82\x01R\x7Fxt expected block number\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aKC`X\x83a5\x84V[\x91PaKN\x82aJ\xC1V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaKr\x81aK6V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output from outside DisputeGam` \x82\x01R\x7FeFactory.create while disputeGam`@\x82\x01R\x7FeFactory is set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[`\0aL!`o\x83a5\x84V[\x91PaL,\x82aKyV[`\x80\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaLP\x81aL\x14V[\x90P\x91\x90PV[\x7FL2OutputOracle: cannot propose L`\0\x82\x01R\x7F2 output from inside DisputeGame` \x82\x01R\x7FFactory.create without setting d`@\x82\x01R\x7FisputeGameFactory\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[`\0aL\xFF`q\x83a5\x84V[\x91PaM\n\x82aLWV[`\x80\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaM.\x81aL\xF2V[\x90P\x91\x90PV[\x7FL2OutputOracle: invalid OP Succi`\0\x82\x01R\x7Fnct configuration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aM\x91`1\x83a5\x84V[\x91PaM\x9C\x82aM5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaM\xC0\x81aM\x84V[\x90P\x91\x90PV[aM\xD0\x81a2\xA3V[\x82RPPV[aM\xDF\x81a28V[\x82RPPV[`\xE0\x82\x01`\0\x82\x01QaM\xFB`\0\x85\x01\x82a9\x02V[P` \x82\x01QaN\x0E` \x85\x01\x82a9\x02V[P`@\x82\x01QaN!`@\x85\x01\x82a9\x02V[P``\x82\x01QaN4``\x85\x01\x82aM\xC7V[P`\x80\x82\x01QaNG`\x80\x85\x01\x82a9\x02V[P`\xA0\x82\x01QaNZ`\xA0\x85\x01\x82a9\x02V[P`\xC0\x82\x01QaNm`\xC0\x85\x01\x82aM\xD6V[PPPPV[`\0`\xE0\x82\x01\x90PaN\x88`\0\x83\x01\x84aM\xE5V[\x92\x91PPV[`\0``\x82\x01\x90PaN\xA3`\0\x83\x01\x86a3\x10V[\x81\x81\x03` \x83\x01RaN\xB5\x81\x85aA\xFFV[\x90P\x81\x81\x03`@\x83\x01RaN\xC9\x81\x84aA\xFFV[\x90P\x94\x93PPPPV[`\0aN\xDE\x82a2\xA3V[\x91PaN\xE9\x83a2\xA3V[\x92P\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aO\"WaO!a<\xF4V[[\x82\x82\x02\x90P\x92\x91PPV[\x7FInitializable: contract is alrea`\0\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aO\x89`.\x83a5\x84V[\x91PaO\x94\x82aO-V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaO\xB8\x81aO|V[\x90P\x91\x90PV[\x7FL2OutputOracle: L2 block time mu`\0\x82\x01R\x7Fst be greater than 0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aP\x1B`4\x83a5\x84V[\x91PaP&\x82aO\xBFV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaPJ\x81aP\x0EV[\x90P\x91\x90PV[\x7FL2OutputOracle: starting L2 time`\0\x82\x01R\x7Fstamp must be less than current ` \x82\x01R\x7Ftime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aP\xD3`D\x83a5\x84V[\x91PaP\xDE\x82aPQV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaQ\x02\x81aP\xC6V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xF1\xB1\xFC\x13\x06\xAF\x82\xA7`\xD2\xC9\xE4\x0E\x19\xB4\xCB\xA6\xFB\x04v\0\x8A+\x85=\xA27\xA3\xE6\xC0r\xADdsolcC\0\x08\x0F\x003",
    );
    /**```solidity
struct InitParams { address challenger; address proposer; address owner; uint256 finalizationPeriodSeconds; uint256 l2BlockTime; bytes32 aggregationVkey; bytes32 rangeVkeyCommitment; bytes32 rollupConfigHash; bytes32 startingOutputRoot; uint256 startingBlockNumber; uint256 startingTimestamp; uint256 submissionInterval; address verifier; uint256 fallbackTimeout; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InitParams {
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub l2BlockTime: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub startingOutputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub startingBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startingTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub submissionInterval: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifier: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fallbackTimeout: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InitParams> for UnderlyingRustTuple<'_> {
            fn from(value: InitParams) -> Self {
                (
                    value.challenger,
                    value.proposer,
                    value.owner,
                    value.finalizationPeriodSeconds,
                    value.l2BlockTime,
                    value.aggregationVkey,
                    value.rangeVkeyCommitment,
                    value.rollupConfigHash,
                    value.startingOutputRoot,
                    value.startingBlockNumber,
                    value.startingTimestamp,
                    value.submissionInterval,
                    value.verifier,
                    value.fallbackTimeout,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InitParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    challenger: tuple.0,
                    proposer: tuple.1,
                    owner: tuple.2,
                    finalizationPeriodSeconds: tuple.3,
                    l2BlockTime: tuple.4,
                    aggregationVkey: tuple.5,
                    rangeVkeyCommitment: tuple.6,
                    rollupConfigHash: tuple.7,
                    startingOutputRoot: tuple.8,
                    startingBlockNumber: tuple.9,
                    startingTimestamp: tuple.10,
                    submissionInterval: tuple.11,
                    verifier: tuple.12,
                    fallbackTimeout: tuple.13,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for InitParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for InitParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.finalizationPeriodSeconds,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2BlockTime),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregationVkey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rangeVkeyCommitment),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rollupConfigHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startingOutputRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startingBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startingTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.submissionInterval),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.verifier,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fallbackTimeout),
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
        impl alloy_sol_types::SolType for InitParams {
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
        impl alloy_sol_types::SolStruct for InitParams {
            const NAME: &'static str = "InitParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "InitParams(address challenger,address proposer,address owner,uint256 finalizationPeriodSeconds,uint256 l2BlockTime,bytes32 aggregationVkey,bytes32 rangeVkeyCommitment,bytes32 rollupConfigHash,bytes32 startingOutputRoot,uint256 startingBlockNumber,uint256 startingTimestamp,uint256 submissionInterval,address verifier,uint256 fallbackTimeout)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challenger,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.finalizationPeriodSeconds,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2BlockTime)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregationVkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rangeVkeyCommitment,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupConfigHash,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startingOutputRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startingBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startingTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.submissionInterval,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.verifier,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.fallbackTimeout,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for InitParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challenger,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.finalizationPeriodSeconds,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2BlockTime,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregationVkey,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rangeVkeyCommitment,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupConfigHash,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startingOutputRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startingBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startingTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.submissionInterval,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.verifier,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fallbackTimeout,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challenger,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.finalizationPeriodSeconds,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2BlockTime,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aggregationVkey,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rangeVkeyCommitment,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupConfigHash,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startingOutputRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startingBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startingTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.submissionInterval,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.verifier,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fallbackTimeout,
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
    /**```solidity
struct OpSuccinctConfig { bytes32 aggregationVkey; bytes32 rangeVkeyCommitment; bytes32 rollupConfigHash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OpSuccinctConfig {
        #[allow(missing_docs)]
        pub aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<OpSuccinctConfig> for UnderlyingRustTuple<'_> {
            fn from(value: OpSuccinctConfig) -> Self {
                (
                    value.aggregationVkey,
                    value.rangeVkeyCommitment,
                    value.rollupConfigHash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OpSuccinctConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    aggregationVkey: tuple.0,
                    rangeVkeyCommitment: tuple.1,
                    rollupConfigHash: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OpSuccinctConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OpSuccinctConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregationVkey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rangeVkeyCommitment),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rollupConfigHash),
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
        impl alloy_sol_types::SolType for OpSuccinctConfig {
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
        impl alloy_sol_types::SolStruct for OpSuccinctConfig {
            const NAME: &'static str = "OpSuccinctConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OpSuccinctConfig(bytes32 aggregationVkey,bytes32 rangeVkeyCommitment,bytes32 rollupConfigHash)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregationVkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rangeVkeyCommitment,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupConfigHash,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OpSuccinctConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregationVkey,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rangeVkeyCommitment,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupConfigHash,
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
                    &rust.aggregationVkey,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rangeVkeyCommitment,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupConfigHash,
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
    /**Custom error with signature `L1BlockHashNotAvailable()` and selector `0x84c06864`.
```solidity
error L1BlockHashNotAvailable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1BlockHashNotAvailable;
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
        impl ::core::convert::From<L1BlockHashNotAvailable> for UnderlyingRustTuple<'_> {
            fn from(value: L1BlockHashNotAvailable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L1BlockHashNotAvailable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for L1BlockHashNotAvailable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1BlockHashNotAvailable()";
            const SELECTOR: [u8; 4] = [132u8, 192u8, 104u8, 100u8];
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
    /**Custom error with signature `L1BlockHashNotCheckpointed()` and selector `0x22aa3a98`.
```solidity
error L1BlockHashNotCheckpointed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1BlockHashNotCheckpointed;
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
        impl ::core::convert::From<L1BlockHashNotCheckpointed>
        for UnderlyingRustTuple<'_> {
            fn from(value: L1BlockHashNotCheckpointed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for L1BlockHashNotCheckpointed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for L1BlockHashNotCheckpointed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1BlockHashNotCheckpointed()";
            const SELECTOR: [u8; 4] = [34u8, 170u8, 58u8, 152u8];
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
    /**Event with signature `DisputeGameFactorySet(address)` and selector `0x73702180ce348e07b058846d1745c99987ae6c741ff97ec28d4539530ef1e8f1`.
```solidity
event DisputeGameFactorySet(address indexed disputeGameFactory);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DisputeGameFactorySet {
        #[allow(missing_docs)]
        pub disputeGameFactory: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DisputeGameFactorySet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DisputeGameFactorySet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                115u8, 112u8, 33u8, 128u8, 206u8, 52u8, 142u8, 7u8, 176u8, 88u8, 132u8,
                109u8, 23u8, 69u8, 201u8, 153u8, 135u8, 174u8, 108u8, 116u8, 31u8, 249u8,
                126u8, 194u8, 141u8, 69u8, 57u8, 83u8, 14u8, 241u8, 232u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    disputeGameFactory: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.disputeGameFactory.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.disputeGameFactory,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DisputeGameFactorySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisputeGameFactorySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DisputeGameFactorySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
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
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OpSuccinctConfigDeleted(bytes32)` and selector `0x4432b02a2fcbed48d94e8d72723e155c6690e4b7f39afa41a2a8ff8c0aa425da`.
```solidity
event OpSuccinctConfigDeleted(bytes32 indexed configName);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OpSuccinctConfigDeleted {
        #[allow(missing_docs)]
        pub configName: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OpSuccinctConfigDeleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OpSuccinctConfigDeleted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8, 50u8, 176u8, 42u8, 47u8, 203u8, 237u8, 72u8, 217u8, 78u8, 141u8,
                114u8, 114u8, 62u8, 21u8, 92u8, 102u8, 144u8, 228u8, 183u8, 243u8, 154u8,
                250u8, 65u8, 162u8, 168u8, 255u8, 140u8, 10u8, 164u8, 37u8, 218u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { configName: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.configName.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.configName);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OpSuccinctConfigDeleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OpSuccinctConfigDeleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OpSuccinctConfigDeleted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OpSuccinctConfigUpdated(bytes32,bytes32,bytes32,bytes32)` and selector `0xea0123c726a665cb0ab5691444f929a7056c7a7709c60c0587829e8046b8d514`.
```solidity
event OpSuccinctConfigUpdated(bytes32 indexed configName, bytes32 aggregationVkey, bytes32 rangeVkeyCommitment, bytes32 rollupConfigHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OpSuccinctConfigUpdated {
        #[allow(missing_docs)]
        pub configName: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OpSuccinctConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OpSuccinctConfigUpdated(bytes32,bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                234u8, 1u8, 35u8, 199u8, 38u8, 166u8, 101u8, 203u8, 10u8, 181u8, 105u8,
                20u8, 68u8, 249u8, 41u8, 167u8, 5u8, 108u8, 122u8, 119u8, 9u8, 198u8,
                12u8, 5u8, 135u8, 130u8, 158u8, 128u8, 70u8, 184u8, 213u8, 20u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    configName: topics.1,
                    aggregationVkey: data.0,
                    rangeVkeyCommitment: data.1,
                    rollupConfigHash: data.2,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregationVkey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rangeVkeyCommitment),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rollupConfigHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.configName.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.configName);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OpSuccinctConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OpSuccinctConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OpSuccinctConfigUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OptimisticModeToggled(bool,uint256)` and selector `0x1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a`.
```solidity
event OptimisticModeToggled(bool indexed enabled, uint256 finalizationPeriodSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OptimisticModeToggled {
        #[allow(missing_docs)]
        pub enabled: bool,
        #[allow(missing_docs)]
        pub finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OptimisticModeToggled {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            const SIGNATURE: &'static str = "OptimisticModeToggled(bool,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                31u8, 92u8, 135u8, 47u8, 30u8, 169u8, 60u8, 87u8, 228u8, 49u8, 18u8,
                234u8, 68u8, 158u8, 225u8, 158u8, 245u8, 117u8, 68u8, 136u8, 184u8,
                118u8, 39u8, 180u8, 197u8, 36u8, 86u8, 176u8, 229u8, 164u8, 16u8, 154u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    enabled: topics.1,
                    finalizationPeriodSeconds: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.finalizationPeriodSeconds,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.enabled.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
                    &self.enabled,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OptimisticModeToggled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OptimisticModeToggled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OptimisticModeToggled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OutputProposed(bytes32,uint256,uint256,uint256)` and selector `0xa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2`.
```solidity
event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OutputProposed {
        #[allow(missing_docs)]
        pub outputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub l2OutputIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub l1Timestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OutputProposed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "OutputProposed(bytes32,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8, 170u8, 242u8, 81u8, 39u8, 105u8, 218u8, 78u8, 68u8, 78u8, 61u8,
                226u8, 71u8, 190u8, 37u8, 100u8, 34u8, 92u8, 46u8, 122u8, 143u8, 116u8,
                207u8, 229u8, 40u8, 228u8, 110u8, 23u8, 210u8, 72u8, 104u8, 226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    outputRoot: topics.1,
                    l2OutputIndex: topics.2,
                    l2BlockNumber: topics.3,
                    l1Timestamp: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l1Timestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.outputRoot.clone(),
                    self.l2OutputIndex.clone(),
                    self.l2BlockNumber.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.outputRoot);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.l2OutputIndex);
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.l2BlockNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OutputProposed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OutputProposed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OutputProposed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OutputsDeleted(uint256,uint256)` and selector `0x4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b6`.
```solidity
event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OutputsDeleted {
        #[allow(missing_docs)]
        pub prevNextOutputIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newNextOutputIndex: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OutputsDeleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "OutputsDeleted(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                78u8, 227u8, 122u8, 194u8, 199u8, 134u8, 236u8, 133u8, 232u8, 117u8,
                146u8, 211u8, 197u8, 200u8, 161u8, 221u8, 102u8, 248u8, 73u8, 109u8,
                218u8, 63u8, 18u8, 93u8, 158u8, 168u8, 202u8, 95u8, 101u8, 118u8, 41u8,
                182u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevNextOutputIndex: topics.1,
                    newNextOutputIndex: topics.2,
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
                    self.prevNextOutputIndex.clone(),
                    self.newNextOutputIndex.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.prevNextOutputIndex,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newNextOutputIndex,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OutputsDeleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OutputsDeleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OutputsDeleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposerUpdated(address,bool)` and selector `0x5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a`.
```solidity
event ProposerUpdated(address indexed proposer, bool added);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposerUpdated {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub added: bool,
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
        impl alloy_sol_types::SolEvent for ProposerUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ProposerUpdated(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 243u8, 141u8, 57u8, 94u8, 220u8, 21u8, 182u8, 105u8, 214u8, 70u8,
                86u8, 155u8, 208u8, 21u8, 81u8, 51u8, 149u8, 7u8, 11u8, 91u8, 77u8,
                235u8, 138u8, 22u8, 48u8, 10u8, 187u8, 6u8, 13u8, 27u8, 90u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    proposer: topics.1,
                    added: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.added,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.proposer.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.proposer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposerUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposerUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposerUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SubmissionIntervalUpdated(uint256,uint256)` and selector `0xc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b2`.
```solidity
event SubmissionIntervalUpdated(uint256 oldSubmissionInterval, uint256 newSubmissionInterval);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubmissionIntervalUpdated {
        #[allow(missing_docs)]
        pub oldSubmissionInterval: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSubmissionInterval: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for SubmissionIntervalUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SubmissionIntervalUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                193u8, 191u8, 154u8, 191u8, 181u8, 126u8, 160u8, 30u8, 217u8, 236u8,
                180u8, 244u8, 94u8, 156u8, 239u8, 167u8, 186u8, 68u8, 178u8, 230u8,
                119u8, 140u8, 60u8, 231u8, 40u8, 20u8, 9u8, 153u8, 159u8, 26u8, 241u8,
                178u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldSubmissionInterval: data.0,
                    newSubmissionInterval: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.oldSubmissionInterval,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newSubmissionInterval),
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
        impl alloy_sol_types::private::IntoLogData for SubmissionIntervalUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubmissionIntervalUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SubmissionIntervalUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `VerifierUpdated(address,address)` and selector `0x0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc`.
```solidity
event VerifierUpdated(address indexed oldVerifier, address indexed newVerifier);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct VerifierUpdated {
        #[allow(missing_docs)]
        pub oldVerifier: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newVerifier: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for VerifierUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "VerifierUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8, 67u8, 84u8, 154u8, 146u8, 178u8, 65u8, 47u8, 122u8, 60u8, 175u8,
                122u8, 46u8, 86u8, 214u8, 91u8, 136u8, 33u8, 185u8, 19u8, 69u8, 54u8,
                63u8, 170u8, 95u8, 87u8, 25u8, 83u8, 132u8, 6u8, 95u8, 204u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldVerifier: topics.1,
                    newVerifier: topics.2,
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
                    self.oldVerifier.clone(),
                    self.newVerifier.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldVerifier,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newVerifier,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for VerifierUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&VerifierUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &VerifierUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        }
    };
    /**Function with signature `GENESIS_CONFIG_NAME()` and selector `0xf72f606d`.
```solidity
function GENESIS_CONFIG_NAME() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_CONFIG_NAMECall;
    ///Container type for the return parameters of the [`GENESIS_CONFIG_NAME()`](GENESIS_CONFIG_NAMECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_CONFIG_NAMEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<GENESIS_CONFIG_NAMECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_CONFIG_NAMECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GENESIS_CONFIG_NAMECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<GENESIS_CONFIG_NAMEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_CONFIG_NAMEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GENESIS_CONFIG_NAMEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENESIS_CONFIG_NAMECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENESIS_CONFIG_NAME()";
            const SELECTOR: [u8; 4] = [247u8, 47u8, 96u8, 109u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: GENESIS_CONFIG_NAMEReturn = r.into();
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
                        let r: GENESIS_CONFIG_NAMEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `addOpSuccinctConfig(bytes32,bytes32,bytes32,bytes32)` and selector `0x47c37e9c`.
```solidity
function addOpSuccinctConfig(bytes32 _configName, bytes32 _rollupConfigHash, bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOpSuccinctConfigCall {
        #[allow(missing_docs)]
        pub _configName: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`addOpSuccinctConfig(bytes32,bytes32,bytes32,bytes32)`](addOpSuccinctConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOpSuccinctConfigReturn {}
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
            impl ::core::convert::From<addOpSuccinctConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addOpSuccinctConfigCall) -> Self {
                    (
                        value._configName,
                        value._rollupConfigHash,
                        value._aggregationVkey,
                        value._rangeVkeyCommitment,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addOpSuccinctConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _configName: tuple.0,
                        _rollupConfigHash: tuple.1,
                        _aggregationVkey: tuple.2,
                        _rangeVkeyCommitment: tuple.3,
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
            impl ::core::convert::From<addOpSuccinctConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addOpSuccinctConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addOpSuccinctConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addOpSuccinctConfigReturn {
            fn _tokenize(
                &self,
            ) -> <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addOpSuccinctConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addOpSuccinctConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addOpSuccinctConfig(bytes32,bytes32,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [71u8, 195u8, 126u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._configName),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._rollupConfigHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._aggregationVkey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._rangeVkeyCommitment),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addOpSuccinctConfigReturn::_tokenize(ret)
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
    /**Function with signature `addProposer(address)` and selector `0xb03cd418`.
```solidity
function addProposer(address _proposer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addProposerCall {
        #[allow(missing_docs)]
        pub _proposer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addProposer(address)`](addProposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addProposerReturn {}
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
            impl ::core::convert::From<addProposerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addProposerCall) -> Self {
                    (value._proposer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addProposerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _proposer: tuple.0 }
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
            impl ::core::convert::From<addProposerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addProposerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addProposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addProposerReturn {
            fn _tokenize(
                &self,
            ) -> <addProposerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addProposerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addProposerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addProposer(address)";
            const SELECTOR: [u8; 4] = [176u8, 60u8, 212u8, 24u8];
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
                        &self._proposer,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addProposerReturn::_tokenize(ret)
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
    /**Function with signature `aggregationVkey()` and selector `0xc32e4e3e`.
```solidity
function aggregationVkey() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregationVkeyCall;
    ///Container type for the return parameters of the [`aggregationVkey()`](aggregationVkeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregationVkeyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<aggregationVkeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: aggregationVkeyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregationVkeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<aggregationVkeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: aggregationVkeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for aggregationVkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregationVkeyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregationVkey()";
            const SELECTOR: [u8; 4] = [195u8, 46u8, 78u8, 62u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: aggregationVkeyReturn = r.into();
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
                        let r: aggregationVkeyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `approvedProposers(address)` and selector `0xd4651276`.
```solidity
function approvedProposers(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approvedProposersCall(pub alloy::sol_types::private::Address);
    ///Container type for the return parameters of the [`approvedProposers(address)`](approvedProposersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approvedProposersReturn {
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
            impl ::core::convert::From<approvedProposersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: approvedProposersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for approvedProposersCall {
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
            impl ::core::convert::From<approvedProposersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: approvedProposersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for approvedProposersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approvedProposersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approvedProposers(address)";
            const SELECTOR: [u8; 4] = [212u8, 101u8, 18u8, 118u8];
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
                        let r: approvedProposersReturn = r.into();
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
                        let r: approvedProposersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `challenger()` and selector `0x534db0e2`.
```solidity
function challenger() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengerCall;
    ///Container type for the return parameters of the [`challenger()`](challengerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengerReturn {
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
            impl ::core::convert::From<challengerCall> for UnderlyingRustTuple<'_> {
                fn from(value: challengerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengerCall {
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
            impl ::core::convert::From<challengerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: challengerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challenger()";
            const SELECTOR: [u8; 4] = [83u8, 77u8, 176u8, 226u8];
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
                        let r: challengerReturn = r.into();
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
                        let r: challengerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `checkpointBlockHash(uint256)` and selector `0x1e856800`.
```solidity
function checkpointBlockHash(uint256 _blockNumber) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBlockHashCall {
        #[allow(missing_docs)]
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`checkpointBlockHash(uint256)`](checkpointBlockHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBlockHashReturn {}
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
            impl ::core::convert::From<checkpointBlockHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBlockHashCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkpointBlockHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
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
            impl ::core::convert::From<checkpointBlockHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBlockHashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkpointBlockHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkpointBlockHashReturn {
            fn _tokenize(
                &self,
            ) -> <checkpointBlockHashCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointBlockHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkpointBlockHashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpointBlockHash(uint256)";
            const SELECTOR: [u8; 4] = [30u8, 133u8, 104u8, 0u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkpointBlockHashReturn::_tokenize(ret)
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
    /**Function with signature `computeL2Timestamp(uint256)` and selector `0xd1de856c`.
```solidity
function computeL2Timestamp(uint256 _l2BlockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeL2TimestampCall {
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`computeL2Timestamp(uint256)`](computeL2TimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeL2TimestampReturn {
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
            impl ::core::convert::From<computeL2TimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeL2TimestampCall) -> Self {
                    (value._l2BlockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeL2TimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2BlockNumber: tuple.0 }
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
            impl ::core::convert::From<computeL2TimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeL2TimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeL2TimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for computeL2TimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "computeL2Timestamp(uint256)";
            const SELECTOR: [u8; 4] = [209u8, 222u8, 133u8, 108u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
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
                        let r: computeL2TimestampReturn = r.into();
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
                        let r: computeL2TimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `deleteL2Outputs(uint256)` and selector `0x89c44cbb`.
```solidity
function deleteL2Outputs(uint256 _l2OutputIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deleteL2OutputsCall {
        #[allow(missing_docs)]
        pub _l2OutputIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deleteL2Outputs(uint256)`](deleteL2OutputsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deleteL2OutputsReturn {}
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
            impl ::core::convert::From<deleteL2OutputsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deleteL2OutputsCall) -> Self {
                    (value._l2OutputIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deleteL2OutputsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2OutputIndex: tuple.0 }
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
            impl ::core::convert::From<deleteL2OutputsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deleteL2OutputsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deleteL2OutputsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl deleteL2OutputsReturn {
            fn _tokenize(
                &self,
            ) -> <deleteL2OutputsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deleteL2OutputsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deleteL2OutputsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deleteL2Outputs(uint256)";
            const SELECTOR: [u8; 4] = [137u8, 196u8, 76u8, 187u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2OutputIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                deleteL2OutputsReturn::_tokenize(ret)
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
    /**Function with signature `deleteOpSuccinctConfig(bytes32)` and selector `0xec5b2e3a`.
```solidity
function deleteOpSuccinctConfig(bytes32 _configName) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deleteOpSuccinctConfigCall {
        #[allow(missing_docs)]
        pub _configName: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`deleteOpSuccinctConfig(bytes32)`](deleteOpSuccinctConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deleteOpSuccinctConfigReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<deleteOpSuccinctConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deleteOpSuccinctConfigCall) -> Self {
                    (value._configName,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deleteOpSuccinctConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _configName: tuple.0 }
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
            impl ::core::convert::From<deleteOpSuccinctConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deleteOpSuccinctConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deleteOpSuccinctConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl deleteOpSuccinctConfigReturn {
            fn _tokenize(
                &self,
            ) -> <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deleteOpSuccinctConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deleteOpSuccinctConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deleteOpSuccinctConfig(bytes32)";
            const SELECTOR: [u8; 4] = [236u8, 91u8, 46u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._configName),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                deleteOpSuccinctConfigReturn::_tokenize(ret)
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
    /**Function with signature `dgfProposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)` and selector `0x7a41a035`.
```solidity
function dgfProposeL2Output(bytes32 _configName, bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof, address _proverAddress) external payable returns (address _game);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dgfProposeL2OutputCall {
        #[allow(missing_docs)]
        pub _configName: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _outputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _proof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _proverAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`dgfProposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)`](dgfProposeL2OutputCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dgfProposeL2OutputReturn {
        #[allow(missing_docs)]
        pub _game: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<dgfProposeL2OutputCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: dgfProposeL2OutputCall) -> Self {
                    (
                        value._configName,
                        value._outputRoot,
                        value._l2BlockNumber,
                        value._l1BlockNumber,
                        value._proof,
                        value._proverAddress,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for dgfProposeL2OutputCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _configName: tuple.0,
                        _outputRoot: tuple.1,
                        _l2BlockNumber: tuple.2,
                        _l1BlockNumber: tuple.3,
                        _proof: tuple.4,
                        _proverAddress: tuple.5,
                    }
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
            impl ::core::convert::From<dgfProposeL2OutputReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: dgfProposeL2OutputReturn) -> Self {
                    (value._game,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for dgfProposeL2OutputReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _game: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for dgfProposeL2OutputCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "dgfProposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)";
            const SELECTOR: [u8; 4] = [122u8, 65u8, 160u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._configName),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._outputRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l1BlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._proof,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._proverAddress,
                    ),
                )
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
                        let r: dgfProposeL2OutputReturn = r.into();
                        r._game
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
                        let r: dgfProposeL2OutputReturn = r.into();
                        r._game
                    })
            }
        }
    };
    /**Function with signature `disableOptimisticMode(uint256)` and selector `0x4ab309ac`.
```solidity
function disableOptimisticMode(uint256 _finalizationPeriodSeconds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableOptimisticModeCall {
        #[allow(missing_docs)]
        pub _finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`disableOptimisticMode(uint256)`](disableOptimisticModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableOptimisticModeReturn {}
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
            impl ::core::convert::From<disableOptimisticModeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableOptimisticModeCall) -> Self {
                    (value._finalizationPeriodSeconds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableOptimisticModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _finalizationPeriodSeconds: tuple.0,
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
            impl ::core::convert::From<disableOptimisticModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableOptimisticModeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableOptimisticModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disableOptimisticModeReturn {
            fn _tokenize(
                &self,
            ) -> <disableOptimisticModeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableOptimisticModeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableOptimisticModeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableOptimisticMode(uint256)";
            const SELECTOR: [u8; 4] = [74u8, 179u8, 9u8, 172u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._finalizationPeriodSeconds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                disableOptimisticModeReturn::_tokenize(ret)
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
    /**Function with signature `disputeGameFactory()` and selector `0xf2b4e617`.
```solidity
function disputeGameFactory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFactoryCall;
    ///Container type for the return parameters of the [`disputeGameFactory()`](disputeGameFactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFactoryReturn {
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
            impl ::core::convert::From<disputeGameFactoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFactoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFactoryCall {
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
            impl ::core::convert::From<disputeGameFactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFactoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disputeGameFactoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disputeGameFactory()";
            const SELECTOR: [u8; 4] = [242u8, 180u8, 230u8, 23u8];
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
                        let r: disputeGameFactoryReturn = r.into();
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
                        let r: disputeGameFactoryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `enableOptimisticMode(uint256)` and selector `0x2c697961`.
```solidity
function enableOptimisticMode(uint256 _finalizationPeriodSeconds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableOptimisticModeCall {
        #[allow(missing_docs)]
        pub _finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`enableOptimisticMode(uint256)`](enableOptimisticModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableOptimisticModeReturn {}
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
            impl ::core::convert::From<enableOptimisticModeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableOptimisticModeCall) -> Self {
                    (value._finalizationPeriodSeconds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableOptimisticModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _finalizationPeriodSeconds: tuple.0,
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
            impl ::core::convert::From<enableOptimisticModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableOptimisticModeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableOptimisticModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl enableOptimisticModeReturn {
            fn _tokenize(
                &self,
            ) -> <enableOptimisticModeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableOptimisticModeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableOptimisticModeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableOptimisticMode(uint256)";
            const SELECTOR: [u8; 4] = [44u8, 105u8, 121u8, 97u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._finalizationPeriodSeconds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                enableOptimisticModeReturn::_tokenize(ret)
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
    /**Function with signature `fallbackTimeout()` and selector `0x4277bc06`.
```solidity
function fallbackTimeout() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fallbackTimeoutCall;
    ///Container type for the return parameters of the [`fallbackTimeout()`](fallbackTimeoutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fallbackTimeoutReturn {
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
            impl ::core::convert::From<fallbackTimeoutCall> for UnderlyingRustTuple<'_> {
                fn from(value: fallbackTimeoutCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fallbackTimeoutCall {
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
            impl ::core::convert::From<fallbackTimeoutReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fallbackTimeoutReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fallbackTimeoutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fallbackTimeoutCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fallbackTimeout()";
            const SELECTOR: [u8; 4] = [66u8, 119u8, 188u8, 6u8];
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
                        let r: fallbackTimeoutReturn = r.into();
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
                        let r: fallbackTimeoutReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `finalizationPeriodSeconds()` and selector `0xce5db8d6`.
```solidity
function finalizationPeriodSeconds() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizationPeriodSecondsCall;
    ///Container type for the return parameters of the [`finalizationPeriodSeconds()`](finalizationPeriodSecondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizationPeriodSecondsReturn {
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
            impl ::core::convert::From<finalizationPeriodSecondsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizationPeriodSecondsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizationPeriodSecondsCall {
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
            impl ::core::convert::From<finalizationPeriodSecondsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizationPeriodSecondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizationPeriodSecondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizationPeriodSecondsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizationPeriodSeconds()";
            const SELECTOR: [u8; 4] = [206u8, 93u8, 184u8, 214u8];
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
                        let r: finalizationPeriodSecondsReturn = r.into();
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
                        let r: finalizationPeriodSecondsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `getL2Output(uint256)` and selector `0xa25ae557`.
```solidity
function getL2Output(uint256 _l2OutputIndex) external view returns (Types.OutputProposal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputCall {
        #[allow(missing_docs)]
        pub _l2OutputIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getL2Output(uint256)`](getL2OutputCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputReturn {
        #[allow(missing_docs)]
        pub _0: <Types::OutputProposal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getL2OutputCall> for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputCall) -> Self {
                    (value._l2OutputIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getL2OutputCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2OutputIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Types::OutputProposal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::OutputProposal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getL2OutputReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getL2OutputReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getL2OutputCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Types::OutputProposal as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Types::OutputProposal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getL2Output(uint256)";
            const SELECTOR: [u8; 4] = [162u8, 90u8, 229u8, 87u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2OutputIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Types::OutputProposal as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getL2OutputReturn = r.into();
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
                        let r: getL2OutputReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `getL2OutputAfter(uint256)` and selector `0xcf8e5cf0`.
```solidity
function getL2OutputAfter(uint256 _l2BlockNumber) external view returns (Types.OutputProposal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputAfterCall {
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getL2OutputAfter(uint256)`](getL2OutputAfterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputAfterReturn {
        #[allow(missing_docs)]
        pub _0: <Types::OutputProposal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getL2OutputAfterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputAfterCall) -> Self {
                    (value._l2BlockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getL2OutputAfterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2BlockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Types::OutputProposal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::OutputProposal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getL2OutputAfterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputAfterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getL2OutputAfterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getL2OutputAfterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Types::OutputProposal as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Types::OutputProposal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getL2OutputAfter(uint256)";
            const SELECTOR: [u8; 4] = [207u8, 142u8, 92u8, 240u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Types::OutputProposal as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getL2OutputAfterReturn = r.into();
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
                        let r: getL2OutputAfterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `getL2OutputIndexAfter(uint256)` and selector `0x7f006420`.
```solidity
function getL2OutputIndexAfter(uint256 _l2BlockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputIndexAfterCall {
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getL2OutputIndexAfter(uint256)`](getL2OutputIndexAfterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getL2OutputIndexAfterReturn {
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
            impl ::core::convert::From<getL2OutputIndexAfterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputIndexAfterCall) -> Self {
                    (value._l2BlockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getL2OutputIndexAfterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2BlockNumber: tuple.0 }
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
            impl ::core::convert::From<getL2OutputIndexAfterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getL2OutputIndexAfterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getL2OutputIndexAfterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getL2OutputIndexAfterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getL2OutputIndexAfter(uint256)";
            const SELECTOR: [u8; 4] = [127u8, 0u8, 100u8, 32u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
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
                        let r: getL2OutputIndexAfterReturn = r.into();
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
                        let r: getL2OutputIndexAfterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `historicBlockHashes(uint256)` and selector `0xa196b525`.
```solidity
function historicBlockHashes(uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct historicBlockHashesCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    ///Container type for the return parameters of the [`historicBlockHashes(uint256)`](historicBlockHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct historicBlockHashesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<historicBlockHashesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: historicBlockHashesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for historicBlockHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<historicBlockHashesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: historicBlockHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for historicBlockHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for historicBlockHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "historicBlockHashes(uint256)";
            const SELECTOR: [u8; 4] = [161u8, 150u8, 181u8, 37u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: historicBlockHashesReturn = r.into();
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
                        let r: historicBlockHashesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address,uint256))` and selector `0xe40b7a12`.
```solidity
function initialize(InitParams memory _initParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _initParams: <InitParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address,uint256))`](initializeCall) function.
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
            type UnderlyingSolTuple<'a> = (InitParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <InitParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._initParams,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _initParams: tuple.0 }
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
            type Parameters<'a> = (InitParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address,uint256))";
            const SELECTOR: [u8; 4] = [228u8, 11u8, 122u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<InitParams as alloy_sol_types::SolType>::tokenize(&self._initParams),)
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
    /**Function with signature `initializerVersion()` and selector `0x7f01ea68`.
```solidity
function initializerVersion() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializerVersionCall;
    ///Container type for the return parameters of the [`initializerVersion()`](initializerVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializerVersionReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<initializerVersionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializerVersionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializerVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<initializerVersionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializerVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializerVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializerVersionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializerVersion()";
            const SELECTOR: [u8; 4] = [127u8, 1u8, 234u8, 104u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: initializerVersionReturn = r.into();
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
                        let r: initializerVersionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `isValidOpSuccinctConfig((bytes32,bytes32,bytes32))` and selector `0x49185e06`.
```solidity
function isValidOpSuccinctConfig(OpSuccinctConfig memory _config) external pure returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidOpSuccinctConfigCall {
        #[allow(missing_docs)]
        pub _config: <OpSuccinctConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isValidOpSuccinctConfig((bytes32,bytes32,bytes32))`](isValidOpSuccinctConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidOpSuccinctConfigReturn {
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
            type UnderlyingSolTuple<'a> = (OpSuccinctConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OpSuccinctConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isValidOpSuccinctConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidOpSuccinctConfigCall) -> Self {
                    (value._config,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidOpSuccinctConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _config: tuple.0 }
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
            impl ::core::convert::From<isValidOpSuccinctConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidOpSuccinctConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidOpSuccinctConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidOpSuccinctConfigCall {
            type Parameters<'a> = (OpSuccinctConfig,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidOpSuccinctConfig((bytes32,bytes32,bytes32))";
            const SELECTOR: [u8; 4] = [73u8, 24u8, 94u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OpSuccinctConfig as alloy_sol_types::SolType>::tokenize(
                        &self._config,
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
                        let r: isValidOpSuccinctConfigReturn = r.into();
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
                        let r: isValidOpSuccinctConfigReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `l2BlockTime()` and selector `0x93991af3`.
```solidity
function l2BlockTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockTimeCall;
    ///Container type for the return parameters of the [`l2BlockTime()`](l2BlockTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockTimeReturn {
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
            impl ::core::convert::From<l2BlockTimeCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockTimeCall {
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
            impl ::core::convert::From<l2BlockTimeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockTime()";
            const SELECTOR: [u8; 4] = [147u8, 153u8, 26u8, 243u8];
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
                        let r: l2BlockTimeReturn = r.into();
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
                        let r: l2BlockTimeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `lastProposalTimestamp()` and selector `0xe0c2f935`.
```solidity
function lastProposalTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProposalTimestampCall;
    ///Container type for the return parameters of the [`lastProposalTimestamp()`](lastProposalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProposalTimestampReturn {
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
            impl ::core::convert::From<lastProposalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProposalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProposalTimestampCall {
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
            impl ::core::convert::From<lastProposalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProposalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProposalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProposalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProposalTimestamp()";
            const SELECTOR: [u8; 4] = [224u8, 194u8, 249u8, 53u8];
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
                        let r: lastProposalTimestampReturn = r.into();
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
                        let r: lastProposalTimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `latestBlockNumber()` and selector `0x4599c788`.
```solidity
function latestBlockNumber() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestBlockNumberCall;
    ///Container type for the return parameters of the [`latestBlockNumber()`](latestBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestBlockNumberReturn {
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
            impl ::core::convert::From<latestBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestBlockNumberCall {
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
            impl ::core::convert::From<latestBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestBlockNumber()";
            const SELECTOR: [u8; 4] = [69u8, 153u8, 199u8, 136u8];
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
                        let r: latestBlockNumberReturn = r.into();
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
                        let r: latestBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `latestOutputIndex()` and selector `0x69f16eec`.
```solidity
function latestOutputIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestOutputIndexCall;
    ///Container type for the return parameters of the [`latestOutputIndex()`](latestOutputIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestOutputIndexReturn {
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
            impl ::core::convert::From<latestOutputIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestOutputIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestOutputIndexCall {
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
            impl ::core::convert::From<latestOutputIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestOutputIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestOutputIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestOutputIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestOutputIndex()";
            const SELECTOR: [u8; 4] = [105u8, 241u8, 110u8, 236u8];
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
                        let r: latestOutputIndexReturn = r.into();
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
                        let r: latestOutputIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `nextBlockNumber()` and selector `0xdcec3348`.
```solidity
function nextBlockNumber() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextBlockNumberCall;
    ///Container type for the return parameters of the [`nextBlockNumber()`](nextBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextBlockNumberReturn {
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
            impl ::core::convert::From<nextBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextBlockNumberCall {
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
            impl ::core::convert::From<nextBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextBlockNumber()";
            const SELECTOR: [u8; 4] = [220u8, 236u8, 51u8, 72u8];
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
                        let r: nextBlockNumberReturn = r.into();
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
                        let r: nextBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `nextOutputIndex()` and selector `0x6abcf563`.
```solidity
function nextOutputIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextOutputIndexCall;
    ///Container type for the return parameters of the [`nextOutputIndex()`](nextOutputIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextOutputIndexReturn {
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
            impl ::core::convert::From<nextOutputIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextOutputIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextOutputIndexCall {
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
            impl ::core::convert::From<nextOutputIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextOutputIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextOutputIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextOutputIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextOutputIndex()";
            const SELECTOR: [u8; 4] = [106u8, 188u8, 245u8, 99u8];
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
                        let r: nextOutputIndexReturn = r.into();
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
                        let r: nextOutputIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `opSuccinctConfigs(bytes32)` and selector `0x6a56620b`.
```solidity
function opSuccinctConfigs(bytes32) external view returns (bytes32 aggregationVkey, bytes32 rangeVkeyCommitment, bytes32 rollupConfigHash);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opSuccinctConfigsCall(pub alloy::sol_types::private::FixedBytes<32>);
    ///Container type for the return parameters of the [`opSuccinctConfigs(bytes32)`](opSuccinctConfigsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opSuccinctConfigsReturn {
        #[allow(missing_docs)]
        pub aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<opSuccinctConfigsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: opSuccinctConfigsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opSuccinctConfigsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<opSuccinctConfigsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: opSuccinctConfigsReturn) -> Self {
                    (
                        value.aggregationVkey,
                        value.rangeVkeyCommitment,
                        value.rollupConfigHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opSuccinctConfigsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        aggregationVkey: tuple.0,
                        rangeVkeyCommitment: tuple.1,
                        rollupConfigHash: tuple.2,
                    }
                }
            }
        }
        impl opSuccinctConfigsReturn {
            fn _tokenize(
                &self,
            ) -> <opSuccinctConfigsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregationVkey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rangeVkeyCommitment),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rollupConfigHash),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opSuccinctConfigsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = opSuccinctConfigsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opSuccinctConfigs(bytes32)";
            const SELECTOR: [u8; 4] = [106u8, 86u8, 98u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                opSuccinctConfigsReturn::_tokenize(ret)
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
    /**Function with signature `optimisticMode()` and selector `0x60caf7a0`.
```solidity
function optimisticMode() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimisticModeCall;
    ///Container type for the return parameters of the [`optimisticMode()`](optimisticModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimisticModeReturn {
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
            impl ::core::convert::From<optimisticModeCall> for UnderlyingRustTuple<'_> {
                fn from(value: optimisticModeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for optimisticModeCall {
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
            impl ::core::convert::From<optimisticModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: optimisticModeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for optimisticModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for optimisticModeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "optimisticMode()";
            const SELECTOR: [u8; 4] = [96u8, 202u8, 247u8, 160u8];
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
                        let r: optimisticModeReturn = r.into();
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
                        let r: optimisticModeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                        let r: ownerReturn = r.into();
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
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `proposeL2Output(bytes32,uint256,bytes32,uint256)` and selector `0x9aaab648`.
```solidity
function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, bytes32 _l1BlockHash, uint256 _l1BlockNumber) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeL2Output_0Call {
        #[allow(missing_docs)]
        pub _outputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _l1BlockHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposeL2Output(bytes32,uint256,bytes32,uint256)`](proposeL2Output_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeL2Output_0Return {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<proposeL2Output_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeL2Output_0Call) -> Self {
                    (
                        value._outputRoot,
                        value._l2BlockNumber,
                        value._l1BlockHash,
                        value._l1BlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeL2Output_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _outputRoot: tuple.0,
                        _l2BlockNumber: tuple.1,
                        _l1BlockHash: tuple.2,
                        _l1BlockNumber: tuple.3,
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
            impl ::core::convert::From<proposeL2Output_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeL2Output_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeL2Output_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proposeL2Output_0Return {
            fn _tokenize(
                &self,
            ) -> <proposeL2Output_0Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeL2Output_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeL2Output_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeL2Output(bytes32,uint256,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [154u8, 170u8, 182u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._outputRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._l1BlockHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l1BlockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proposeL2Output_0Return::_tokenize(ret)
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
    /**Function with signature `proposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)` and selector `0xa4ee9d7b`.
```solidity
function proposeL2Output(bytes32 _configName, bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof, address _proverAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeL2Output_1Call {
        #[allow(missing_docs)]
        pub _configName: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _outputRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _proof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _proverAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`proposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)`](proposeL2Output_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeL2Output_1Return {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<proposeL2Output_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeL2Output_1Call) -> Self {
                    (
                        value._configName,
                        value._outputRoot,
                        value._l2BlockNumber,
                        value._l1BlockNumber,
                        value._proof,
                        value._proverAddress,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeL2Output_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _configName: tuple.0,
                        _outputRoot: tuple.1,
                        _l2BlockNumber: tuple.2,
                        _l1BlockNumber: tuple.3,
                        _proof: tuple.4,
                        _proverAddress: tuple.5,
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
            impl ::core::convert::From<proposeL2Output_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeL2Output_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeL2Output_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proposeL2Output_1Return {
            fn _tokenize(
                &self,
            ) -> <proposeL2Output_1Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeL2Output_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeL2Output_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeL2Output(bytes32,bytes32,uint256,uint256,bytes,address)";
            const SELECTOR: [u8; 4] = [164u8, 238u8, 157u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._configName),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._outputRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l2BlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l1BlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._proof,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._proverAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proposeL2Output_1Return::_tokenize(ret)
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
    /**Function with signature `proposer()` and selector `0xa8e4fb90`.
```solidity
function proposer() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposerCall;
    ///Container type for the return parameters of the [`proposer()`](proposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposerReturn {
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
            impl ::core::convert::From<proposerCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposerCall {
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
            impl ::core::convert::From<proposerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposer()";
            const SELECTOR: [u8; 4] = [168u8, 228u8, 251u8, 144u8];
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
                        let r: proposerReturn = r.into();
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
                        let r: proposerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `rangeVkeyCommitment()` and selector `0x2b31841e`.
```solidity
function rangeVkeyCommitment() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rangeVkeyCommitmentCall;
    ///Container type for the return parameters of the [`rangeVkeyCommitment()`](rangeVkeyCommitmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rangeVkeyCommitmentReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<rangeVkeyCommitmentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rangeVkeyCommitmentCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rangeVkeyCommitmentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<rangeVkeyCommitmentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rangeVkeyCommitmentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rangeVkeyCommitmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rangeVkeyCommitmentCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rangeVkeyCommitment()";
            const SELECTOR: [u8; 4] = [43u8, 49u8, 132u8, 30u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: rangeVkeyCommitmentReturn = r.into();
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
                        let r: rangeVkeyCommitmentReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `removeProposer(address)` and selector `0x09d632d3`.
```solidity
function removeProposer(address _proposer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeProposerCall {
        #[allow(missing_docs)]
        pub _proposer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeProposer(address)`](removeProposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeProposerReturn {}
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
            impl ::core::convert::From<removeProposerCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeProposerCall) -> Self {
                    (value._proposer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeProposerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _proposer: tuple.0 }
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
            impl ::core::convert::From<removeProposerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeProposerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeProposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeProposerReturn {
            fn _tokenize(
                &self,
            ) -> <removeProposerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeProposerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeProposerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeProposer(address)";
            const SELECTOR: [u8; 4] = [9u8, 214u8, 50u8, 211u8];
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
                        &self._proposer,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeProposerReturn::_tokenize(ret)
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
    /**Function with signature `rollupConfigHash()` and selector `0x6d9a1c8b`.
```solidity
function rollupConfigHash() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupConfigHashCall;
    ///Container type for the return parameters of the [`rollupConfigHash()`](rollupConfigHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupConfigHashReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<rollupConfigHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupConfigHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupConfigHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<rollupConfigHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupConfigHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupConfigHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rollupConfigHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rollupConfigHash()";
            const SELECTOR: [u8; 4] = [109u8, 154u8, 28u8, 139u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: rollupConfigHashReturn = r.into();
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
                        let r: rollupConfigHashReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `setDisputeGameFactory(address)` and selector `0x3419d2c2`.
```solidity
function setDisputeGameFactory(address _disputeGameFactory) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDisputeGameFactoryCall {
        #[allow(missing_docs)]
        pub _disputeGameFactory: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setDisputeGameFactory(address)`](setDisputeGameFactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDisputeGameFactoryReturn {}
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
            impl ::core::convert::From<setDisputeGameFactoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDisputeGameFactoryCall) -> Self {
                    (value._disputeGameFactory,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDisputeGameFactoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputeGameFactory: tuple.0,
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
            impl ::core::convert::From<setDisputeGameFactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDisputeGameFactoryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDisputeGameFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setDisputeGameFactoryReturn {
            fn _tokenize(
                &self,
            ) -> <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setDisputeGameFactoryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setDisputeGameFactoryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setDisputeGameFactory(address)";
            const SELECTOR: [u8; 4] = [52u8, 25u8, 210u8, 194u8];
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
                        &self._disputeGameFactory,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setDisputeGameFactoryReturn::_tokenize(ret)
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
    /**Function with signature `startingBlockNumber()` and selector `0x70872aa5`.
```solidity
function startingBlockNumber() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberCall;
    ///Container type for the return parameters of the [`startingBlockNumber()`](startingBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberReturn {
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        let r: startingBlockNumberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `startingTimestamp()` and selector `0x88786272`.
```solidity
function startingTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingTimestampCall;
    ///Container type for the return parameters of the [`startingTimestamp()`](startingTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingTimestampReturn {
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
            impl ::core::convert::From<startingTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingTimestampCall {
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
            impl ::core::convert::From<startingTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingTimestamp()";
            const SELECTOR: [u8; 4] = [136u8, 120u8, 98u8, 114u8];
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
                        let r: startingTimestampReturn = r.into();
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
                        let r: startingTimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `submissionInterval()` and selector `0xe1a41bcf`.
```solidity
function submissionInterval() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionIntervalCall;
    ///Container type for the return parameters of the [`submissionInterval()`](submissionIntervalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionIntervalReturn {
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
            impl ::core::convert::From<submissionIntervalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: submissionIntervalCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submissionIntervalCall {
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
            impl ::core::convert::From<submissionIntervalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submissionIntervalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submissionIntervalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submissionIntervalCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submissionInterval()";
            const SELECTOR: [u8; 4] = [225u8, 164u8, 27u8, 207u8];
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
                        let r: submissionIntervalReturn = r.into();
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
                        let r: submissionIntervalReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address _owner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub _owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value._owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _owner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self._owner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `updateSubmissionInterval(uint256)` and selector `0x336c9e81`.
```solidity
function updateSubmissionInterval(uint256 _submissionInterval) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSubmissionIntervalCall {
        #[allow(missing_docs)]
        pub _submissionInterval: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateSubmissionInterval(uint256)`](updateSubmissionIntervalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSubmissionIntervalReturn {}
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
            impl ::core::convert::From<updateSubmissionIntervalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSubmissionIntervalCall) -> Self {
                    (value._submissionInterval,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSubmissionIntervalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _submissionInterval: tuple.0,
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
            impl ::core::convert::From<updateSubmissionIntervalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSubmissionIntervalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSubmissionIntervalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateSubmissionIntervalReturn {
            fn _tokenize(
                &self,
            ) -> <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSubmissionIntervalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSubmissionIntervalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSubmissionInterval(uint256)";
            const SELECTOR: [u8; 4] = [51u8, 108u8, 158u8, 129u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._submissionInterval),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateSubmissionIntervalReturn::_tokenize(ret)
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
    /**Function with signature `updateVerifier(address)` and selector `0x97fc007c`.
```solidity
function updateVerifier(address _verifier) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateVerifierCall {
        #[allow(missing_docs)]
        pub _verifier: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateVerifier(address)`](updateVerifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateVerifierReturn {}
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
            impl ::core::convert::From<updateVerifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateVerifierCall) -> Self {
                    (value._verifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateVerifierCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _verifier: tuple.0 }
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
            impl ::core::convert::From<updateVerifierReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateVerifierReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateVerifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateVerifierReturn {
            fn _tokenize(
                &self,
            ) -> <updateVerifierCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateVerifierCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateVerifierReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateVerifier(address)";
            const SELECTOR: [u8; 4] = [151u8, 252u8, 0u8, 124u8];
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
                        &self._verifier,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateVerifierReturn::_tokenize(ret)
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
    /**Function with signature `verifier()` and selector `0x2b7ac3f3`.
```solidity
function verifier() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifierCall;
    ///Container type for the return parameters of the [`verifier()`](verifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifierReturn {
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
            impl ::core::convert::From<verifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifierCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifierCall {
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
            impl ::core::convert::From<verifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifierCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifier()";
            const SELECTOR: [u8; 4] = [43u8, 122u8, 195u8, 243u8];
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
                        let r: verifierReturn = r.into();
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
                        let r: verifierReturn = r.into();
                        r._0
                    })
            }
        }
    };
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external view returns (string memory);
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
    ///Container for all the [`OPSuccinctL2OutputOracle`](self) function calls.
    #[derive(Clone)]
    pub enum OPSuccinctL2OutputOracleCalls {
        #[allow(missing_docs)]
        GENESIS_CONFIG_NAME(GENESIS_CONFIG_NAMECall),
        #[allow(missing_docs)]
        addOpSuccinctConfig(addOpSuccinctConfigCall),
        #[allow(missing_docs)]
        addProposer(addProposerCall),
        #[allow(missing_docs)]
        aggregationVkey(aggregationVkeyCall),
        #[allow(missing_docs)]
        approvedProposers(approvedProposersCall),
        #[allow(missing_docs)]
        challenger(challengerCall),
        #[allow(missing_docs)]
        checkpointBlockHash(checkpointBlockHashCall),
        #[allow(missing_docs)]
        computeL2Timestamp(computeL2TimestampCall),
        #[allow(missing_docs)]
        deleteL2Outputs(deleteL2OutputsCall),
        #[allow(missing_docs)]
        deleteOpSuccinctConfig(deleteOpSuccinctConfigCall),
        #[allow(missing_docs)]
        dgfProposeL2Output(dgfProposeL2OutputCall),
        #[allow(missing_docs)]
        disableOptimisticMode(disableOptimisticModeCall),
        #[allow(missing_docs)]
        disputeGameFactory(disputeGameFactoryCall),
        #[allow(missing_docs)]
        enableOptimisticMode(enableOptimisticModeCall),
        #[allow(missing_docs)]
        fallbackTimeout(fallbackTimeoutCall),
        #[allow(missing_docs)]
        finalizationPeriodSeconds(finalizationPeriodSecondsCall),
        #[allow(missing_docs)]
        getL2Output(getL2OutputCall),
        #[allow(missing_docs)]
        getL2OutputAfter(getL2OutputAfterCall),
        #[allow(missing_docs)]
        getL2OutputIndexAfter(getL2OutputIndexAfterCall),
        #[allow(missing_docs)]
        historicBlockHashes(historicBlockHashesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        initializerVersion(initializerVersionCall),
        #[allow(missing_docs)]
        isValidOpSuccinctConfig(isValidOpSuccinctConfigCall),
        #[allow(missing_docs)]
        l2BlockTime(l2BlockTimeCall),
        #[allow(missing_docs)]
        lastProposalTimestamp(lastProposalTimestampCall),
        #[allow(missing_docs)]
        latestBlockNumber(latestBlockNumberCall),
        #[allow(missing_docs)]
        latestOutputIndex(latestOutputIndexCall),
        #[allow(missing_docs)]
        nextBlockNumber(nextBlockNumberCall),
        #[allow(missing_docs)]
        nextOutputIndex(nextOutputIndexCall),
        #[allow(missing_docs)]
        opSuccinctConfigs(opSuccinctConfigsCall),
        #[allow(missing_docs)]
        optimisticMode(optimisticModeCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        proposeL2Output_0(proposeL2Output_0Call),
        #[allow(missing_docs)]
        proposeL2Output_1(proposeL2Output_1Call),
        #[allow(missing_docs)]
        proposer(proposerCall),
        #[allow(missing_docs)]
        rangeVkeyCommitment(rangeVkeyCommitmentCall),
        #[allow(missing_docs)]
        removeProposer(removeProposerCall),
        #[allow(missing_docs)]
        rollupConfigHash(rollupConfigHashCall),
        #[allow(missing_docs)]
        setDisputeGameFactory(setDisputeGameFactoryCall),
        #[allow(missing_docs)]
        startingBlockNumber(startingBlockNumberCall),
        #[allow(missing_docs)]
        startingTimestamp(startingTimestampCall),
        #[allow(missing_docs)]
        submissionInterval(submissionIntervalCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateSubmissionInterval(updateSubmissionIntervalCall),
        #[allow(missing_docs)]
        updateVerifier(updateVerifierCall),
        #[allow(missing_docs)]
        verifier(verifierCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl OPSuccinctL2OutputOracleCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 214u8, 50u8, 211u8],
            [30u8, 133u8, 104u8, 0u8],
            [43u8, 49u8, 132u8, 30u8],
            [43u8, 122u8, 195u8, 243u8],
            [44u8, 105u8, 121u8, 97u8],
            [51u8, 108u8, 158u8, 129u8],
            [52u8, 25u8, 210u8, 194u8],
            [66u8, 119u8, 188u8, 6u8],
            [69u8, 153u8, 199u8, 136u8],
            [71u8, 195u8, 126u8, 156u8],
            [73u8, 24u8, 94u8, 6u8],
            [74u8, 179u8, 9u8, 172u8],
            [83u8, 77u8, 176u8, 226u8],
            [84u8, 253u8, 77u8, 80u8],
            [96u8, 202u8, 247u8, 160u8],
            [105u8, 241u8, 110u8, 236u8],
            [106u8, 86u8, 98u8, 11u8],
            [106u8, 188u8, 245u8, 99u8],
            [109u8, 154u8, 28u8, 139u8],
            [112u8, 135u8, 42u8, 165u8],
            [122u8, 65u8, 160u8, 53u8],
            [127u8, 0u8, 100u8, 32u8],
            [127u8, 1u8, 234u8, 104u8],
            [136u8, 120u8, 98u8, 114u8],
            [137u8, 196u8, 76u8, 187u8],
            [141u8, 165u8, 203u8, 91u8],
            [147u8, 153u8, 26u8, 243u8],
            [151u8, 252u8, 0u8, 124u8],
            [154u8, 170u8, 182u8, 72u8],
            [161u8, 150u8, 181u8, 37u8],
            [162u8, 90u8, 229u8, 87u8],
            [164u8, 238u8, 157u8, 123u8],
            [168u8, 228u8, 251u8, 144u8],
            [176u8, 60u8, 212u8, 24u8],
            [195u8, 46u8, 78u8, 62u8],
            [206u8, 93u8, 184u8, 214u8],
            [207u8, 142u8, 92u8, 240u8],
            [209u8, 222u8, 133u8, 108u8],
            [212u8, 101u8, 18u8, 118u8],
            [220u8, 236u8, 51u8, 72u8],
            [224u8, 194u8, 249u8, 53u8],
            [225u8, 164u8, 27u8, 207u8],
            [228u8, 11u8, 122u8, 18u8],
            [236u8, 91u8, 46u8, 58u8],
            [242u8, 180u8, 230u8, 23u8],
            [242u8, 253u8, 227u8, 139u8],
            [247u8, 47u8, 96u8, 109u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(removeProposer),
            ::core::stringify!(checkpointBlockHash),
            ::core::stringify!(rangeVkeyCommitment),
            ::core::stringify!(verifier),
            ::core::stringify!(enableOptimisticMode),
            ::core::stringify!(updateSubmissionInterval),
            ::core::stringify!(setDisputeGameFactory),
            ::core::stringify!(fallbackTimeout),
            ::core::stringify!(latestBlockNumber),
            ::core::stringify!(addOpSuccinctConfig),
            ::core::stringify!(isValidOpSuccinctConfig),
            ::core::stringify!(disableOptimisticMode),
            ::core::stringify!(challenger),
            ::core::stringify!(version),
            ::core::stringify!(optimisticMode),
            ::core::stringify!(latestOutputIndex),
            ::core::stringify!(opSuccinctConfigs),
            ::core::stringify!(nextOutputIndex),
            ::core::stringify!(rollupConfigHash),
            ::core::stringify!(startingBlockNumber),
            ::core::stringify!(dgfProposeL2Output),
            ::core::stringify!(getL2OutputIndexAfter),
            ::core::stringify!(initializerVersion),
            ::core::stringify!(startingTimestamp),
            ::core::stringify!(deleteL2Outputs),
            ::core::stringify!(owner),
            ::core::stringify!(l2BlockTime),
            ::core::stringify!(updateVerifier),
            ::core::stringify!(proposeL2Output_0),
            ::core::stringify!(historicBlockHashes),
            ::core::stringify!(getL2Output),
            ::core::stringify!(proposeL2Output_1),
            ::core::stringify!(proposer),
            ::core::stringify!(addProposer),
            ::core::stringify!(aggregationVkey),
            ::core::stringify!(finalizationPeriodSeconds),
            ::core::stringify!(getL2OutputAfter),
            ::core::stringify!(computeL2Timestamp),
            ::core::stringify!(approvedProposers),
            ::core::stringify!(nextBlockNumber),
            ::core::stringify!(lastProposalTimestamp),
            ::core::stringify!(submissionInterval),
            ::core::stringify!(initialize),
            ::core::stringify!(deleteOpSuccinctConfig),
            ::core::stringify!(disputeGameFactory),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(GENESIS_CONFIG_NAME),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <removeProposerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkpointBlockHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <verifierCall as alloy_sol_types::SolCall>::SIGNATURE,
            <enableOptimisticModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fallbackTimeoutCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestBlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disableOptimisticModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <optimisticModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestOutputIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <opSuccinctConfigsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextOutputIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <rollupConfigHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingBlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializerVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingTimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
            <deleteL2OutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateVerifierCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposeL2Output_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <historicBlockHashesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getL2OutputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposeL2Output_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <proposerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addProposerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <aggregationVkeyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getL2OutputAfterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <computeL2TimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
            <approvedProposersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextBlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastProposalTimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
            <submissionIntervalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameFactoryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OPSuccinctL2OutputOracleCalls {
        const NAME: &'static str = "OPSuccinctL2OutputOracleCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 47usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GENESIS_CONFIG_NAME(_) => {
                    <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addOpSuccinctConfig(_) => {
                    <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addProposer(_) => {
                    <addProposerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregationVkey(_) => {
                    <aggregationVkeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approvedProposers(_) => {
                    <approvedProposersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challenger(_) => {
                    <challengerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkpointBlockHash(_) => {
                    <checkpointBlockHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::computeL2Timestamp(_) => {
                    <computeL2TimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deleteL2Outputs(_) => {
                    <deleteL2OutputsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deleteOpSuccinctConfig(_) => {
                    <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::dgfProposeL2Output(_) => {
                    <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableOptimisticMode(_) => {
                    <disableOptimisticModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeGameFactory(_) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableOptimisticMode(_) => {
                    <enableOptimisticModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fallbackTimeout(_) => {
                    <fallbackTimeoutCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizationPeriodSeconds(_) => {
                    <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getL2Output(_) => {
                    <getL2OutputCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getL2OutputAfter(_) => {
                    <getL2OutputAfterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getL2OutputIndexAfter(_) => {
                    <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::historicBlockHashes(_) => {
                    <historicBlockHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializerVersion(_) => {
                    <initializerVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidOpSuccinctConfig(_) => {
                    <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2BlockTime(_) => {
                    <l2BlockTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProposalTimestamp(_) => {
                    <lastProposalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestBlockNumber(_) => {
                    <latestBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestOutputIndex(_) => {
                    <latestOutputIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextBlockNumber(_) => {
                    <nextBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextOutputIndex(_) => {
                    <nextOutputIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::opSuccinctConfigs(_) => {
                    <opSuccinctConfigsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::optimisticMode(_) => {
                    <optimisticModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proposeL2Output_0(_) => {
                    <proposeL2Output_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposeL2Output_1(_) => {
                    <proposeL2Output_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposer(_) => <proposerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::rangeVkeyCommitment(_) => {
                    <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeProposer(_) => {
                    <removeProposerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rollupConfigHash(_) => {
                    <rollupConfigHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setDisputeGameFactory(_) => {
                    <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingBlockNumber(_) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingTimestamp(_) => {
                    <startingTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submissionInterval(_) => {
                    <submissionIntervalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateSubmissionInterval(_) => {
                    <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateVerifier(_) => {
                    <updateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifier(_) => <verifierCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls>] = &[
                {
                    fn removeProposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <removeProposerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::removeProposer)
                    }
                    removeProposer
                },
                {
                    fn checkpointBlockHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <checkpointBlockHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::checkpointBlockHash)
                    }
                    checkpointBlockHash
                },
                {
                    fn rangeVkeyCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::rangeVkeyCommitment)
                    }
                    rangeVkeyCommitment
                },
                {
                    fn verifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <verifierCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPSuccinctL2OutputOracleCalls::verifier)
                    }
                    verifier
                },
                {
                    fn enableOptimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <enableOptimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::enableOptimisticMode)
                    }
                    enableOptimisticMode
                },
                {
                    fn updateSubmissionInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::updateSubmissionInterval)
                    }
                    updateSubmissionInterval
                },
                {
                    fn setDisputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::setDisputeGameFactory)
                    }
                    setDisputeGameFactory
                },
                {
                    fn fallbackTimeout(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <fallbackTimeoutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::fallbackTimeout)
                    }
                    fallbackTimeout
                },
                {
                    fn latestBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <latestBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::latestBlockNumber)
                    }
                    latestBlockNumber
                },
                {
                    fn addOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::addOpSuccinctConfig)
                    }
                    addOpSuccinctConfig
                },
                {
                    fn isValidOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::isValidOpSuccinctConfig)
                    }
                    isValidOpSuccinctConfig
                },
                {
                    fn disableOptimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <disableOptimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::disableOptimisticMode)
                    }
                    disableOptimisticMode
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPSuccinctL2OutputOracleCalls::version)
                    }
                    version
                },
                {
                    fn optimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <optimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::optimisticMode)
                    }
                    optimisticMode
                },
                {
                    fn latestOutputIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <latestOutputIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::latestOutputIndex)
                    }
                    latestOutputIndex
                },
                {
                    fn opSuccinctConfigs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <opSuccinctConfigsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::opSuccinctConfigs)
                    }
                    opSuccinctConfigs
                },
                {
                    fn nextOutputIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <nextOutputIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::nextOutputIndex)
                    }
                    nextOutputIndex
                },
                {
                    fn rollupConfigHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <rollupConfigHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::rollupConfigHash)
                    }
                    rollupConfigHash
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn dgfProposeL2Output(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::dgfProposeL2Output)
                    }
                    dgfProposeL2Output
                },
                {
                    fn getL2OutputIndexAfter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2OutputIndexAfter)
                    }
                    getL2OutputIndexAfter
                },
                {
                    fn initializerVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <initializerVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::initializerVersion)
                    }
                    initializerVersion
                },
                {
                    fn startingTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <startingTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::startingTimestamp)
                    }
                    startingTimestamp
                },
                {
                    fn deleteL2Outputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <deleteL2OutputsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::deleteL2Outputs)
                    }
                    deleteL2Outputs
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPSuccinctL2OutputOracleCalls::owner)
                    }
                    owner
                },
                {
                    fn l2BlockTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <l2BlockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::l2BlockTime)
                    }
                    l2BlockTime
                },
                {
                    fn updateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <updateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::updateVerifier)
                    }
                    updateVerifier
                },
                {
                    fn proposeL2Output_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposeL2Output_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::proposeL2Output_0)
                    }
                    proposeL2Output_0
                },
                {
                    fn historicBlockHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <historicBlockHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::historicBlockHashes)
                    }
                    historicBlockHashes
                },
                {
                    fn getL2Output(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2Output)
                    }
                    getL2Output
                },
                {
                    fn proposeL2Output_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposeL2Output_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::proposeL2Output_1)
                    }
                    proposeL2Output_1
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPSuccinctL2OutputOracleCalls::proposer)
                    }
                    proposer
                },
                {
                    fn addProposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <addProposerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::addProposer)
                    }
                    addProposer
                },
                {
                    fn aggregationVkey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <aggregationVkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::aggregationVkey)
                    }
                    aggregationVkey
                },
                {
                    fn finalizationPeriodSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OPSuccinctL2OutputOracleCalls::finalizationPeriodSeconds,
                            )
                    }
                    finalizationPeriodSeconds
                },
                {
                    fn getL2OutputAfter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputAfterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2OutputAfter)
                    }
                    getL2OutputAfter
                },
                {
                    fn computeL2Timestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <computeL2TimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::computeL2Timestamp)
                    }
                    computeL2Timestamp
                },
                {
                    fn approvedProposers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <approvedProposersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::approvedProposers)
                    }
                    approvedProposers
                },
                {
                    fn nextBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <nextBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::nextBlockNumber)
                    }
                    nextBlockNumber
                },
                {
                    fn lastProposalTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <lastProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::lastProposalTimestamp)
                    }
                    lastProposalTimestamp
                },
                {
                    fn submissionInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <submissionIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::submissionInterval)
                    }
                    submissionInterval
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::initialize)
                    }
                    initialize
                },
                {
                    fn deleteOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::deleteOpSuccinctConfig)
                    }
                    deleteOpSuccinctConfig
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::disputeGameFactory)
                    }
                    disputeGameFactory
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn GENESIS_CONFIG_NAME(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::GENESIS_CONFIG_NAME)
                    }
                    GENESIS_CONFIG_NAME
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
            ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls>] = &[
                {
                    fn removeProposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <removeProposerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::removeProposer)
                    }
                    removeProposer
                },
                {
                    fn checkpointBlockHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <checkpointBlockHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::checkpointBlockHash)
                    }
                    checkpointBlockHash
                },
                {
                    fn rangeVkeyCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::rangeVkeyCommitment)
                    }
                    rangeVkeyCommitment
                },
                {
                    fn verifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <verifierCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::verifier)
                    }
                    verifier
                },
                {
                    fn enableOptimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <enableOptimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::enableOptimisticMode)
                    }
                    enableOptimisticMode
                },
                {
                    fn updateSubmissionInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::updateSubmissionInterval)
                    }
                    updateSubmissionInterval
                },
                {
                    fn setDisputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::setDisputeGameFactory)
                    }
                    setDisputeGameFactory
                },
                {
                    fn fallbackTimeout(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <fallbackTimeoutCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::fallbackTimeout)
                    }
                    fallbackTimeout
                },
                {
                    fn latestBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <latestBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::latestBlockNumber)
                    }
                    latestBlockNumber
                },
                {
                    fn addOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::addOpSuccinctConfig)
                    }
                    addOpSuccinctConfig
                },
                {
                    fn isValidOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::isValidOpSuccinctConfig)
                    }
                    isValidOpSuccinctConfig
                },
                {
                    fn disableOptimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <disableOptimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::disableOptimisticMode)
                    }
                    disableOptimisticMode
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::version)
                    }
                    version
                },
                {
                    fn optimisticMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <optimisticModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::optimisticMode)
                    }
                    optimisticMode
                },
                {
                    fn latestOutputIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <latestOutputIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::latestOutputIndex)
                    }
                    latestOutputIndex
                },
                {
                    fn opSuccinctConfigs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <opSuccinctConfigsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::opSuccinctConfigs)
                    }
                    opSuccinctConfigs
                },
                {
                    fn nextOutputIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <nextOutputIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::nextOutputIndex)
                    }
                    nextOutputIndex
                },
                {
                    fn rollupConfigHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <rollupConfigHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::rollupConfigHash)
                    }
                    rollupConfigHash
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn dgfProposeL2Output(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::dgfProposeL2Output)
                    }
                    dgfProposeL2Output
                },
                {
                    fn getL2OutputIndexAfter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2OutputIndexAfter)
                    }
                    getL2OutputIndexAfter
                },
                {
                    fn initializerVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <initializerVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::initializerVersion)
                    }
                    initializerVersion
                },
                {
                    fn startingTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <startingTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::startingTimestamp)
                    }
                    startingTimestamp
                },
                {
                    fn deleteL2Outputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <deleteL2OutputsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::deleteL2Outputs)
                    }
                    deleteL2Outputs
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::owner)
                    }
                    owner
                },
                {
                    fn l2BlockTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <l2BlockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::l2BlockTime)
                    }
                    l2BlockTime
                },
                {
                    fn updateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <updateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::updateVerifier)
                    }
                    updateVerifier
                },
                {
                    fn proposeL2Output_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposeL2Output_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::proposeL2Output_0)
                    }
                    proposeL2Output_0
                },
                {
                    fn historicBlockHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <historicBlockHashesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::historicBlockHashes)
                    }
                    historicBlockHashes
                },
                {
                    fn getL2Output(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2Output)
                    }
                    getL2Output
                },
                {
                    fn proposeL2Output_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposeL2Output_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::proposeL2Output_1)
                    }
                    proposeL2Output_1
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::proposer)
                    }
                    proposer
                },
                {
                    fn addProposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <addProposerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::addProposer)
                    }
                    addProposer
                },
                {
                    fn aggregationVkey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <aggregationVkeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::aggregationVkey)
                    }
                    aggregationVkey
                },
                {
                    fn finalizationPeriodSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OPSuccinctL2OutputOracleCalls::finalizationPeriodSeconds,
                            )
                    }
                    finalizationPeriodSeconds
                },
                {
                    fn getL2OutputAfter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <getL2OutputAfterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::getL2OutputAfter)
                    }
                    getL2OutputAfter
                },
                {
                    fn computeL2Timestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <computeL2TimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::computeL2Timestamp)
                    }
                    computeL2Timestamp
                },
                {
                    fn approvedProposers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <approvedProposersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::approvedProposers)
                    }
                    approvedProposers
                },
                {
                    fn nextBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <nextBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::nextBlockNumber)
                    }
                    nextBlockNumber
                },
                {
                    fn lastProposalTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <lastProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::lastProposalTimestamp)
                    }
                    lastProposalTimestamp
                },
                {
                    fn submissionInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <submissionIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::submissionInterval)
                    }
                    submissionInterval
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::initialize)
                    }
                    initialize
                },
                {
                    fn deleteOpSuccinctConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::deleteOpSuccinctConfig)
                    }
                    deleteOpSuccinctConfig
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::disputeGameFactory)
                    }
                    disputeGameFactory
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn GENESIS_CONFIG_NAME(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleCalls> {
                        <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleCalls::GENESIS_CONFIG_NAME)
                    }
                    GENESIS_CONFIG_NAME
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
                Self::GENESIS_CONFIG_NAME(inner) => {
                    <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addOpSuccinctConfig(inner) => {
                    <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addProposer(inner) => {
                    <addProposerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregationVkey(inner) => {
                    <aggregationVkeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::approvedProposers(inner) => {
                    <approvedProposersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challenger(inner) => {
                    <challengerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkpointBlockHash(inner) => {
                    <checkpointBlockHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::computeL2Timestamp(inner) => {
                    <computeL2TimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deleteL2Outputs(inner) => {
                    <deleteL2OutputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deleteOpSuccinctConfig(inner) => {
                    <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::dgfProposeL2Output(inner) => {
                    <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableOptimisticMode(inner) => {
                    <disableOptimisticModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeGameFactory(inner) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableOptimisticMode(inner) => {
                    <enableOptimisticModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fallbackTimeout(inner) => {
                    <fallbackTimeoutCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizationPeriodSeconds(inner) => {
                    <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getL2Output(inner) => {
                    <getL2OutputCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getL2OutputAfter(inner) => {
                    <getL2OutputAfterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getL2OutputIndexAfter(inner) => {
                    <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::historicBlockHashes(inner) => {
                    <historicBlockHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initializerVersion(inner) => {
                    <initializerVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidOpSuccinctConfig(inner) => {
                    <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2BlockTime(inner) => {
                    <l2BlockTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastProposalTimestamp(inner) => {
                    <lastProposalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestBlockNumber(inner) => {
                    <latestBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestOutputIndex(inner) => {
                    <latestOutputIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextBlockNumber(inner) => {
                    <nextBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextOutputIndex(inner) => {
                    <nextOutputIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::opSuccinctConfigs(inner) => {
                    <opSuccinctConfigsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::optimisticMode(inner) => {
                    <optimisticModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proposeL2Output_0(inner) => {
                    <proposeL2Output_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposeL2Output_1(inner) => {
                    <proposeL2Output_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposer(inner) => {
                    <proposerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::rangeVkeyCommitment(inner) => {
                    <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeProposer(inner) => {
                    <removeProposerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rollupConfigHash(inner) => {
                    <rollupConfigHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setDisputeGameFactory(inner) => {
                    <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingBlockNumber(inner) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingTimestamp(inner) => {
                    <startingTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submissionInterval(inner) => {
                    <submissionIntervalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateSubmissionInterval(inner) => {
                    <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateVerifier(inner) => {
                    <updateVerifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifier(inner) => {
                    <verifierCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::GENESIS_CONFIG_NAME(inner) => {
                    <GENESIS_CONFIG_NAMECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addOpSuccinctConfig(inner) => {
                    <addOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addProposer(inner) => {
                    <addProposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::aggregationVkey(inner) => {
                    <aggregationVkeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approvedProposers(inner) => {
                    <approvedProposersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challenger(inner) => {
                    <challengerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpointBlockHash(inner) => {
                    <checkpointBlockHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::computeL2Timestamp(inner) => {
                    <computeL2TimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deleteL2Outputs(inner) => {
                    <deleteL2OutputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deleteOpSuccinctConfig(inner) => {
                    <deleteOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::dgfProposeL2Output(inner) => {
                    <dgfProposeL2OutputCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableOptimisticMode(inner) => {
                    <disableOptimisticModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disputeGameFactory(inner) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableOptimisticMode(inner) => {
                    <enableOptimisticModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fallbackTimeout(inner) => {
                    <fallbackTimeoutCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizationPeriodSeconds(inner) => {
                    <finalizationPeriodSecondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getL2Output(inner) => {
                    <getL2OutputCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getL2OutputAfter(inner) => {
                    <getL2OutputAfterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getL2OutputIndexAfter(inner) => {
                    <getL2OutputIndexAfterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::historicBlockHashes(inner) => {
                    <historicBlockHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializerVersion(inner) => {
                    <initializerVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidOpSuccinctConfig(inner) => {
                    <isValidOpSuccinctConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2BlockTime(inner) => {
                    <l2BlockTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastProposalTimestamp(inner) => {
                    <lastProposalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestBlockNumber(inner) => {
                    <latestBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestOutputIndex(inner) => {
                    <latestOutputIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextBlockNumber(inner) => {
                    <nextBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextOutputIndex(inner) => {
                    <nextOutputIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::opSuccinctConfigs(inner) => {
                    <opSuccinctConfigsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::optimisticMode(inner) => {
                    <optimisticModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proposeL2Output_0(inner) => {
                    <proposeL2Output_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposeL2Output_1(inner) => {
                    <proposeL2Output_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposer(inner) => {
                    <proposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rangeVkeyCommitment(inner) => {
                    <rangeVkeyCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeProposer(inner) => {
                    <removeProposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rollupConfigHash(inner) => {
                    <rollupConfigHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setDisputeGameFactory(inner) => {
                    <setDisputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::startingTimestamp(inner) => {
                    <startingTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submissionInterval(inner) => {
                    <submissionIntervalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateSubmissionInterval(inner) => {
                    <updateSubmissionIntervalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateVerifier(inner) => {
                    <updateVerifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifier(inner) => {
                    <verifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`OPSuccinctL2OutputOracle`](self) custom errors.
    #[derive(Clone)]
    pub enum OPSuccinctL2OutputOracleErrors {
        #[allow(missing_docs)]
        L1BlockHashNotAvailable(L1BlockHashNotAvailable),
        #[allow(missing_docs)]
        L1BlockHashNotCheckpointed(L1BlockHashNotCheckpointed),
    }
    impl OPSuccinctL2OutputOracleErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [34u8, 170u8, 58u8, 152u8],
            [132u8, 192u8, 104u8, 100u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(L1BlockHashNotCheckpointed),
            ::core::stringify!(L1BlockHashNotAvailable),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::SIGNATURE,
            <L1BlockHashNotAvailable as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OPSuccinctL2OutputOracleErrors {
        const NAME: &'static str = "OPSuccinctL2OutputOracleErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::L1BlockHashNotAvailable(_) => {
                    <L1BlockHashNotAvailable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::L1BlockHashNotCheckpointed(_) => {
                    <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors>] = &[
                {
                    fn L1BlockHashNotCheckpointed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors> {
                        <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OPSuccinctL2OutputOracleErrors::L1BlockHashNotCheckpointed,
                            )
                    }
                    L1BlockHashNotCheckpointed
                },
                {
                    fn L1BlockHashNotAvailable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors> {
                        <L1BlockHashNotAvailable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleErrors::L1BlockHashNotAvailable)
                    }
                    L1BlockHashNotAvailable
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
            ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors>] = &[
                {
                    fn L1BlockHashNotCheckpointed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors> {
                        <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OPSuccinctL2OutputOracleErrors::L1BlockHashNotCheckpointed,
                            )
                    }
                    L1BlockHashNotCheckpointed
                },
                {
                    fn L1BlockHashNotAvailable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPSuccinctL2OutputOracleErrors> {
                        <L1BlockHashNotAvailable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPSuccinctL2OutputOracleErrors::L1BlockHashNotAvailable)
                    }
                    L1BlockHashNotAvailable
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
                Self::L1BlockHashNotAvailable(inner) => {
                    <L1BlockHashNotAvailable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L1BlockHashNotCheckpointed(inner) => {
                    <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::L1BlockHashNotAvailable(inner) => {
                    <L1BlockHashNotAvailable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L1BlockHashNotCheckpointed(inner) => {
                    <L1BlockHashNotCheckpointed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OPSuccinctL2OutputOracle`](self) events.
    #[derive(Clone)]
    pub enum OPSuccinctL2OutputOracleEvents {
        #[allow(missing_docs)]
        DisputeGameFactorySet(DisputeGameFactorySet),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OpSuccinctConfigDeleted(OpSuccinctConfigDeleted),
        #[allow(missing_docs)]
        OpSuccinctConfigUpdated(OpSuccinctConfigUpdated),
        #[allow(missing_docs)]
        OptimisticModeToggled(OptimisticModeToggled),
        #[allow(missing_docs)]
        OutputProposed(OutputProposed),
        #[allow(missing_docs)]
        OutputsDeleted(OutputsDeleted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        ProposerUpdated(ProposerUpdated),
        #[allow(missing_docs)]
        SubmissionIntervalUpdated(SubmissionIntervalUpdated),
        #[allow(missing_docs)]
        VerifierUpdated(VerifierUpdated),
    }
    impl OPSuccinctL2OutputOracleEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8, 67u8, 84u8, 154u8, 146u8, 178u8, 65u8, 47u8, 122u8, 60u8, 175u8,
                122u8, 46u8, 86u8, 214u8, 91u8, 136u8, 33u8, 185u8, 19u8, 69u8, 54u8,
                63u8, 170u8, 95u8, 87u8, 25u8, 83u8, 132u8, 6u8, 95u8, 204u8,
            ],
            [
                31u8, 92u8, 135u8, 47u8, 30u8, 169u8, 60u8, 87u8, 228u8, 49u8, 18u8,
                234u8, 68u8, 158u8, 225u8, 158u8, 245u8, 117u8, 68u8, 136u8, 184u8,
                118u8, 39u8, 180u8, 197u8, 36u8, 86u8, 176u8, 229u8, 164u8, 16u8, 154u8,
            ],
            [
                68u8, 50u8, 176u8, 42u8, 47u8, 203u8, 237u8, 72u8, 217u8, 78u8, 141u8,
                114u8, 114u8, 62u8, 21u8, 92u8, 102u8, 144u8, 228u8, 183u8, 243u8, 154u8,
                250u8, 65u8, 162u8, 168u8, 255u8, 140u8, 10u8, 164u8, 37u8, 218u8,
            ],
            [
                78u8, 227u8, 122u8, 194u8, 199u8, 134u8, 236u8, 133u8, 232u8, 117u8,
                146u8, 211u8, 197u8, 200u8, 161u8, 221u8, 102u8, 248u8, 73u8, 109u8,
                218u8, 63u8, 18u8, 93u8, 158u8, 168u8, 202u8, 95u8, 101u8, 118u8, 41u8,
                182u8,
            ],
            [
                93u8, 243u8, 141u8, 57u8, 94u8, 220u8, 21u8, 182u8, 105u8, 214u8, 70u8,
                86u8, 155u8, 208u8, 21u8, 81u8, 51u8, 149u8, 7u8, 11u8, 91u8, 77u8,
                235u8, 138u8, 22u8, 48u8, 10u8, 187u8, 6u8, 13u8, 27u8, 90u8,
            ],
            [
                115u8, 112u8, 33u8, 128u8, 206u8, 52u8, 142u8, 7u8, 176u8, 88u8, 132u8,
                109u8, 23u8, 69u8, 201u8, 153u8, 135u8, 174u8, 108u8, 116u8, 31u8, 249u8,
                126u8, 194u8, 141u8, 69u8, 57u8, 83u8, 14u8, 241u8, 232u8, 241u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                167u8, 170u8, 242u8, 81u8, 39u8, 105u8, 218u8, 78u8, 68u8, 78u8, 61u8,
                226u8, 71u8, 190u8, 37u8, 100u8, 34u8, 92u8, 46u8, 122u8, 143u8, 116u8,
                207u8, 229u8, 40u8, 228u8, 110u8, 23u8, 210u8, 72u8, 104u8, 226u8,
            ],
            [
                193u8, 191u8, 154u8, 191u8, 181u8, 126u8, 160u8, 30u8, 217u8, 236u8,
                180u8, 244u8, 94u8, 156u8, 239u8, 167u8, 186u8, 68u8, 178u8, 230u8,
                119u8, 140u8, 60u8, 231u8, 40u8, 20u8, 9u8, 153u8, 159u8, 26u8, 241u8,
                178u8,
            ],
            [
                234u8, 1u8, 35u8, 199u8, 38u8, 166u8, 101u8, 203u8, 10u8, 181u8, 105u8,
                20u8, 68u8, 249u8, 41u8, 167u8, 5u8, 108u8, 122u8, 119u8, 9u8, 198u8,
                12u8, 5u8, 135u8, 130u8, 158u8, 128u8, 70u8, 184u8, 213u8, 20u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(VerifierUpdated),
            ::core::stringify!(OptimisticModeToggled),
            ::core::stringify!(OpSuccinctConfigDeleted),
            ::core::stringify!(OutputsDeleted),
            ::core::stringify!(ProposerUpdated),
            ::core::stringify!(DisputeGameFactorySet),
            ::core::stringify!(Initialized),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(OutputProposed),
            ::core::stringify!(SubmissionIntervalUpdated),
            ::core::stringify!(OpSuccinctConfigUpdated),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <VerifierUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <OptimisticModeToggled as alloy_sol_types::SolEvent>::SIGNATURE,
            <OpSuccinctConfigDeleted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutputsDeleted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ProposerUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <DisputeGameFactorySet as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutputProposed as alloy_sol_types::SolEvent>::SIGNATURE,
            <SubmissionIntervalUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <OpSuccinctConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for OPSuccinctL2OutputOracleEvents {
        const NAME: &'static str = "OPSuccinctL2OutputOracleEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DisputeGameFactorySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DisputeGameFactorySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DisputeGameFactorySet)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OpSuccinctConfigDeleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OpSuccinctConfigDeleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OpSuccinctConfigDeleted)
                }
                Some(
                    <OpSuccinctConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OpSuccinctConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OpSuccinctConfigUpdated)
                }
                Some(
                    <OptimisticModeToggled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OptimisticModeToggled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OptimisticModeToggled)
                }
                Some(<OutputProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OutputProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OutputProposed)
                }
                Some(<OutputsDeleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OutputsDeleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OutputsDeleted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<ProposerUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProposerUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ProposerUpdated)
                }
                Some(
                    <SubmissionIntervalUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubmissionIntervalUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SubmissionIntervalUpdated)
                }
                Some(<VerifierUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <VerifierUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::VerifierUpdated)
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
    impl alloy_sol_types::private::IntoLogData for OPSuccinctL2OutputOracleEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisputeGameFactorySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OpSuccinctConfigDeleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OpSuccinctConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OptimisticModeToggled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OutputProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OutputsDeleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubmissionIntervalUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::VerifierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisputeGameFactorySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OpSuccinctConfigDeleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OpSuccinctConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OptimisticModeToggled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OutputProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OutputsDeleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubmissionIntervalUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::VerifierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OPSuccinctL2OutputOracle`](self) contract instance.

See the [wrapper's documentation](`OPSuccinctL2OutputOracleInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OPSuccinctL2OutputOracleInstance<P, N> {
        OPSuccinctL2OutputOracleInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<OPSuccinctL2OutputOracleInstance<P, N>>,
    > {
        OPSuccinctL2OutputOracleInstance::<P, N>::deploy(__provider)
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
        OPSuccinctL2OutputOracleInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`OPSuccinctL2OutputOracle`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OPSuccinctL2OutputOracle`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OPSuccinctL2OutputOracleInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OPSuccinctL2OutputOracleInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OPSuccinctL2OutputOracleInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OPSuccinctL2OutputOracleInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OPSuccinctL2OutputOracle`](self) contract instance.

See the [wrapper's documentation](`OPSuccinctL2OutputOracleInstance`) for more details.*/
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
        ) -> alloy_contract::Result<OPSuccinctL2OutputOracleInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> OPSuccinctL2OutputOracleInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OPSuccinctL2OutputOracleInstance<P, N> {
            OPSuccinctL2OutputOracleInstance {
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
    > OPSuccinctL2OutputOracleInstance<P, N> {
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
        ///Creates a new call builder for the [`GENESIS_CONFIG_NAME`] function.
        pub fn GENESIS_CONFIG_NAME(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GENESIS_CONFIG_NAMECall, N> {
            self.call_builder(&GENESIS_CONFIG_NAMECall)
        }
        ///Creates a new call builder for the [`addOpSuccinctConfig`] function.
        pub fn addOpSuccinctConfig(
            &self,
            _configName: alloy::sol_types::private::FixedBytes<32>,
            _rollupConfigHash: alloy::sol_types::private::FixedBytes<32>,
            _aggregationVkey: alloy::sol_types::private::FixedBytes<32>,
            _rangeVkeyCommitment: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, addOpSuccinctConfigCall, N> {
            self.call_builder(
                &addOpSuccinctConfigCall {
                    _configName,
                    _rollupConfigHash,
                    _aggregationVkey,
                    _rangeVkeyCommitment,
                },
            )
        }
        ///Creates a new call builder for the [`addProposer`] function.
        pub fn addProposer(
            &self,
            _proposer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, addProposerCall, N> {
            self.call_builder(&addProposerCall { _proposer })
        }
        ///Creates a new call builder for the [`aggregationVkey`] function.
        pub fn aggregationVkey(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, aggregationVkeyCall, N> {
            self.call_builder(&aggregationVkeyCall)
        }
        ///Creates a new call builder for the [`approvedProposers`] function.
        pub fn approvedProposers(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, approvedProposersCall, N> {
            self.call_builder(&approvedProposersCall(_0))
        }
        ///Creates a new call builder for the [`challenger`] function.
        pub fn challenger(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, challengerCall, N> {
            self.call_builder(&challengerCall)
        }
        ///Creates a new call builder for the [`checkpointBlockHash`] function.
        pub fn checkpointBlockHash(
            &self,
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, checkpointBlockHashCall, N> {
            self.call_builder(
                &checkpointBlockHashCall {
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`computeL2Timestamp`] function.
        pub fn computeL2Timestamp(
            &self,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, computeL2TimestampCall, N> {
            self.call_builder(
                &computeL2TimestampCall {
                    _l2BlockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`deleteL2Outputs`] function.
        pub fn deleteL2Outputs(
            &self,
            _l2OutputIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, deleteL2OutputsCall, N> {
            self.call_builder(
                &deleteL2OutputsCall {
                    _l2OutputIndex,
                },
            )
        }
        ///Creates a new call builder for the [`deleteOpSuccinctConfig`] function.
        pub fn deleteOpSuccinctConfig(
            &self,
            _configName: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, deleteOpSuccinctConfigCall, N> {
            self.call_builder(
                &deleteOpSuccinctConfigCall {
                    _configName,
                },
            )
        }
        ///Creates a new call builder for the [`dgfProposeL2Output`] function.
        pub fn dgfProposeL2Output(
            &self,
            _configName: alloy::sol_types::private::FixedBytes<32>,
            _outputRoot: alloy::sol_types::private::FixedBytes<32>,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            _proof: alloy::sol_types::private::Bytes,
            _proverAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, dgfProposeL2OutputCall, N> {
            self.call_builder(
                &dgfProposeL2OutputCall {
                    _configName,
                    _outputRoot,
                    _l2BlockNumber,
                    _l1BlockNumber,
                    _proof,
                    _proverAddress,
                },
            )
        }
        ///Creates a new call builder for the [`disableOptimisticMode`] function.
        pub fn disableOptimisticMode(
            &self,
            _finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, disableOptimisticModeCall, N> {
            self.call_builder(
                &disableOptimisticModeCall {
                    _finalizationPeriodSeconds,
                },
            )
        }
        ///Creates a new call builder for the [`disputeGameFactory`] function.
        pub fn disputeGameFactory(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disputeGameFactoryCall, N> {
            self.call_builder(&disputeGameFactoryCall)
        }
        ///Creates a new call builder for the [`enableOptimisticMode`] function.
        pub fn enableOptimisticMode(
            &self,
            _finalizationPeriodSeconds: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, enableOptimisticModeCall, N> {
            self.call_builder(
                &enableOptimisticModeCall {
                    _finalizationPeriodSeconds,
                },
            )
        }
        ///Creates a new call builder for the [`fallbackTimeout`] function.
        pub fn fallbackTimeout(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, fallbackTimeoutCall, N> {
            self.call_builder(&fallbackTimeoutCall)
        }
        ///Creates a new call builder for the [`finalizationPeriodSeconds`] function.
        pub fn finalizationPeriodSeconds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, finalizationPeriodSecondsCall, N> {
            self.call_builder(&finalizationPeriodSecondsCall)
        }
        ///Creates a new call builder for the [`getL2Output`] function.
        pub fn getL2Output(
            &self,
            _l2OutputIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getL2OutputCall, N> {
            self.call_builder(&getL2OutputCall { _l2OutputIndex })
        }
        ///Creates a new call builder for the [`getL2OutputAfter`] function.
        pub fn getL2OutputAfter(
            &self,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getL2OutputAfterCall, N> {
            self.call_builder(
                &getL2OutputAfterCall {
                    _l2BlockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getL2OutputIndexAfter`] function.
        pub fn getL2OutputIndexAfter(
            &self,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getL2OutputIndexAfterCall, N> {
            self.call_builder(
                &getL2OutputIndexAfterCall {
                    _l2BlockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`historicBlockHashes`] function.
        pub fn historicBlockHashes(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, historicBlockHashesCall, N> {
            self.call_builder(&historicBlockHashesCall(_0))
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initParams: <InitParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { _initParams })
        }
        ///Creates a new call builder for the [`initializerVersion`] function.
        pub fn initializerVersion(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializerVersionCall, N> {
            self.call_builder(&initializerVersionCall)
        }
        ///Creates a new call builder for the [`isValidOpSuccinctConfig`] function.
        pub fn isValidOpSuccinctConfig(
            &self,
            _config: <OpSuccinctConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, isValidOpSuccinctConfigCall, N> {
            self.call_builder(
                &isValidOpSuccinctConfigCall {
                    _config,
                },
            )
        }
        ///Creates a new call builder for the [`l2BlockTime`] function.
        pub fn l2BlockTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockTimeCall, N> {
            self.call_builder(&l2BlockTimeCall)
        }
        ///Creates a new call builder for the [`lastProposalTimestamp`] function.
        pub fn lastProposalTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastProposalTimestampCall, N> {
            self.call_builder(&lastProposalTimestampCall)
        }
        ///Creates a new call builder for the [`latestBlockNumber`] function.
        pub fn latestBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestBlockNumberCall, N> {
            self.call_builder(&latestBlockNumberCall)
        }
        ///Creates a new call builder for the [`latestOutputIndex`] function.
        pub fn latestOutputIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestOutputIndexCall, N> {
            self.call_builder(&latestOutputIndexCall)
        }
        ///Creates a new call builder for the [`nextBlockNumber`] function.
        pub fn nextBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nextBlockNumberCall, N> {
            self.call_builder(&nextBlockNumberCall)
        }
        ///Creates a new call builder for the [`nextOutputIndex`] function.
        pub fn nextOutputIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nextOutputIndexCall, N> {
            self.call_builder(&nextOutputIndexCall)
        }
        ///Creates a new call builder for the [`opSuccinctConfigs`] function.
        pub fn opSuccinctConfigs(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, opSuccinctConfigsCall, N> {
            self.call_builder(&opSuccinctConfigsCall(_0))
        }
        ///Creates a new call builder for the [`optimisticMode`] function.
        pub fn optimisticMode(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, optimisticModeCall, N> {
            self.call_builder(&optimisticModeCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`proposeL2Output_0`] function.
        pub fn proposeL2Output_0(
            &self,
            _outputRoot: alloy::sol_types::private::FixedBytes<32>,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            _l1BlockHash: alloy::sol_types::private::FixedBytes<32>,
            _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposeL2Output_0Call, N> {
            self.call_builder(
                &proposeL2Output_0Call {
                    _outputRoot,
                    _l2BlockNumber,
                    _l1BlockHash,
                    _l1BlockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`proposeL2Output_1`] function.
        pub fn proposeL2Output_1(
            &self,
            _configName: alloy::sol_types::private::FixedBytes<32>,
            _outputRoot: alloy::sol_types::private::FixedBytes<32>,
            _l2BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            _l1BlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            _proof: alloy::sol_types::private::Bytes,
            _proverAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, proposeL2Output_1Call, N> {
            self.call_builder(
                &proposeL2Output_1Call {
                    _configName,
                    _outputRoot,
                    _l2BlockNumber,
                    _l1BlockNumber,
                    _proof,
                    _proverAddress,
                },
            )
        }
        ///Creates a new call builder for the [`proposer`] function.
        pub fn proposer(&self) -> alloy_contract::SolCallBuilder<&P, proposerCall, N> {
            self.call_builder(&proposerCall)
        }
        ///Creates a new call builder for the [`rangeVkeyCommitment`] function.
        pub fn rangeVkeyCommitment(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, rangeVkeyCommitmentCall, N> {
            self.call_builder(&rangeVkeyCommitmentCall)
        }
        ///Creates a new call builder for the [`removeProposer`] function.
        pub fn removeProposer(
            &self,
            _proposer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, removeProposerCall, N> {
            self.call_builder(&removeProposerCall { _proposer })
        }
        ///Creates a new call builder for the [`rollupConfigHash`] function.
        pub fn rollupConfigHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, rollupConfigHashCall, N> {
            self.call_builder(&rollupConfigHashCall)
        }
        ///Creates a new call builder for the [`setDisputeGameFactory`] function.
        pub fn setDisputeGameFactory(
            &self,
            _disputeGameFactory: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setDisputeGameFactoryCall, N> {
            self.call_builder(
                &setDisputeGameFactoryCall {
                    _disputeGameFactory,
                },
            )
        }
        ///Creates a new call builder for the [`startingBlockNumber`] function.
        pub fn startingBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingBlockNumberCall, N> {
            self.call_builder(&startingBlockNumberCall)
        }
        ///Creates a new call builder for the [`startingTimestamp`] function.
        pub fn startingTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingTimestampCall, N> {
            self.call_builder(&startingTimestampCall)
        }
        ///Creates a new call builder for the [`submissionInterval`] function.
        pub fn submissionInterval(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, submissionIntervalCall, N> {
            self.call_builder(&submissionIntervalCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            _owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { _owner })
        }
        ///Creates a new call builder for the [`updateSubmissionInterval`] function.
        pub fn updateSubmissionInterval(
            &self,
            _submissionInterval: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateSubmissionIntervalCall, N> {
            self.call_builder(
                &updateSubmissionIntervalCall {
                    _submissionInterval,
                },
            )
        }
        ///Creates a new call builder for the [`updateVerifier`] function.
        pub fn updateVerifier(
            &self,
            _verifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, updateVerifierCall, N> {
            self.call_builder(&updateVerifierCall { _verifier })
        }
        ///Creates a new call builder for the [`verifier`] function.
        pub fn verifier(&self) -> alloy_contract::SolCallBuilder<&P, verifierCall, N> {
            self.call_builder(&verifierCall)
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OPSuccinctL2OutputOracleInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DisputeGameFactorySet`] event.
        pub fn DisputeGameFactorySet_filter(
            &self,
        ) -> alloy_contract::Event<&P, DisputeGameFactorySet, N> {
            self.event_filter::<DisputeGameFactorySet>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OpSuccinctConfigDeleted`] event.
        pub fn OpSuccinctConfigDeleted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OpSuccinctConfigDeleted, N> {
            self.event_filter::<OpSuccinctConfigDeleted>()
        }
        ///Creates a new event filter for the [`OpSuccinctConfigUpdated`] event.
        pub fn OpSuccinctConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, OpSuccinctConfigUpdated, N> {
            self.event_filter::<OpSuccinctConfigUpdated>()
        }
        ///Creates a new event filter for the [`OptimisticModeToggled`] event.
        pub fn OptimisticModeToggled_filter(
            &self,
        ) -> alloy_contract::Event<&P, OptimisticModeToggled, N> {
            self.event_filter::<OptimisticModeToggled>()
        }
        ///Creates a new event filter for the [`OutputProposed`] event.
        pub fn OutputProposed_filter(
            &self,
        ) -> alloy_contract::Event<&P, OutputProposed, N> {
            self.event_filter::<OutputProposed>()
        }
        ///Creates a new event filter for the [`OutputsDeleted`] event.
        pub fn OutputsDeleted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OutputsDeleted, N> {
            self.event_filter::<OutputsDeleted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`ProposerUpdated`] event.
        pub fn ProposerUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProposerUpdated, N> {
            self.event_filter::<ProposerUpdated>()
        }
        ///Creates a new event filter for the [`SubmissionIntervalUpdated`] event.
        pub fn SubmissionIntervalUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SubmissionIntervalUpdated, N> {
            self.event_filter::<SubmissionIntervalUpdated>()
        }
        ///Creates a new event filter for the [`VerifierUpdated`] event.
        pub fn VerifierUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, VerifierUpdated, N> {
            self.event_filter::<VerifierUpdated>()
        }
    }
}
