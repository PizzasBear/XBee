use crate::stream::{self, Endianness, InnerData, ReadStream, WriteStream};
use crate::{ClusterId, Endpoint, IeeeAddress, NetworkAddress, ProfileId};
use bitflags::bitflags;

pub trait FrameData: InnerData {
    const API_TYPE: u8;
}

#[derive(Debug, Clone, PartialEq, Eq, InnerData)]
#[repr(C)]
pub struct LocalAtCommandRequest<T> {
    pub id: u8,
    pub at_command: [u8; 2],
    pub parameter: T,
}

impl<T: InnerData> FrameData for LocalAtCommandRequest<T> {
    const API_TYPE: u8 = 0x08;
}

#[derive(Debug, Clone, PartialEq, Eq, InnerData)]
#[repr(C)]
pub struct LocalAtCommandResponse<T> {
    pub id: u8,
    pub at_command: [u8; 2],
    pub command_status: u8,
    pub command_data: T,
}

impl<T: InnerData> FrameData for LocalAtCommandResponse<T> {
    const API_TYPE: u8 = 0x88;
}

#[derive(Debug, Clone, PartialEq, Eq, InnerData)]
#[repr(C)]
pub struct QueueLocalAtCommandRequest<T> {
    pub id: u8,
    pub at_command: [u8; 2],
    pub parameter: T,
}

impl<T: InnerData> FrameData for QueueLocalAtCommandRequest<T> {
    const API_TYPE: u8 = 0x09;
}

bitflags! {
    #[derive(Default, InnerData)]
    #[repr(transparent)]
    pub struct TransmitOpts: u8 {
        const DISABLE_ACK = 0x01;
        const INDIRECT_TRANSMISSION = 0x04;
        const MULTICAST = 0x08;
        const SECURE_SESSION_ENCRYPTION = 0x10;
        const ENABLE_APS_ENCRYPTION = 0x20;
        const USE_EXTENDED_TIMEOUT = 0x40;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, InnerData)]
#[repr(C)]
pub struct TransmitRequest<T> {
    pub id: u8,
    pub ieee_address: IeeeAddress,
    pub network_address: NetworkAddress,
    pub broadcast_radius: u8,
    pub transmit_opts: TransmitOpts,
    pub payload_data: T,
}

impl<T: InnerData> FrameData for TransmitRequest<T> {
    const API_TYPE: u8 = 0x10;
}

#[derive(Debug, Clone, PartialEq, Eq, InnerData)]
#[repr(C)]
pub struct ExplicitAddressingCommandRequest {
    pub id: u8,
    pub dest64: IeeeAddress,
    pub dest16: NetworkAddress,
    pub source_ep: Endpoint,
    pub dest_ep: Endpoint,
    pub cluster_id: ClusterId,
    pub profile_id: ProfileId,
    pub broadcast_radius: u8,
    pub transmit_opts: TransmitOpts,
    pub command_data: stream::OverwriteLittleEndian<ClusterData>,
}

impl<T: InnerData> FrameData for ExplicitAddressingCommandRequest<T> {
    const API_TYPE: u8 = 0x11;
}

pub mod extended_transmit_status {
    use super::*;
    stream::inner_data_enum! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum DeliveryStatus: u8 {
            Success = 0x00,
            MacAckFailure = 0x01,
            CcaLbtFailure = 0x02,
            // IndirectMessageUnrequestedNoSpectrumAvailable = 0x03,
            InvalidDestinationEndpoint = 0x15,
            NetworkAckFailure = 0x21,
            NotJoinedToNetwork = 0x22,
            SelfAddressed = 0x23,
            AddressNotFound = 0x24,
            RouteNotFound = 0x25,
            BroadcastSourceFailedToHearRelay = 0x26,
            InvalidBindingTableIndex = 0x2B,
            /// Lack of free buffers, timers, etc.
            ResourceError = 0x2C,
            AttemptedBroadcastWithApsTransmission = 0x2D,
            AttemptedUnicastWithApsTransmissionAndDisabledEncryption = 0x2E,
            // InternalResourceError = 0x31,
            ResourceError2 = 0x32,
            NoSecureSessionConnection = 0x34,
            EncryptionFailure = 0x35,
            DataPayloadTooLarge = 0x74,
            IndirectMessageUnrequested = 0x75,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum DiscoveryStatus: u8 {
            NoDiscoveryOverhead = 0x00,
            ZigbeeAddressDiscovery = 0x01,
            RouteDiscovery = 0x02,
            ZigbeeAddressAndRouteDiscovery = 0x03,
            ZigbeeEndDeviceExtendedTimeout = 0x40,
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, InnerData)]
    #[repr(C)]
    pub struct ExtendedTransmitStatus {
        pub id: u8,
        pub network_address: NetworkAddress,
        pub transit_retry_count: u8,
        pub delivery_status: DeliveryStatus,
        pub discovery_status: DiscoveryStatus,
    }

    impl FrameData for ExtendedTransmitStatus {
        const API_TYPE: u8 = 0x8B;
    }
}
pub use extended_transmit_status::ExtendedTransmitStatus;

pub mod explicit_rx_indicator {
    use super::*;

    bitflags! {
        #[derive(Default, InnerData)]
        pub struct ReceiveOpts: u8 {
            const ACKNOWLEDGED = 1 << 0;
            const SENT_AS_BROADCAST = 1 << 1;
            const SENT_SECURELY = 1 << 4;
            const ENCRYPTED_WITH_ZIGBEE_APS = 1 << 5;
            const SENT_FROM_END_DEVICE = 1 << 6;
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, InnerData)]
    #[repr(C)]
    pub struct ExplicitRxIndicator {
        pub source_ieee_address: IeeeAddress,
        pub source_network_address: NetworkAddress,
        pub source_endpoint: Endpoint,
        pub destination_endpoint: Endpoint,
        pub cluster_id: ClusterId,
        pub profile_id: ProfileId,
        pub receive_opts: ReceiveOpts,
        pub received_data: ClusterData,
    }

    impl FrameData for ExplicitRxIndicator {
        const API_TYPE: u8 = 0x91;
    }
}
pub use explicit_rx_indicator::ExplicitRxIndicator;

mod many_to_one_route_request_indicator {
    use super::*;

    bitflags! {
        #[derive(Default, InnerData)]
        pub struct ReceiveOpts: u8 {}
    }

    #[derive(Debug, Clone, PartialEq, Eq, InnerData)]
    #[repr(C)]
    pub struct ManyToOneRouteRequestIndicator {
        pub source_ieee_address: IeeeAddress,
        pub source_network_address: NetworkAddress,
        pub recieve_opts: ReceiveOpts,
    }

    impl FrameData for ManyToOneRouteRequestIndicator {
        const API_TYPE: u8 = 0xA3;
    }
}
pub use many_to_one_route_request_indicator::ManyToOneRouteRequestIndicator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame<T: FrameData>(pub T);

impl<T: FrameData> Frame<T> {
    pub fn write<F: FnMut(&[u8])>(&self, write_f: &mut F) {
        struct ApiWriteStream<F>(F);

        impl<F: FnMut(&[u8])> WriteStream for ApiWriteStream<F> {
            fn endianness(&self) -> Endianness {
                Endianness::BigEndian
            }
            fn write(&mut self, bytes: &[u8]) {
                (self.0)(bytes);
            }
        }

        let stream = &mut ApiWriteStream(write_f);
        stream.write(&[0x7e]);
        (self.0.byte_size() + 1).write(stream);

        let mut checksum = 0xffu8;
        let cs_stream = &mut ApiWriteStream(|bytes: &[u8]| {
            for &byte in bytes {
                checksum = checksum.wrapping_sub(byte);
            }
            stream.write(bytes);
        });

        cs_stream.write(&[T::API_TYPE]);
        self.0.write(cs_stream);
        stream.write(&[checksum]);
    }
}
