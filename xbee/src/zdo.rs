pub use crate::stream::{Endianness, InnerData, ReadStream, WriteStream};
use crate::{stream, Cluster, ClusterId, Endpoint, IeeeAddress, NetworkAddress, ProfileId};
use bitflags::bitflags;
use heapless::Vec;

// ZDO Command                              | Cluster ID
// -----------------------------------------+-----------
// Network (16-bit) Address Request         | 0x0000
// Network (16-bit) Address Response        | 0x8000
// IEEE (64-bit) Address Request            | 0x0001
// IEEE (64-bit) Address Response           | 0x8001
// Node Descriptor Request                  | 0x0002
// Node Descriptor Response                 | 0x8002
// Simple Descriptor Request                | 0x0004
// Simple Descriptor Response               | 0x8004
// Active Endpoints Request                 | 0x0005
// Active Endpoints Response                | 0x8005
// Match Descriptor Request                 | 0x0006
// Match Descriptor Response                | 0x8006
// Complex Descriptor Request               | 0x0010
// Complex Descriptor Response              | 0x8010
// User Descriptor Request                  | 0x0011
// User Descriptor Response                 | 0x8011
// User Descriptor Set                      | 0x0014
// Management Network Discovery Request     | 0x0030
// Management Network Discovery Response    | 0x8030
// Management LQI (Neighbor Table) Request  | 0x0031
// Management LQI (Neighbor Table) Response | 0x8031
// Management Rtg (Routing Table) Request   | 0x0032
// Management Rtg (Routing Table) Response  | 0x8032
// Management Leave Request                 | 0x0034
// Management Leave Response                | 0x8034
// Management Permit Join Request           | 0x0036
// Management Permit Join Response          | 0x8036
// Management Network Update Request        | 0x0038
// Management Network Update Notify         | 0x8038

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct StatusCode(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct NetworkAddressRequest {
    pub ieee_address: IeeeAddress,
    pub extended_response: bool,
    pub start_index: u8,
}

impl Cluster for NetworkAddressRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct NetworkAddressResponseSingle {
    pub status: StatusCode,
    pub ieee_address: IeeeAddress,
    pub network_address: NetworkAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkAddressResponseExtended {
    pub status: StatusCode,
    pub ieee_address: IeeeAddress,
    pub network_address: NetworkAddress,
    pub num_addresses: u8,
    pub start_index: u8,
    pub addresses: Vec<NetworkAddress, 256>,
}

impl InnerData for NetworkAddressResponseExtended {
    const MIN_SIZE: usize = {
        StatusCode::MIN_SIZE
            + IeeeAddress::MIN_SIZE
            + NetworkAddress::MIN_SIZE
            + u8::MIN_SIZE
            + u8::MIN_SIZE
    };
    const MAX_SIZE: Option<usize> = None;

    fn byte_size(&self) -> usize {
        self.status.byte_size()
            + self.ieee_address.byte_size()
            + self.network_address.byte_size()
            + self.num_addresses.byte_size()
            + self.start_index.byte_size()
            + self.addresses.len() * NetworkAddress::MIN_SIZE
    }
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
        assert!(Self::MIN_SIZE <= max_size, "`max_size` too small");
        let status = StatusCode::read(stream, StatusCode::MIN_SIZE);
        let ieee_address = IeeeAddress::read(stream, IeeeAddress::MIN_SIZE);
        let network_address = NetworkAddress::read(stream, NetworkAddress::MIN_SIZE);
        let num_addresses = u8::read(stream, 1);
        let start_index = u8::read(stream, 1);

        assert!(
            Self::MIN_SIZE + num_addresses as usize * NetworkAddress::MIN_SIZE <= max_size,
            "`max_size` too small for the read address count"
        );
        let addresses = (0..num_addresses)
            .map(|_| NetworkAddress::read(stream, NetworkAddress::MIN_SIZE))
            .collect();
        Self {
            status,
            ieee_address,
            network_address,
            num_addresses,
            start_index,
            addresses,
        }
    }
    fn write<T: WriteStream>(&self, stream: &mut T) {
        self.status.write(stream);
        self.ieee_address.write(stream);
        self.network_address.write(stream);
        self.num_addresses.write(stream);
        self.start_index.write(stream);
        for x in &self.addresses {
            x.write(stream);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkAddressResponse {
    Single(NetworkAddressResponseSingle),
    Extended(NetworkAddressResponseExtended),
}

impl InnerData for NetworkAddressResponse {
    const MIN_SIZE: usize = NetworkAddressResponseSingle::MIN_SIZE;
    const MAX_SIZE: Option<usize> = None;

    fn byte_size(&self) -> usize {
        match self {
            Self::Single(resp) => resp.byte_size(),
            Self::Extended(resp) => resp.byte_size(),
        }
    }
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
        if max_size < NetworkAddressResponseExtended::MIN_SIZE {
            Self::Single(NetworkAddressResponseSingle::read(stream, max_size))
        } else {
            Self::Extended(NetworkAddressResponseExtended::read(stream, max_size))
        }
    }
    fn write<T: WriteStream>(&self, stream: &mut T) {
        match self {
            Self::Single(resp) => resp.write(stream),
            Self::Extended(resp) => resp.write(stream),
        }
    }
}

impl Cluster for NetworkAddressResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct IeeeAddressRequest {
    pub network_address: NetworkAddress,
    pub extended_response: bool,
    pub start_index: u8,
}

impl Cluster for IeeeAddressRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0001);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct IeeeAddressResponseSingle {
    pub status: StatusCode,
    pub ieee_address: IeeeAddress,
    pub network_address: NetworkAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IeeeAddressResponseExtended {
    pub status: StatusCode,
    pub ieee_address: IeeeAddress,
    pub network_address: NetworkAddress,
    pub num_addresses: u8,
    pub start_index: u8,
    pub addresses: Vec<NetworkAddress, 256>,
}

impl InnerData for IeeeAddressResponseExtended {
    const MIN_SIZE: usize = {
        StatusCode::MIN_SIZE
            + IeeeAddress::MIN_SIZE
            + NetworkAddress::MIN_SIZE
            + u8::MIN_SIZE
            + u8::MIN_SIZE
    };
    const MAX_SIZE: Option<usize> = None;

    fn byte_size(&self) -> usize {
        self.status.byte_size()
            + self.ieee_address.byte_size()
            + self.network_address.byte_size()
            + self.num_addresses.byte_size()
            + self.start_index.byte_size()
            + self.addresses.len() * NetworkAddress::MIN_SIZE
    }
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
        assert!(Self::MIN_SIZE <= max_size, "`max_size` too small");
        let status = StatusCode::read(stream, StatusCode::MIN_SIZE);
        let ieee_address = IeeeAddress::read(stream, IeeeAddress::MIN_SIZE);
        let network_address = NetworkAddress::read(stream, NetworkAddress::MIN_SIZE);
        let num_addresses = u8::read(stream, 1);
        let start_index = u8::read(stream, 1);

        assert!(
            Self::MIN_SIZE + num_addresses as usize * NetworkAddress::MIN_SIZE <= max_size,
            "`max_size` too small for the read address count"
        );
        let addresses = (0..num_addresses)
            .map(|_| NetworkAddress::read(stream, NetworkAddress::MIN_SIZE))
            .collect();
        Self {
            status,
            ieee_address,
            network_address,
            num_addresses,
            start_index,
            addresses,
        }
    }
    fn write<T: WriteStream>(&self, stream: &mut T) {
        self.status.write(stream);
        self.ieee_address.write(stream);
        self.network_address.write(stream);
        self.num_addresses.write(stream);
        self.start_index.write(stream);
        for x in &self.addresses {
            x.write(stream);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IeeeAddressResponse {
    Single(IeeeAddressResponseSingle),
    Extended(IeeeAddressResponseExtended),
}

impl InnerData for IeeeAddressResponse {
    const MIN_SIZE: usize = IeeeAddressResponseSingle::MIN_SIZE;
    const MAX_SIZE: Option<usize> = None;

    fn byte_size(&self) -> usize {
        match self {
            Self::Single(resp) => resp.byte_size(),
            Self::Extended(resp) => resp.byte_size(),
        }
    }
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
        if max_size < IeeeAddressResponseExtended::MIN_SIZE {
            Self::Single(IeeeAddressResponseSingle::read(stream, max_size))
        } else {
            Self::Extended(IeeeAddressResponseExtended::read(stream, max_size))
        }
    }
    fn write<T: WriteStream>(&self, stream: &mut T) {
        match self {
            Self::Single(resp) => resp.write(stream),
            Self::Extended(resp) => resp.write(stream),
        }
    }
}

impl Cluster for IeeeAddressResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8001);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct NodeDescriptorRequest {
    pub network_address: NetworkAddress,
}

impl Cluster for NodeDescriptorRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0002);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LogicalType {
    Coordinator = 0b00,
    Router = 0b01,
    EndDevice = 0b10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrequencyBand {
    F868Mhz = 0,
    F900Mhz = 2,
    F2_4Ghz = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(C)]
pub struct NodeDescriptorOpts(u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservedError;

impl NodeDescriptorOpts {
    pub fn new(
        logical_type: LogicalType,
        complex_desc_supported: bool,
        user_desc_supported: bool,
        freq_band: FrequencyBand,
    ) -> Self {
        let mut slf = Self(0, 0);
        slf.set_logical_type(logical_type);
        slf.set_complex_desc_supported(complex_desc_supported);
        slf.set_user_desc_supported(user_desc_supported);
        slf.set_freq_band(freq_band);
        slf
    }

    pub fn set_logical_type(&mut self, logical_type: LogicalType) {
        self.0 &= 0b1111_1000;
        self.0 |= match logical_type {
            LogicalType::Coordinator => 0b000,
            LogicalType::Router => 0b001,
            LogicalType::EndDevice => 0b010,
        };
    }

    pub fn logical_type(&self) -> Result<LogicalType, ReservedError> {
        match self.0 & 3 {
            0b000 => Ok(LogicalType::Coordinator),
            0b001 => Ok(LogicalType::Router),
            0b010 => Ok(LogicalType::EndDevice),
            _ => Err(ReservedError),
        }
    }

    pub fn set_complex_desc_supported(&mut self, complex_desc_supported: bool) {
        self.0 &= 0b1111_0111;
        self.0 |= (complex_desc_supported as u8) << 3;
    }

    pub fn complex_desc_supported(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub fn set_user_desc_supported(&mut self, user_desc_supported: bool) {
        self.0 &= 0b1110_1111;
        self.0 |= (user_desc_supported as u8) << 4;
    }

    pub fn user_desc_supported(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    pub fn set_freq_band(&mut self, freq_band: FrequencyBand) {
        self.0 &= 0b0000_0111;
        self.0 |= match freq_band {
            FrequencyBand::F868Mhz => 0b0000_1000,
            FrequencyBand::F900Mhz => 0b0010_0000,
            FrequencyBand::F2_4Ghz => 0b0100_0000,
        };
    }

    pub fn freq_band(&self) -> Result<FrequencyBand, ReservedError> {
        match self.0 & 0b1111_1000 {
            0b0000_1000 => Ok(FrequencyBand::F868Mhz),
            0b0010_0000 => Ok(FrequencyBand::F900Mhz),
            0b0100_0000 => Ok(FrequencyBand::F2_4Ghz),
            _ => Err(ReservedError),
        }
    }
}

bitflags! {
    #[derive(Default, InnerData)]
    pub struct MacCapabilityFlags: u8 {
        const ALTERNATE_PAN_COORDINATOR = 1 << 0;
        const DEVICE_TYPE = 1 << 1;
        const POWER_SOURCE = 1 << 2;
        const RECIEVER_ON_WHEN_IDLE = 1 << 3;
        const SECURITY_CAPABILITY = 1 << 6;
        const ALLOCATE_ADDRESS = 1 << 7;
    }

    #[derive(Default, InnerData)]
    pub struct DescriptorCapabilityField: u8 {
        const EXTENDED_ACTIVE_ENDPOINT_LIST_AVAILABLE = 1 << 0;
        const EXTENDED_SIMPLE_DESCRIPTOR_LIST_AVAILABLE = 1 << 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct NodeDescriptor {
    pub opts: NodeDescriptorOpts,
    pub mac_capability_flags: MacCapabilityFlags,
    pub manugacturer_code: u16,
    pub max_buffer_size: u8,
    pub max_incoming_transfer_size: u16,
    pub server_mask: u16,
    pub max_outgoing_transfer_size: u16,
    pub desc_capability_field: DescriptorCapabilityField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct NodeDescriptorResponse {
    pub status: StatusCode,
    pub network_address: NetworkAddress,
    pub node_descriptor: NodeDescriptor,
}

impl Cluster for NodeDescriptorResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8002);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
pub struct SimpleDescriptorRequest {
    pub network_address: NetworkAddress,
    pub endpoint: Endpoint,
}

impl Cluster for SimpleDescriptorRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0004);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct SimpleDescriptor {
    pub endpoint: Endpoint,
    pub app_profile_id: ProfileId,
    pub app_device_id: u16,
    pub app_device_version: u8,
    pub intput_cluster_list: stream::SizeVec<stream::U8Len, ClusterId, 127>,
    pub output_cluster_list: stream::SizeVec<stream::U8Len, ClusterId, 127>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct SimpleDescriptorResponse {
    pub status: StatusCode,
    pub network_address: NetworkAddress,
    pub len: u8,
    pub simple_descriptor: SimpleDescriptor,
}

impl Cluster for SimpleDescriptorResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8004);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct ActiveEndpointsRequest {
    pub network_address: NetworkAddress,
}

impl Cluster for ActiveEndpointsRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0005);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct ActiveEndpointsResponse {
    pub status: StatusCode,
    pub network_address: NetworkAddress,
    pub active_endpoint_list: stream::SizeVec<stream::U8Len, Endpoint, 255>,
}

impl Cluster for ActiveEndpointsResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8005);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct MatchDescriptorRequest {
    pub network_address: NetworkAddress,
    pub profile_id: ProfileId,
    pub intput_cluster_list: stream::SizeVec<stream::U8Len, ClusterId, 255>,
    pub output_cluster_list: stream::SizeVec<stream::U8Len, ClusterId, 255>,
}

impl Cluster for MatchDescriptorRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0006);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct MatchDescriptorResponse {
    pub status: StatusCode,
    pub network_address: NetworkAddress,
    pub match_list: stream::SizeVec<stream::U8Len, Endpoint, 255>,
}

impl Cluster for MatchDescriptorResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8006);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct ComplexDescriptorRequest {
    pub network_address: NetworkAddress,
}

impl Cluster for ComplexDescriptorRequest {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x0010);
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
// pub struct ComplexDescriptor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct ComplexDescriptorResponse {
    pub status: StatusCode,
    pub network_address: NetworkAddress,
    pub complex_descriptor: stream::SizeVec<stream::U8Len, u8, 255>,
}

impl Cluster for ComplexDescriptorResponse {
    const PROFILE_ID: ProfileId = ProfileId::ZIGBEE_DEVICE;
    const CLUSTER_ID: ClusterId = ClusterId(0x8010);
}
