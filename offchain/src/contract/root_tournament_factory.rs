pub use root_tournament_factory::*;
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
pub mod root_tournament_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_innerFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IInnerTournamentFactory",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stateInterface"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IUArchState"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stepInterface"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IUArchStep"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("instantiateSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantiateSingle"),
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
                                        ::std::borrow::ToOwned::to_owned("contract RootTournament"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantiateTopOfMultiple"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "instantiateTopOfMultiple",
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
                                        ::std::borrow::ToOwned::to_owned("contract RootTournament"),
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
    pub static ROOTTOURNAMENTFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct RootTournamentFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RootTournamentFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RootTournamentFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RootTournamentFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RootTournamentFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RootTournamentFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RootTournamentFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROOTTOURNAMENTFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `instantiateSingle` (0xd514cfc7) function
        pub fn instantiate_single(
            &self,
            initial_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([213, 20, 207, 199], initial_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantiateTopOfMultiple` (0x1e78da56) function
        pub fn instantiate_top_of_multiple(
            &self,
            initial_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([30, 120, 218, 86], initial_hash)
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
    for RootTournamentFactory<M> {
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
    ///Container type for all input parameters for the `instantiateSingle` function with signature `instantiateSingle(bytes32)` and selector `0xd514cfc7`
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
    #[ethcall(name = "instantiateSingle", abi = "instantiateSingle(bytes32)")]
    pub struct InstantiateSingleCall {
        pub initial_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `instantiateTopOfMultiple` function with signature `instantiateTopOfMultiple(bytes32)` and selector `0x1e78da56`
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
        name = "instantiateTopOfMultiple",
        abi = "instantiateTopOfMultiple(bytes32)"
    )]
    pub struct InstantiateTopOfMultipleCall {
        pub initial_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RootTournamentFactoryCalls {
        InstantiateSingle(InstantiateSingleCall),
        InstantiateTopOfMultiple(InstantiateTopOfMultipleCall),
    }
    impl ::ethers::core::abi::AbiDecode for RootTournamentFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <InstantiateSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InstantiateSingle(decoded));
            }
            if let Ok(decoded)
                = <InstantiateTopOfMultipleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InstantiateTopOfMultiple(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RootTournamentFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::InstantiateSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InstantiateTopOfMultiple(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RootTournamentFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InstantiateSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstantiateTopOfMultiple(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InstantiateSingleCall> for RootTournamentFactoryCalls {
        fn from(value: InstantiateSingleCall) -> Self {
            Self::InstantiateSingle(value)
        }
    }
    impl ::core::convert::From<InstantiateTopOfMultipleCall>
    for RootTournamentFactoryCalls {
        fn from(value: InstantiateTopOfMultipleCall) -> Self {
            Self::InstantiateTopOfMultiple(value)
        }
    }
    ///Container type for all return fields from the `instantiateSingle` function with signature `instantiateSingle(bytes32)` and selector `0xd514cfc7`
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
    pub struct InstantiateSingleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `instantiateTopOfMultiple` function with signature `instantiateTopOfMultiple(bytes32)` and selector `0x1e78da56`
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
    pub struct InstantiateTopOfMultipleReturn(pub ::ethers::core::types::Address);
}
