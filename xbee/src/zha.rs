pub use crate::stream::{Endianness, InnerData, ReadStream, WriteStream};
use crate::{
    inner_data_enum, stream, Cluster, ClusterId, Endpoint, IeeeAddress, NetworkAddress, ProfileId,
};
use bitflags::bitflags;
use heapless::String;

// Explicit RX Indicator (API 1)
//
// 7E 00 17 91 84 B4 DB FF FE EF C8 75 00 00 01 E6 00 00 01 04 01 10 54 00 00 00 E1
//
// Start delimiter: 7E
// Length: 00 17 (23)
// Frame type: 91 (Explicit RX Indicator)
// 64-bit source address: 84 B4 DB FF FE EF C8 75
// 16-bit source address: 00 00
// Source endpoint: 01
// Destination endpoint: E6
// Cluster ID: 00 00
// Profile ID: 01 04
// Receive options: 01
// RF data (HEX): 10 54 00 00 00

inner_data_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PowerSource: u8 {
        Unknown = 0x00,
        MainsSinglePhase = 0x01,
        Mains3Phase = 0x02,
        Battery = 0x03,
        DcSource = 0x04,
        EmergencyMainsConstantlyPowered = 0x05,
        EmergencyMainsAndTransferSwitch = 0x06,

        BatteryBackupedUnknown = 0x80,
        BatteryBackupedMainsSinglePhase = 0x81,
        BatteryBackupedMains3Phase = 0x82,
        BatteryBackupedBattery = 0x83,
        BatteryBackupedDcSource = 0x84,
        BatteryBackupedEmergencyMainsConstantlyPowered = 0x85,
        BatteryBackupedEmergencyMainsAndTransferSwitch = 0x86,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PhysicalEnvironment: u8 {
        UnspecifiedEnvironment = 0x00,
        DeprecatedMirrorCapacityAvailable = 0x01,
        Bar = 0x02,
        Courtyard = 0x03,
        Bathroom = 0x04,
        Bedroom = 0x05,
        BilliardRoom = 0x06,
        UtilityRoom = 0x07,
        Cellar = 0x08,
        StorageCloset = 0x09,
        Theater = 0x0a,
        Office1 = 0x0b,
        Deck = 0x0c,
        Den = 0x0d,
        DiningRoom = 0x0e,
        ElectricalRoom = 0x0f,
        Elevator = 0x10,
        Entry = 0x11,
        FamilyRoom = 0x12,
        MainFloor = 0x13,
        Upstairs = 0x14,
        Downstairs = 0x15,
        Basement = 0x16,
        Gallery = 0x17,
        GameRoom = 0x18,
        Garage = 0x19,
        Gym = 0x1a,
        Hallway = 0x1b,
        House = 0x1c,
        Kitchen = 0x1d,
        LaundryRoom = 0x1e,
        Library = 0x1f,
        MasterBedroom = 0x20,
        /// small room for coats and boots
        MudRoom = 0x21,
        Nursery = 0x22,
        Pantry = 0x23,
        Office2 = 0x24,
        Outside = 0x25,
        Pool = 0x26,
        Porch = 0x27,
        SewingRoom = 0x28,
        SittingRoom = 0x29,
        Stairway = 0x2a,
        Yard = 0x2b,
        Attic = 0x2c,
        HotTub = 0x2d,
        LivingRoom1 = 0x2e,
        Sauna = 0x2f,
        ShopOrWorkshop = 0x30,
        GuestBedroom = 0x31,
        GuestBath = 0x32,
        /// 1/2 bath
        PowderRoom = 0x33,
        BackYard = 0x34,
        FrontYard = 0x35,
        Patio = 0x36,
        Driveway = 0x37,
        SunRoom = 0x38,
        LivingRoom2 = 0x39,
        Spa = 0x3a,
        Whirlpool = 0x3b,
        Shed = 0x3c,
        EquipmentStorage = 0x3d,
        HobbyOrCraftRoom = 0x3e,
        Fountain = 0x3f,
        Pond = 0x40,
        ReceptionRoom = 0x41,
        BreakfastRoom = 0x42,
        Nook = 0x43,
        Garden = 0x44,
        Balcony = 0x45,
        PanicRoom = 0x46,
        Terrace = 0x47,
        Roof = 0x48,
        Toilet = 0x49,
        ToiletMain = 0x4a,
        OutsideToilet = 0x4b,
        Showerroom = 0x4c,
        Study = 0x4d,
        FrontGarden = 0x4e,
        BackGarden = 0x4f,
        Kettle = 0x50,
        Television = 0x51,
        Stove = 0x52,
        Microwave = 0x53,
        Toaster = 0x54,
        Vacuum = 0x55,
        Appliance = 0x56,
        FrontDoor = 0x57,
        BackDoor = 0x58,
        FridgeDoor = 0x59,
        MedicationCabinetDoor = 0x60,
        WardrobeDoor = 0x61,
        FrontCupboardDoor = 0x62,
        OtherDoor = 0x63,
        WaitingRoom = 0x64,
        TriageRoom = 0x65,
        DoctorsOffice = 0x66,
        PatientsPrivateRoom = 0x67,
        ConsultationRoom = 0x68,
        NurseStation = 0x69,
        Ward = 0x6a,
        Corridor = 0x6b,
        OperatingTheatre = 0x6c,
        DentalSurgeryRoom = 0x6d,
        MedicalImagingRoom = 0x6e,
        DecontaminationRoom = 0x6f,
        Atrium = 0x70,
        Mirror = 0x71,
        UnknownEnvironment = 0xff,
    }
}

impl PowerSource {
    #[inline]
    pub fn battery_backup(&self) -> bool {
        *self as u8 & 0x80 != 0
    }
}

impl Default for PhysicalEnvironment {
    fn default() -> Self {
        Self::UnknownEnvironment
    }
}

bitflags! {
    #[derive(Default, InnerData)]
    pub struct AlarmMask: u8 {
        const GENERAL_HARDWARE_FAULT = 0b01;
        const GENERAL_SOFTWARE_FAULT = 0b10;
    }

    #[derive(Default, InnerData)]
    pub struct DisableLocalConfig: u8 {
        const RESET_TO_FACTORY_DEFAULTS_DISABLED = 0x01;
        const DEVICE_CONF_DISABLED = 0x10;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, InnerData)]
pub struct BasicCluster {
    pub zcl_version: u8,
    pub app_version: u8,
    pub stack_version: u8,
    pub hw_version: u8,
    pub manugacturer_name: String<32>,
    pub model_identifier: String<32>,
    pub date_code: String<16>,
    pub power_source: u8,
    pub generic_device_class: u8,
    pub generic_device_type: u8,
    pub product_code: String<64>,
    pub product_url: String<64>,
    pub manufacturer_version_details: String<64>,
    pub serial_number: String<64>,
    pub product_label: String<64>,
    pub location_description: String<16>,
    pub physical_environment: PhysicalEnvironment,
    pub device_enabled: bool,
    pub alarm_mask: AlarmMask,
    pub disable_local_config: DisableLocalConfig,
    pub sw_build_id: String<16>,
}
