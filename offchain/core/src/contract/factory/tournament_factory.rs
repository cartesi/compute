pub use tournament_factory::*;
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
pub mod tournament_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_singleLevelFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract SingleLevelTournamentFactory",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_topFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract TopTournamentFactory",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_middleFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract MiddleTournamentFactory",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bottomFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract BottomTournamentFactory",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("instantiateBottom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantiateBottom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedCommitmentOne",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedFinalStateOne",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedCommitmentTwo",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedFinalStateTwo",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Time.Duration"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startCycle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_level"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Tournament"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiateMiddle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantiateMiddle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedCommitmentOne",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedFinalStateOne",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedCommitmentTwo",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Tree.Node"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_contestedFinalStateTwo",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Time.Duration"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startCycle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_level"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Tournament"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiateSingleLevel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "instantiateSingleLevel",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Tournament"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiateTop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantiateTop"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Machine.Hash"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Tournament"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("rootCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("rootCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static TOURNAMENTFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x05]8\x03\x80a\x05]\x839\x81\x01`@\x81\x90Ra\x000\x91a\0jV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xA0R\x90\x82\x16`\xC0R\x81\x16`\xE0R\x16`\x80Ra\0\xC9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0gW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\0\x80W`\0\x80\xFD[\x84Qa\0\x8B\x81a\0RV[` \x86\x01Q\x90\x94Pa\0\x9C\x81a\0RV[`@\x86\x01Q\x90\x93Pa\0\xAD\x81a\0RV[``\x86\x01Q\x90\x92Pa\0\xBE\x81a\0RV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x04\\a\x01\x01`\09`\0a\x02\x96\x01R`\0a\x01\xE6\x01R`\0`\xDC\x01R`\0a\x01\xB2\x01Ra\x04\\`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\nZ\xBBK\x14a\0QW\x80c\x19\x80\x9F\xED\x14a\0\x80W\x80cP@K|\x14a\0\x93W\x80c\xB9n\xFAk\x14a\0\xA6W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\xF0V[a\0\xB9V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\x8E6`\x04a\x02\xF0V[a\x01\x8FV[a\0da\0\xA16`\x04a\x03\"V[a\x01\xE1V[a\0da\0\xB46`\x04a\x03\"V[a\x02\x91V[`@Qc\r=K\x1B`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xEAX\xD8\x90`$\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01J\x91\x90a\x03\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x90\x91P\x7F\xBA5\xF4\xF3\xA3\x8CEM+\xB9\xD8\x8FL\x8E=3\xEE\xFC\x0E\xB6\x93OF\nS\xFBu\x8F\xD7\0\x9Eh\x90` \x01`@Q\x80\x91\x03\x90\xA1\x92\x91PPV[`@Qc\r=K\x1B`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xEAX\xD8\x90`$\x01a\x01\x07V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\xD8p\x16\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B3`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x03\xCEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x83\x91\x90a\x03\xAAV[\x9A\x99PPPPPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\xD8p\x16\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B3`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x03\xCEV[`\0` \x82\x84\x03\x12\x15a\x03\x02W`\0\x80\xFD[P5\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x1FW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x03?W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015\x93P`\xA0\x89\x015a\x03m\x81a\x03\tV[\x92P`\xC0\x89\x015\x91P`\xE0\x89\x015a\x03\x84\x81a\x03\tV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xBCW`\0\x80\xFD[\x81Qa\x03\xC7\x81a\x03\x95V[\x93\x92PPPV[\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x86\x01R`\xC0\x85\x01\x91\x90\x91R\x16`\xE0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x82\x01Ra\x01 \x01\x90V\xFE\xA2dipfsX\"\x12 \xDE\xB7\xC5\xAC\x9B.P'\xC5\x8B\x0E\x9C\x1CKp\xAE\xA4\x85x?\x1A\xA5\\\xB50\xD6[\x05uA4:dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static TOURNAMENTFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\nZ\xBBK\x14a\0QW\x80c\x19\x80\x9F\xED\x14a\0\x80W\x80cP@K|\x14a\0\x93W\x80c\xB9n\xFAk\x14a\0\xA6W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\xF0V[a\0\xB9V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\x8E6`\x04a\x02\xF0V[a\x01\x8FV[a\0da\0\xA16`\x04a\x03\"V[a\x01\xE1V[a\0da\0\xB46`\x04a\x03\"V[a\x02\x91V[`@Qc\r=K\x1B`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xEAX\xD8\x90`$\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01J\x91\x90a\x03\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x90\x91P\x7F\xBA5\xF4\xF3\xA3\x8CEM+\xB9\xD8\x8FL\x8E=3\xEE\xFC\x0E\xB6\x93OF\nS\xFBu\x8F\xD7\0\x9Eh\x90` \x01`@Q\x80\x91\x03\x90\xA1\x92\x91PPV[`@Qc\r=K\x1B`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xEAX\xD8\x90`$\x01a\x01\x07V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\xD8p\x16\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B3`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x03\xCEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x83\x91\x90a\x03\xAAV[\x9A\x99PPPPPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\xD8p\x16\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B3`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x03\xCEV[`\0` \x82\x84\x03\x12\x15a\x03\x02W`\0\x80\xFD[P5\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x1FW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x03?W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015\x93P`\xA0\x89\x015a\x03m\x81a\x03\tV[\x92P`\xC0\x89\x015\x91P`\xE0\x89\x015a\x03\x84\x81a\x03\tV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xBCW`\0\x80\xFD[\x81Qa\x03\xC7\x81a\x03\x95V[\x93\x92PPPV[\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x86\x01R`\xC0\x85\x01\x91\x90\x91R\x16`\xE0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x82\x01Ra\x01 \x01\x90V\xFE\xA2dipfsX\"\x12 \xDE\xB7\xC5\xAC\x9B.P'\xC5\x8B\x0E\x9C\x1CKp\xAE\xA4\x85x?\x1A\xA5\\\xB50\xD6[\x05uA4:dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static TOURNAMENTFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TournamentFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TournamentFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TournamentFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TournamentFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TournamentFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TournamentFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TournamentFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TOURNAMENTFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TOURNAMENTFACTORY_ABI.clone(),
                TOURNAMENTFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `instantiateBottom` (0xb96efa6b) function
        pub fn instantiate_bottom(
            &self,
            initial_hash: [u8; 32],
            contested_commitment_one: [u8; 32],
            contested_final_state_one: [u8; 32],
            contested_commitment_two: [u8; 32],
            contested_final_state_two: [u8; 32],
            allowance: u64,
            start_cycle: ::ethers::core::types::U256,
            level: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [185, 110, 250, 107],
                    (
                        initial_hash,
                        contested_commitment_one,
                        contested_final_state_one,
                        contested_commitment_two,
                        contested_final_state_two,
                        allowance,
                        start_cycle,
                        level,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiateMiddle` (0x50404b7c) function
        pub fn instantiate_middle(
            &self,
            initial_hash: [u8; 32],
            contested_commitment_one: [u8; 32],
            contested_final_state_one: [u8; 32],
            contested_commitment_two: [u8; 32],
            contested_final_state_two: [u8; 32],
            allowance: u64,
            start_cycle: ::ethers::core::types::U256,
            level: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [80, 64, 75, 124],
                    (
                        initial_hash,
                        contested_commitment_one,
                        contested_final_state_one,
                        contested_commitment_two,
                        contested_final_state_two,
                        allowance,
                        start_cycle,
                        level,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiateSingleLevel` (0x19809fed) function
        pub fn instantiate_single_level(
            &self,
            initial_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([25, 128, 159, 237], initial_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiateTop` (0x0a5abb4b) function
        pub fn instantiate_top(
            &self,
            initial_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([10, 90, 187, 75], initial_hash)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `rootCreated` event
        pub fn root_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RootCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RootCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TournamentFactory<M> {
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
    #[ethevent(name = "rootCreated", abi = "rootCreated(address)")]
    pub struct RootCreatedFilter(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `instantiateBottom` function with signature `instantiateBottom(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)` and selector `0xb96efa6b`
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
        name = "instantiateBottom",
        abi = "instantiateBottom(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)"
    )]
    pub struct InstantiateBottomCall {
        pub initial_hash: [u8; 32],
        pub contested_commitment_one: [u8; 32],
        pub contested_final_state_one: [u8; 32],
        pub contested_commitment_two: [u8; 32],
        pub contested_final_state_two: [u8; 32],
        pub allowance: u64,
        pub start_cycle: ::ethers::core::types::U256,
        pub level: u64,
    }
    ///Container type for all input parameters for the `instantiateMiddle` function with signature `instantiateMiddle(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)` and selector `0x50404b7c`
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
        name = "instantiateMiddle",
        abi = "instantiateMiddle(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)"
    )]
    pub struct InstantiateMiddleCall {
        pub initial_hash: [u8; 32],
        pub contested_commitment_one: [u8; 32],
        pub contested_final_state_one: [u8; 32],
        pub contested_commitment_two: [u8; 32],
        pub contested_final_state_two: [u8; 32],
        pub allowance: u64,
        pub start_cycle: ::ethers::core::types::U256,
        pub level: u64,
    }
    ///Container type for all input parameters for the `instantiateSingleLevel` function with signature `instantiateSingleLevel(bytes32)` and selector `0x19809fed`
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
    #[ethcall(name = "instantiateSingleLevel", abi = "instantiateSingleLevel(bytes32)")]
    pub struct InstantiateSingleLevelCall {
        pub initial_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `instantiateTop` function with signature `instantiateTop(bytes32)` and selector `0x0a5abb4b`
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
    #[ethcall(name = "instantiateTop", abi = "instantiateTop(bytes32)")]
    pub struct InstantiateTopCall {
        pub initial_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TournamentFactoryCalls {
        InstantiateBottom(InstantiateBottomCall),
        InstantiateMiddle(InstantiateMiddleCall),
        InstantiateSingleLevel(InstantiateSingleLevelCall),
        InstantiateTop(InstantiateTopCall),
    }
    impl ::ethers::core::abi::AbiDecode for TournamentFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <InstantiateBottomCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InstantiateBottom(decoded));
            }
            if let Ok(decoded) = <InstantiateMiddleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InstantiateMiddle(decoded));
            }
            if let Ok(decoded) = <InstantiateSingleLevelCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InstantiateSingleLevel(decoded));
            }
            if let Ok(decoded) = <InstantiateTopCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InstantiateTop(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TournamentFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::InstantiateBottom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InstantiateMiddle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InstantiateSingleLevel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InstantiateTop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TournamentFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InstantiateBottom(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstantiateMiddle(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstantiateSingleLevel(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InstantiateTop(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InstantiateBottomCall> for TournamentFactoryCalls {
        fn from(value: InstantiateBottomCall) -> Self {
            Self::InstantiateBottom(value)
        }
    }
    impl ::core::convert::From<InstantiateMiddleCall> for TournamentFactoryCalls {
        fn from(value: InstantiateMiddleCall) -> Self {
            Self::InstantiateMiddle(value)
        }
    }
    impl ::core::convert::From<InstantiateSingleLevelCall> for TournamentFactoryCalls {
        fn from(value: InstantiateSingleLevelCall) -> Self {
            Self::InstantiateSingleLevel(value)
        }
    }
    impl ::core::convert::From<InstantiateTopCall> for TournamentFactoryCalls {
        fn from(value: InstantiateTopCall) -> Self {
            Self::InstantiateTop(value)
        }
    }
    ///Container type for all return fields from the `instantiateBottom` function with signature `instantiateBottom(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)` and selector `0xb96efa6b`
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
    pub struct InstantiateBottomReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `instantiateMiddle` function with signature `instantiateMiddle(bytes32,bytes32,bytes32,bytes32,bytes32,uint64,uint256,uint64)` and selector `0x50404b7c`
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
    pub struct InstantiateMiddleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `instantiateSingleLevel` function with signature `instantiateSingleLevel(bytes32)` and selector `0x19809fed`
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
    pub struct InstantiateSingleLevelReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `instantiateTop` function with signature `instantiateTop(bytes32)` and selector `0x0a5abb4b`
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
    pub struct InstantiateTopReturn(pub ::ethers::core::types::Address);
}
