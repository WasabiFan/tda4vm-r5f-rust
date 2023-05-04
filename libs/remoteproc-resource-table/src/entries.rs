pub mod carveout;
pub mod trace;
pub mod vdev;

#[repr(u32)]
pub enum FwResourceType {
    Carveout = 0,
    DevMem = 1,
    Trace = 2,
    VDev = 3,
}
