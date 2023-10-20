pub use leaf_tournament::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod leaf_tournament {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("advanceMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("advanceMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.Id"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leftNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rightNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newLeftNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newRightNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("canWinMatchByTimeout"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "canWinMatchByTimeout",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.Id"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCommitment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCommitment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_commitmentRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Clock.State"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchIdHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Match.IdHash"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.State"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMatchCycle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMatchCycle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchIdHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Match.IdHash"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("joinTournament"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("joinTournament"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_finalState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leftNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rightNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maximumEnforceableDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maximumEnforceableDelay",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Time.Instant"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sealLeafMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sealLeafMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.Id"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leftLeaf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rightLeaf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_agreeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_agreeHashProof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tournamentLevelConstants"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tournamentLevelConstants",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_level"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_log2step"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_height"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("winLeafMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("winLeafMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.Id"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leftNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rightNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proofs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("winMatchByTimeout"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("winMatchByTimeout"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_matchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Match.Id"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leftNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rightNode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("matchAdvanced"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("matchAdvanced"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("parent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("left"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("matchCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("one"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("two"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("leftOfTwo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LEAFTOURNAMENT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct LeafTournament<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LeafTournament<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LeafTournament<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LeafTournament<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LeafTournament<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LeafTournament))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LeafTournament<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LEAFTOURNAMENT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `advanceMatch` (0xfcc85391) function
        pub fn advance_match(
            &self,
            match_id: Id,
            left_node: [u8; 32],
            right_node: [u8; 32],
            new_left_node: [u8; 32],
            new_right_node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [252, 200, 83, 145],
                    (match_id, left_node, right_node, new_left_node, new_right_node),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canWinMatchByTimeout` (0x6a1a140d) function
        pub fn can_win_match_by_timeout(
            &self,
            match_id: Id,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([106, 26, 20, 13], (match_id,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCommitment` (0x7795820c) function
        pub fn get_commitment(
            &self,
            commitment_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (ClockState, [u8; 32])> {
            self.0
                .method_hash([119, 149, 130, 12], commitment_root)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMatch` (0xfcc6077d) function
        pub fn get_match(
            &self,
            match_id_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, MatchState> {
            self.0
                .method_hash([252, 198, 7, 125], match_id_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMatchCycle` (0x8acc802d) function
        pub fn get_match_cycle(
            &self,
            match_id_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([138, 204, 128, 45], match_id_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `joinTournament` (0x1d5bf796) function
        pub fn join_tournament(
            &self,
            final_state: [u8; 32],
            proof: ::std::vec::Vec<[u8; 32]>,
            left_node: [u8; 32],
            right_node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [29, 91, 247, 150],
                    (final_state, proof, left_node, right_node),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumEnforceableDelay` (0x60f302dc) function
        pub fn maximum_enforceable_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([96, 243, 2, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sealLeafMatch` (0x5017746a) function
        pub fn seal_leaf_match(
            &self,
            match_id: Id,
            left_leaf: [u8; 32],
            right_leaf: [u8; 32],
            agree_hash: [u8; 32],
            agree_hash_proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [80, 23, 116, 106],
                    (match_id, left_leaf, right_leaf, agree_hash, agree_hash_proof),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tournamentLevelConstants` (0xa1af906b) function
        pub fn tournament_level_constants(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64, u64)> {
            self.0
                .method_hash([161, 175, 144, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `winLeafMatch` (0x6041ddd5) function
        pub fn win_leaf_match(
            &self,
            match_id: Id,
            left_node: [u8; 32],
            right_node: [u8; 32],
            proofs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 65, 221, 213],
                    (match_id, left_node, right_node, proofs),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `winMatchByTimeout` (0xff78e0ee) function
        pub fn win_match_by_timeout(
            &self,
            match_id: Id,
            left_node: [u8; 32],
            right_node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 120, 224, 238], (match_id, left_node, right_node))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `matchAdvanced` event
        pub fn match_advanced_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MatchAdvancedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `matchCreated` event
        pub fn match_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MatchCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LeafTournamentEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LeafTournament<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "matchAdvanced", abi = "matchAdvanced(bytes32,bytes32,bytes32)")]
    pub struct MatchAdvancedFilter {
        #[ethevent(indexed)]
        pub p0: [u8; 32],
        pub parent: [u8; 32],
        pub left: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "matchCreated", abi = "matchCreated(bytes32,bytes32,bytes32)")]
    pub struct MatchCreatedFilter {
        #[ethevent(indexed)]
        pub one: [u8; 32],
        #[ethevent(indexed)]
        pub two: [u8; 32],
        pub left_of_two: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LeafTournamentEvents {
        MatchAdvancedFilter(MatchAdvancedFilter),
        MatchCreatedFilter(MatchCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LeafTournamentEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MatchAdvancedFilter::decode_log(log) {
                return Ok(LeafTournamentEvents::MatchAdvancedFilter(decoded));
            }
            if let Ok(decoded) = MatchCreatedFilter::decode_log(log) {
                return Ok(LeafTournamentEvents::MatchCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LeafTournamentEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MatchAdvancedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MatchCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MatchAdvancedFilter> for LeafTournamentEvents {
        fn from(value: MatchAdvancedFilter) -> Self {
            Self::MatchAdvancedFilter(value)
        }
    }
    impl ::core::convert::From<MatchCreatedFilter> for LeafTournamentEvents {
        fn from(value: MatchCreatedFilter) -> Self {
            Self::MatchCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `advanceMatch` function with signature `advanceMatch((bytes32,bytes32),bytes32,bytes32,bytes32,bytes32)` and selector `0xfcc85391`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "advanceMatch",
        abi = "advanceMatch((bytes32,bytes32),bytes32,bytes32,bytes32,bytes32)"
    )]
    pub struct AdvanceMatchCall {
        pub match_id: Id,
        pub left_node: [u8; 32],
        pub right_node: [u8; 32],
        pub new_left_node: [u8; 32],
        pub new_right_node: [u8; 32],
    }
    ///Container type for all input parameters for the `canWinMatchByTimeout` function with signature `canWinMatchByTimeout((bytes32,bytes32))` and selector `0x6a1a140d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "canWinMatchByTimeout",
        abi = "canWinMatchByTimeout((bytes32,bytes32))"
    )]
    pub struct CanWinMatchByTimeoutCall {
        pub match_id: Id,
    }
    ///Container type for all input parameters for the `getCommitment` function with signature `getCommitment(bytes32)` and selector `0x7795820c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCommitment", abi = "getCommitment(bytes32)")]
    pub struct GetCommitmentCall {
        pub commitment_root: [u8; 32],
    }
    ///Container type for all input parameters for the `getMatch` function with signature `getMatch(bytes32)` and selector `0xfcc6077d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMatch", abi = "getMatch(bytes32)")]
    pub struct GetMatchCall {
        pub match_id_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getMatchCycle` function with signature `getMatchCycle(bytes32)` and selector `0x8acc802d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMatchCycle", abi = "getMatchCycle(bytes32)")]
    pub struct GetMatchCycleCall {
        pub match_id_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `joinTournament` function with signature `joinTournament(bytes32,bytes32[],bytes32,bytes32)` and selector `0x1d5bf796`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "joinTournament",
        abi = "joinTournament(bytes32,bytes32[],bytes32,bytes32)"
    )]
    pub struct JoinTournamentCall {
        pub final_state: [u8; 32],
        pub proof: ::std::vec::Vec<[u8; 32]>,
        pub left_node: [u8; 32],
        pub right_node: [u8; 32],
    }
    ///Container type for all input parameters for the `maximumEnforceableDelay` function with signature `maximumEnforceableDelay()` and selector `0x60f302dc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maximumEnforceableDelay", abi = "maximumEnforceableDelay()")]
    pub struct MaximumEnforceableDelayCall;
    ///Container type for all input parameters for the `sealLeafMatch` function with signature `sealLeafMatch((bytes32,bytes32),bytes32,bytes32,bytes32,bytes32[])` and selector `0x5017746a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sealLeafMatch",
        abi = "sealLeafMatch((bytes32,bytes32),bytes32,bytes32,bytes32,bytes32[])"
    )]
    pub struct SealLeafMatchCall {
        pub match_id: Id,
        pub left_leaf: [u8; 32],
        pub right_leaf: [u8; 32],
        pub agree_hash: [u8; 32],
        pub agree_hash_proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `tournamentLevelConstants` function with signature `tournamentLevelConstants()` and selector `0xa1af906b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tournamentLevelConstants", abi = "tournamentLevelConstants()")]
    pub struct TournamentLevelConstantsCall;
    ///Container type for all input parameters for the `winLeafMatch` function with signature `winLeafMatch((bytes32,bytes32),bytes32,bytes32,bytes)` and selector `0x6041ddd5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "winLeafMatch",
        abi = "winLeafMatch((bytes32,bytes32),bytes32,bytes32,bytes)"
    )]
    pub struct WinLeafMatchCall {
        pub match_id: Id,
        pub left_node: [u8; 32],
        pub right_node: [u8; 32],
        pub proofs: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `winMatchByTimeout` function with signature `winMatchByTimeout((bytes32,bytes32),bytes32,bytes32)` and selector `0xff78e0ee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "winMatchByTimeout",
        abi = "winMatchByTimeout((bytes32,bytes32),bytes32,bytes32)"
    )]
    pub struct WinMatchByTimeoutCall {
        pub match_id: Id,
        pub left_node: [u8; 32],
        pub right_node: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LeafTournamentCalls {
        AdvanceMatch(AdvanceMatchCall),
        CanWinMatchByTimeout(CanWinMatchByTimeoutCall),
        GetCommitment(GetCommitmentCall),
        GetMatch(GetMatchCall),
        GetMatchCycle(GetMatchCycleCall),
        JoinTournament(JoinTournamentCall),
        MaximumEnforceableDelay(MaximumEnforceableDelayCall),
        SealLeafMatch(SealLeafMatchCall),
        TournamentLevelConstants(TournamentLevelConstantsCall),
        WinLeafMatch(WinLeafMatchCall),
        WinMatchByTimeout(WinMatchByTimeoutCall),
    }
    impl ::ethers::core::abi::AbiDecode for LeafTournamentCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdvanceMatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdvanceMatch(decoded));
            }
            if let Ok(decoded) = <CanWinMatchByTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanWinMatchByTimeout(decoded));
            }
            if let Ok(decoded) = <GetCommitmentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCommitment(decoded));
            }
            if let Ok(decoded) = <GetMatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMatch(decoded));
            }
            if let Ok(decoded) = <GetMatchCycleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMatchCycle(decoded));
            }
            if let Ok(decoded) = <JoinTournamentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::JoinTournament(decoded));
            }
            if let Ok(decoded) = <MaximumEnforceableDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaximumEnforceableDelay(decoded));
            }
            if let Ok(decoded) = <SealLeafMatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SealLeafMatch(decoded));
            }
            if let Ok(decoded) = <TournamentLevelConstantsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TournamentLevelConstants(decoded));
            }
            if let Ok(decoded) = <WinLeafMatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WinLeafMatch(decoded));
            }
            if let Ok(decoded) = <WinMatchByTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WinMatchByTimeout(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LeafTournamentCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdvanceMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanWinMatchByTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCommitment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMatchCycle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::JoinTournament(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumEnforceableDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SealLeafMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TournamentLevelConstants(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WinLeafMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WinMatchByTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LeafTournamentCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdvanceMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanWinMatchByTimeout(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMatchCycle(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinTournament(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximumEnforceableDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SealLeafMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::TournamentLevelConstants(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WinLeafMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::WinMatchByTimeout(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdvanceMatchCall> for LeafTournamentCalls {
        fn from(value: AdvanceMatchCall) -> Self {
            Self::AdvanceMatch(value)
        }
    }
    impl ::core::convert::From<CanWinMatchByTimeoutCall> for LeafTournamentCalls {
        fn from(value: CanWinMatchByTimeoutCall) -> Self {
            Self::CanWinMatchByTimeout(value)
        }
    }
    impl ::core::convert::From<GetCommitmentCall> for LeafTournamentCalls {
        fn from(value: GetCommitmentCall) -> Self {
            Self::GetCommitment(value)
        }
    }
    impl ::core::convert::From<GetMatchCall> for LeafTournamentCalls {
        fn from(value: GetMatchCall) -> Self {
            Self::GetMatch(value)
        }
    }
    impl ::core::convert::From<GetMatchCycleCall> for LeafTournamentCalls {
        fn from(value: GetMatchCycleCall) -> Self {
            Self::GetMatchCycle(value)
        }
    }
    impl ::core::convert::From<JoinTournamentCall> for LeafTournamentCalls {
        fn from(value: JoinTournamentCall) -> Self {
            Self::JoinTournament(value)
        }
    }
    impl ::core::convert::From<MaximumEnforceableDelayCall> for LeafTournamentCalls {
        fn from(value: MaximumEnforceableDelayCall) -> Self {
            Self::MaximumEnforceableDelay(value)
        }
    }
    impl ::core::convert::From<SealLeafMatchCall> for LeafTournamentCalls {
        fn from(value: SealLeafMatchCall) -> Self {
            Self::SealLeafMatch(value)
        }
    }
    impl ::core::convert::From<TournamentLevelConstantsCall> for LeafTournamentCalls {
        fn from(value: TournamentLevelConstantsCall) -> Self {
            Self::TournamentLevelConstants(value)
        }
    }
    impl ::core::convert::From<WinLeafMatchCall> for LeafTournamentCalls {
        fn from(value: WinLeafMatchCall) -> Self {
            Self::WinLeafMatch(value)
        }
    }
    impl ::core::convert::From<WinMatchByTimeoutCall> for LeafTournamentCalls {
        fn from(value: WinMatchByTimeoutCall) -> Self {
            Self::WinMatchByTimeout(value)
        }
    }
    ///Container type for all return fields from the `canWinMatchByTimeout` function with signature `canWinMatchByTimeout((bytes32,bytes32))` and selector `0x6a1a140d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CanWinMatchByTimeoutReturn(pub bool);
    ///Container type for all return fields from the `getCommitment` function with signature `getCommitment(bytes32)` and selector `0x7795820c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetCommitmentReturn(pub ClockState, pub [u8; 32]);
    ///Container type for all return fields from the `getMatch` function with signature `getMatch(bytes32)` and selector `0xfcc6077d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMatchReturn(pub MatchState);
    ///Container type for all return fields from the `getMatchCycle` function with signature `getMatchCycle(bytes32)` and selector `0x8acc802d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMatchCycleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maximumEnforceableDelay` function with signature `maximumEnforceableDelay()` and selector `0x60f302dc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaximumEnforceableDelayReturn(pub u64);
    ///Container type for all return fields from the `tournamentLevelConstants` function with signature `tournamentLevelConstants()` and selector `0xa1af906b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TournamentLevelConstantsReturn {
        pub level: u64,
        pub log_2step: u64,
        pub height: u64,
    }
    ///`ClockState(uint64,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClockState {
        pub allowance: u64,
        pub start_instant: u64,
    }
    ///`Id(bytes32,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Id {
        pub commitment_one: [u8; 32],
        pub commitment_two: [u8; 32],
    }
    ///`MatchState(bytes32,bytes32,bytes32,uint256,uint64,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MatchState {
        pub other_parent: [u8; 32],
        pub left_node: [u8; 32],
        pub right_node: [u8; 32],
        pub running_leaf_position: ::ethers::core::types::U256,
        pub current_height: u64,
        pub level: u64,
    }
}
