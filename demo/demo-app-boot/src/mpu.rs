//! A client interface for the Cortex-R5 MPU (Memory Protection Unit).

use core::fmt::Write;

use cortex_r5_pac::registers::{
    system_control::{DRACR, DRBAR, DRSR, RGNR, SCTLR},
    ReadWriteable, Readable, Writeable,
};

#[derive(Eq, PartialEq)]
pub struct MpuRegion {
    /// Region. Must be aligned to its own size.
    pub base_address: u32,
    pub size: MpuRegionSize,
    pub attributes: MpuRegionAttributes,
}

/// Size of the region (bytes).
/// Valid sizes are 2^N for 2 <= N <= 32.
#[derive(Eq, PartialEq)]
pub struct MpuRegionSize(u8);

impl MpuRegionSize {
    // TODO: provide helpers for bytes, kibibytes, mebibytes, etc.

    /// Construct a region size from the log2 of the size in bytes.
    /// The returned size represents 2^N bytes for 2 <= N <= 32, or an error otherwise.
    pub const fn from_log2_bytes(log2: u8) -> Result<Self, ()> {
        if log2 >= 2 && log2 < 32 {
            Ok(Self(log2 - 1))
        } else {
            Err(())
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct MpuRegionAttributes {
    /// If true, suppresses all execution and instruction fetch speculation into this region.
    /// If false, code is executable so long as the relevant permissions ring has read permissions.
    pub execute_never: bool,

    // Read/write permissions for supervisor and user rings
    pub access_permissions: MpuRegionAccessPermissions,

    // Cache coherence, cache read/write policies, and memory ordering
    pub coherence_configuration: MpuRegionCoherenceConfiguration,

    /// TODO: better support subregions
    pub subregion_disable_mask: u8,
}

#[derive(Eq, PartialEq)]
pub enum MpuRegionAccessPermissions {
    AllNoAccess,
    SupervisorReadWriteUserNoAccess,
    SupervisorReadWriteUserReadOnly,
    AllReadWrite,
    SupervisorReadOnlyUserNoAccess,
    AllReadOnly,
}

/// Defines the memory ordering, memory "type", caching policies, and _sometimes_ shareability of a memory region.
/// Maps to various encodings of the TEX, C, B, and S fields of the DRACR/IRACR.
/// See section "B5.3.1: C, B, and TEX[2:0] encodings" in the armv7 architecture reference manual.
#[derive(Eq, PartialEq)]
pub enum MpuRegionCoherenceConfiguration {
    /// "Strongly ordered" memory type and always Shareable.
    StronglyOrdered,
    /// "Device" memory type, Shareability configurable.
    Device(MpuRegionShareability),
    /// "Normal" memory type, Shareability configurable.
    Normal(MpuRegionCachePolicy, MpuRegionShareability),
}

/// "Shareability" of an MPU memory region.
/// In essence, whether whether this region requires memory coherence (Shareable) or not.
/// Defines either the "S" bit or the TEX encoding depending on region Memory Type.
#[derive(Eq, PartialEq)]
pub enum MpuRegionShareability {
    Shareable,
    NonShareable,
}

impl MpuRegionShareability {
    fn is_shareable(&self) -> bool {
        *self == Self::Shareable
    }
}

/// Cache policy for Normal memory regions.
/// Either uniformly applied to all shareability domains, or configurable for Inner and Outer domains separately.
/// TODO: Inner and Outer combined policies should be equivalent to the per-domain policies with matching settings. Can we hide the duplication?
#[derive(Eq, PartialEq)]
pub enum MpuRegionCachePolicy {
    AllDomains(MpuRegionShareabilityDomainCachePolicy),
    PerShareabilityDomain {
        /// Cache policy for the Inner shareability domain (typically within the core complex, perhaps hitting the L1)
        inner_domain: MpuRegionShareabilityDomainCachePolicy,
        /// Cache policy for the Outer shareability domain (typically from an observer outside the core complex, perhaps hitting the L2)
        outer_domain: MpuRegionShareabilityDomainCachePolicy,
    },
}

#[derive(Eq, PartialEq)]
pub enum MpuRegionShareabilityDomainCachePolicy {
    /// Not cacheable, thus inherently Shareable.
    NonCacheable,
    WriteBackWriteAllocate,
    WriteThroughNoWriteAllocate,
    WriteBackNoWriteAllocate,
}

impl MpuRegionCoherenceConfiguration {
    fn compute_encoding(&self) -> CoherenceEncoding {
        // "S" bit is ignored for all but "Normal" memory type. Shareability in those cases is
        // defined by the other fields.
        const S_DONT_CARE: bool = false;
        match &self {
            Self::StronglyOrdered => CoherenceEncoding {
                tex: 0b000,
                c: false,
                b: false,
                s: S_DONT_CARE,
            },
            Self::Device(MpuRegionShareability::Shareable) => CoherenceEncoding {
                tex: 0b000,
                c: false,
                b: true,
                s: S_DONT_CARE,
            },
            Self::Device(MpuRegionShareability::NonShareable) => CoherenceEncoding {
                tex: 0b010,
                c: false,
                b: true,
                s: S_DONT_CARE,
            },
            Self::Normal(MpuRegionCachePolicy::AllDomains(policy), shareability) => match policy {
                MpuRegionShareabilityDomainCachePolicy::NonCacheable => CoherenceEncoding {
                    tex: 0b001,
                    c: false,
                    b: false,
                    s: shareability.is_shareable(),
                },
                MpuRegionShareabilityDomainCachePolicy::WriteBackWriteAllocate => {
                    CoherenceEncoding {
                        tex: 0b001,
                        c: true,
                        b: true,
                        s: shareability.is_shareable(),
                    }
                }
                MpuRegionShareabilityDomainCachePolicy::WriteThroughNoWriteAllocate => {
                    CoherenceEncoding {
                        tex: 0b000,
                        c: true,
                        b: false,
                        s: shareability.is_shareable(),
                    }
                }
                MpuRegionShareabilityDomainCachePolicy::WriteBackNoWriteAllocate => {
                    CoherenceEncoding {
                        tex: 0b000,
                        c: true,
                        b: true,
                        s: shareability.is_shareable(),
                    }
                }
            },
            Self::Normal(
                MpuRegionCachePolicy::PerShareabilityDomain {
                    inner_domain,
                    outer_domain,
                },
                shareability,
            ) => {
                fn policy_to_two_bit_encoding(
                    policy: &MpuRegionShareabilityDomainCachePolicy,
                ) -> u8 {
                    match &policy {
                        MpuRegionShareabilityDomainCachePolicy::NonCacheable => 0b00,
                        MpuRegionShareabilityDomainCachePolicy::WriteBackWriteAllocate => 0b01,
                        MpuRegionShareabilityDomainCachePolicy::WriteThroughNoWriteAllocate => 0b10,
                        MpuRegionShareabilityDomainCachePolicy::WriteBackNoWriteAllocate => 0b11,
                    }
                }

                let outer_encoding = policy_to_two_bit_encoding(outer_domain);
                let inner_encoding = policy_to_two_bit_encoding(inner_domain);

                CoherenceEncoding {
                    tex: 0b100 | (outer_encoding & 0b011),
                    c: (inner_encoding & 0b10) != 0,
                    b: (inner_encoding & 0b01) != 0,
                    s: shareability.is_shareable(),
                }
            }
        }
    }
}

pub struct Mpu;

impl Mpu {
    const MAX_SUPPORTED_REGIONS: u8 = 16;

    pub fn init(regions: &[MpuRegion], background_region_enable: bool) {
        assert!(regions.len() <= Self::MAX_SUPPORTED_REGIONS as usize);

        // TODO: we can probably support only "disable, configure from scratch, enable" APIs rather than handling progressive region changes
        // TODO: handle MPU being already enabled

        // TODO: dsb/isb somewhere? Check on barrier requirements.

        SCTLR.modify(SCTLR::BR.val(0));

        for (i, region) in regions.iter().enumerate() {
            Self::configure_region(i.try_into().unwrap(), Some(region));
        }

        for i in regions.len().try_into().unwrap()..Self::MAX_SUPPORTED_REGIONS {
            Self::configure_region(i, None);
        }

        if background_region_enable {
            SCTLR.modify(SCTLR::BR.val(1));
        }

        // TODO: Handle cache disable/flush/enable
        SCTLR.modify(SCTLR::M.val(1));
    }

    fn configure_region(region_idx: u8, config: Option<&MpuRegion>) {
        assert!(region_idx < Self::MAX_SUPPORTED_REGIONS);

        // TODO: make sure MPU is disabled when reconfiguring regions
        // TODO: when changing coherence/cacheability parameters, we need to configure the region into an intermediate state
        // TODO: critical section

        let config = if let Some(c) = config {
            c
        } else {
            RGNR.modify(RGNR::Region.val(region_idx.into()));
            DRSR.modify(DRSR::En.val(0));
            return;
        };

        // Region base address must be aligned to its size
        let size_alignment_mask: u32 = (1u64 << (config.size.0 + 1))
            .wrapping_sub(1)
            .try_into()
            .unwrap();
        assert!((config.base_address & size_alignment_mask) == 0);

        let ap = match config.attributes.access_permissions {
            MpuRegionAccessPermissions::AllNoAccess => DRACR::AP::AllNoAccess,
            MpuRegionAccessPermissions::SupervisorReadWriteUserNoAccess => {
                DRACR::AP::SupervisorReadWriteUserNoAccess
            }
            MpuRegionAccessPermissions::SupervisorReadWriteUserReadOnly => {
                DRACR::AP::SupervisorReadWriteUserReadOnly
            }
            MpuRegionAccessPermissions::AllReadWrite => DRACR::AP::AllReadWrite,
            MpuRegionAccessPermissions::SupervisorReadOnlyUserNoAccess => {
                DRACR::AP::SupervisorReadOnlyUserNoAccess
            }
            MpuRegionAccessPermissions::AllReadOnly => DRACR::AP::AllReadOnly,
        };

        RGNR.modify(RGNR::Region.val(region_idx.into()));
        DRBAR.write(DRBAR::BaseAddress.val(config.base_address));
        DRSR.modify(
            DRSR::En.val(1)
                + DRSR::RSize.val(config.size.0.into())
                + DRSR::SD.val(config.attributes.subregion_disable_mask.into()),
        );
        let CoherenceEncoding { tex, c, b, s } =
            config.attributes.coherence_configuration.compute_encoding();
        DRACR.modify(
            DRACR::XN.val(config.attributes.execute_never.into())
                + ap
                + DRACR::TEX.val(tex.into())
                + DRACR::C.val(c.into())
                + DRACR::B.val(b.into())
                + DRACR::S.val(s.into()),
        );
    }
}

struct CoherenceEncoding {
    tex: u8,
    c: bool,
    b: bool,
    s: bool,
}
