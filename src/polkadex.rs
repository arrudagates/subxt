use sp_runtime::{
    generic::Header,
    impl_opaque_keys,
    traits::{
        BlakeTwo256,
        IdentifyAccount,
        Verify,
    },
    MultiSignature,
    OpaqueExtrinsic,
};
use sp_std::prelude::*;
use crate::{extrinsic::{
    DefaultExtra,
}, frame::{
    balances::{
        AccountData,
        Balances,
        BalancesEventTypeRegistry,
    },
    contracts::{
        Contracts,
        ContractsEventTypeRegistry,
    },
    session::{
        Session,
        SessionEventTypeRegistry,
    },
    staking::{
        Staking,
        StakingEventTypeRegistry,
    },
    sudo::{
        Sudo,
        SudoEventTypeRegistry,
    },
    system::{
        System,
        SystemEventTypeRegistry,
    },
}, EventTypeRegistry, Runtime, register_default_type_sizes, app, Babe, Grandpa, ImOnline, AuthorityDiscovery};


/// Parachain marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Thea;

impl sp_runtime::BoundToRuntimeAppPublic for Thea {
    type Public = app::thea::Public;
}

impl_opaque_keys! {
    /// Substrate base runtime keys
    pub struct PolkadexSessionKeys {
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// BABE session key
        pub babe: Babe,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// Parachain validation session key
        pub thea: Thea,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}
/// Polkadex Runtime
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolkadexRuntime;


impl Staking for PolkadexRuntime {}

impl Runtime for PolkadexRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        event_type_registry.with_staking();
        event_type_registry.with_contracts();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
        register_polkadex_type_sizes(event_type_registry);
    }
}

impl System for PolkadexRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for PolkadexRuntime {
    type Balance = u128;
}

impl Session for PolkadexRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = PolkadexSessionKeys;
}

impl Contracts for PolkadexRuntime {}

impl Sudo for PolkadexRuntime {}

/// Register default common runtime type sizes
pub fn register_polkadex_type_sizes<T: Runtime>(
    event_type_registry: &mut EventTypeRegistry<T>,
) {
    event_type_registry.register_type_size::<T::AccountId>("T::AccountId");
    event_type_registry.register_type_size::<T::BlockNumber>("T::BlockNumber");
}