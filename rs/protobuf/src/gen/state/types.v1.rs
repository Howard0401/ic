#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrincipalId {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanisterId {
    #[prost(message, optional, tag = "1")]
    pub principal_id: ::core::option::Option<PrincipalId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetId {
    #[prost(message, optional, tag = "1")]
    pub principal_id: ::core::option::Option<PrincipalId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserId {
    #[prost(message, optional, tag = "1")]
    pub principal_id: ::core::option::Option<PrincipalId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeId {
    #[prost(message, optional, tag = "1")]
    pub principal_id: ::core::option::Option<PrincipalId>,
}
/// A non-interactive distributed key generation (NI-DKG) ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NiDkgId {
    #[prost(uint64, tag = "1")]
    pub start_block_height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub dealer_subnet: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "NiDkgTag", tag = "4")]
    pub dkg_tag: i32,
    #[prost(message, optional, tag = "5")]
    pub remote_target_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NominalCycles {
    #[prost(uint64, tag = "1")]
    pub high: u64,
    #[prost(uint64, tag = "2")]
    pub low: u64,
}
/// A non-interactive distributed key generation (NI-DKG) tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NiDkgTag {
    Unspecified = 0,
    LowThreshold = 1,
    HighThreshold = 2,
}
impl NiDkgTag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NiDkgTag::Unspecified => "NI_DKG_TAG_UNSPECIFIED",
            NiDkgTag::LowThreshold => "NI_DKG_TAG_LOW_THRESHOLD",
            NiDkgTag::HighThreshold => "NI_DKG_TAG_HIGH_THRESHOLD",
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanisterUpgradeOptions {
    #[prost(bool, optional, tag = "1")]
    pub skip_pre_upgrade: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanisterInstallModeV2 {
    #[prost(
        oneof = "canister_install_mode_v2::CanisterInstallModeV2",
        tags = "1, 2"
    )]
    pub canister_install_mode_v2:
        ::core::option::Option<canister_install_mode_v2::CanisterInstallModeV2>,
}
/// Nested message and enum types in `CanisterInstallModeV2`.
pub mod canister_install_mode_v2 {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CanisterInstallModeV2 {
        #[prost(enumeration = "super::CanisterInstallMode", tag = "1")]
        Mode(i32),
        #[prost(message, tag = "2")]
        Mode2(super::CanisterUpgradeOptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CanisterInstallMode {
    Unspecified = 0,
    Install = 1,
    Reinstall = 2,
    Upgrade = 3,
}
impl CanisterInstallMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CanisterInstallMode::Unspecified => "CANISTER_INSTALL_MODE_UNSPECIFIED",
            CanisterInstallMode::Install => "CANISTER_INSTALL_MODE_INSTALL",
            CanisterInstallMode::Reinstall => "CANISTER_INSTALL_MODE_REINSTALL",
            CanisterInstallMode::Upgrade => "CANISTER_INSTALL_MODE_UPGRADE",
        }
    }
}
