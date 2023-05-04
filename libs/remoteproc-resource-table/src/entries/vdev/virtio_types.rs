//! https://github.com/torvalds/linux/blob/master/include/uapi/linux/virtio_ids.h
//! https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-1930005

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VirtIODeviceId {
    /// virtio net
    VIRTIO_ID_NET = 1,
    /// virtio block
    VIRTIO_ID_BLOCK = 2,
    /// virtio console
    VIRTIO_ID_CONSOLE = 3,
    /// virtio rng
    VIRTIO_ID_RNG = 4,
    /// virtio balloon
    VIRTIO_ID_BALLOON = 5,
    /// virtio ioMemory
    VIRTIO_ID_IOMEM = 6,
    /// virtio remote processor messaging
    VIRTIO_ID_RPMSG = 7,
    /// virtio scsi
    VIRTIO_ID_SCSI = 8,
    /// 9p virtio console
    VIRTIO_ID_9P = 9,
    /// virtio WLAN MAC
    VIRTIO_ID_MAC80211_WLAN = 10,
    /// virtio remoteproc serial link
    VIRTIO_ID_RPROC_SERIAL = 11,
    /// Virtio caif
    VIRTIO_ID_CAIF = 12,
    /// virtio memory balloon
    VIRTIO_ID_MEMORY_BALLOON = 13,
    /// virtio GPU
    VIRTIO_ID_GPU = 16,
    /// virtio clock/timer
    VIRTIO_ID_CLOCK = 17,
    /// virtio input
    VIRTIO_ID_INPUT = 18,
    /// virtio vsock transport
    VIRTIO_ID_VSOCK = 19,
    /// virtio crypto
    VIRTIO_ID_CRYPTO = 20,
    /// virtio signal distribution device
    VIRTIO_ID_SIGNAL_DIST = 21,
    /// virtio pstore device
    VIRTIO_ID_PSTORE = 22,
    /// virtio IOMMU
    VIRTIO_ID_IOMMU = 23,
    /// virtio mem
    VIRTIO_ID_MEM = 24,
    /// virtio sound
    VIRTIO_ID_SOUND = 25,
    /// virtio filesystem
    VIRTIO_ID_FS = 26,
    /// virtio pmem
    VIRTIO_ID_PMEM = 27,
    /// virtio rpmb
    VIRTIO_ID_RPMB = 28,
    /// virtio mac80211-hwsim
    VIRTIO_ID_MAC80211_HWSIM = 29,
    /// virtio video encoder
    VIRTIO_ID_VIDEO_ENCODER = 30,
    /// virtio video decoder
    VIRTIO_ID_VIDEO_DECODER = 31,
    /// virtio SCMI
    VIRTIO_ID_SCMI = 32,
    /// virtio nitro secure modul
    VIRTIO_ID_NITRO_SEC_MOD = 33,
    /// virtio i2c adapter
    VIRTIO_ID_I2C_ADAPTER = 34,
    /// virtio watchdog
    VIRTIO_ID_WATCHDOG = 35,
    /// virtio can
    VIRTIO_ID_CAN = 36,
    /// virtio dmabuf
    VIRTIO_ID_DMABUF = 37,
    /// virtio parameter server
    VIRTIO_ID_PARAM_SERV = 38,
    /// virtio audio policy
    VIRTIO_ID_AUDIO_POLICY = 39,
    /// virtio bluetooth
    VIRTIO_ID_BT = 40,
    /// virtio gpio
    VIRTIO_ID_GPIO = 41,
}
