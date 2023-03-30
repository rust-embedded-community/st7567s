//! Controller commands

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

/// ST7567S command representation\
/// All commands are 8-bit long\
/// Some commands have data bytes following them\
pub(crate) trait Command {
    fn command(&self) -> u8;
    fn data(&self) -> Option<u8> {
        None
    }

    /// Write command data to display
    fn write(&self, display_interface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        let mut bytes = [0; 2];
        let mut len = 1;
        bytes[0] = self.command();
        if let Some(data) = self.data() {
            bytes[1] = data;
            len = 2;
        }

        display_interface.send_commands(DataFormat::U8(&bytes[..len]))?;

        Ok(())
    }
}

// Commands implementation
// Reference: https://www.buydisplay.com/download/ic/ST7567S.pdf, chapter 9

/// Select bias setting: 1/9 or 1/7
#[allow(dead_code)]
pub(crate) enum SetBiasCommand {
    Bias1_7,
    Bias1_9,
}

impl Command for SetBiasCommand {
    fn command(&self) -> u8 {
        match self {
            SetBiasCommand::Bias1_7 => 0xa3,
            SetBiasCommand::Bias1_9 => 0xa2,
        }
    }
}

/// Set scan direction of SEG
#[allow(dead_code)]
pub(crate) enum SetSEGDirectionCommand {
    Normal,
    Reverse,
}

impl Command for SetSEGDirectionCommand {
    fn command(&self) -> u8 {
        match self {
            SetSEGDirectionCommand::Normal => 0xa0,
            SetSEGDirectionCommand::Reverse => 0xa1,
        }
    }
}

/// Set output direction of COM
#[allow(dead_code)]
pub(crate) enum SetCOMDirectionCommand {
    Normal,
    Reverse,
}

impl Command for SetCOMDirectionCommand {
    fn command(&self) -> u8 {
        match self {
            SetCOMDirectionCommand::Normal => 0xc0,
            SetCOMDirectionCommand::Reverse => 0xc8,
        }
    }
}

/// Select regulation resistor ratio
#[allow(dead_code)]
pub(crate) enum SetRegulationResistorRatioCommand {
    Ratio3_0,
    Ratio3_5,
    Ratio4_0,
    Ratio4_5,
    Ratio5_0,
    Ratio5_5,
    Ratio6_0,
    Ratio6_5,
}

impl Command for SetRegulationResistorRatioCommand {
    fn command(&self) -> u8 {
        match self {
            SetRegulationResistorRatioCommand::Ratio3_0 => 0x20,
            SetRegulationResistorRatioCommand::Ratio3_5 => 0x21,
            SetRegulationResistorRatioCommand::Ratio4_0 => 0x22,
            SetRegulationResistorRatioCommand::Ratio4_5 => 0x23,
            SetRegulationResistorRatioCommand::Ratio5_0 => 0x24,
            SetRegulationResistorRatioCommand::Ratio5_5 => 0x25,
            SetRegulationResistorRatioCommand::Ratio6_0 => 0x26,
            SetRegulationResistorRatioCommand::Ratio6_5 => 0x27,
        }
    }
}

/// Set electronic volume (EV) level
pub(crate) struct SetElectronicVolumeCommand {
    /// EV level (0-63)
    level: u8,
}

impl SetElectronicVolumeCommand {
    pub(crate) fn new(level: u8) -> Option<Self> {
        if level > 63 {
            None
        } else {
            Some(Self { level })
        }
    }
}

impl Command for SetElectronicVolumeCommand {
    fn command(&self) -> u8 {
        0x81
    }
    fn data(&self) -> Option<u8> {
        Some(self.level)
    }
}

/// Set Power Control
pub(crate) enum SetPowerControlCommand {
    BoosterOn,
    VoltageRegulatorOn,
    VoltageFollowerOn,
}

impl Command for SetPowerControlCommand {
    fn command(&self) -> u8 {
        match self {
            SetPowerControlCommand::BoosterOn => 0x2c,
            SetPowerControlCommand::VoltageRegulatorOn => 0x2e,
            SetPowerControlCommand::VoltageFollowerOn => 0x2f,
        }
    }
}

/// Set start line
pub(crate) struct SetStartLineCommand {
    /// Start line (0-63)
    line: u8,
}

impl SetStartLineCommand {
    pub(crate) fn new(line: u8) -> Option<Self> {
        if line > 63 {
            None
        } else {
            Some(Self { line })
        }
    }
}

impl Command for SetStartLineCommand {
    fn command(&self) -> u8 {
        0x40 | self.line
    }
}

/// Set page address
pub(crate) struct SetPageAddressCommand {
    /// Page address (0-7)
    address: u8,
}

impl SetPageAddressCommand {
    pub(crate) fn new(address: u8) -> Option<Self> {
        if address > 7 {
            None
        } else {
            Some(Self { address })
        }
    }
}

impl Command for SetPageAddressCommand {
    fn command(&self) -> u8 {
        0xb0 | self.address
    }
}

/// Set Column address
pub(crate) struct SetColumnAddressLSNibbleCommand {
    /// Column address (0-131)
    address: u8,
}

impl SetColumnAddressLSNibbleCommand {
    pub(crate) fn new(address: u8) -> Option<Self> {
        if address > 131 {
            None
        } else {
            Some(Self { address })
        }
    }
}

impl Command for SetColumnAddressLSNibbleCommand {
    fn command(&self) -> u8 {
        0x00 | (self.address & 0x0f)
    }
}

pub(crate) struct SetColumnAddressMSNibbleCommand {
    /// Column address (0-131)
    address: u8,
}

impl SetColumnAddressMSNibbleCommand {
    pub(crate) fn new(address: u8) -> Option<Self> {
        if address > 131 {
            None
        } else {
            Some(Self { address })
        }
    }
}

impl Command for SetColumnAddressMSNibbleCommand {
    fn command(&self) -> u8 {
        0x10 | ((self.address >> 4) & 0x0f)
    }
}

/// Set display on/off
#[allow(dead_code)]
pub(crate) enum DisplayOnCommand {
    On,
    Off,
}

impl Command for DisplayOnCommand {
    fn command(&self) -> u8 {
        match self {
            DisplayOnCommand::On => 0xaf,
            DisplayOnCommand::Off => 0xae,
        }
    }
}

/// Reset display
pub(crate) struct ResetCommand;

impl Command for ResetCommand {
    fn command(&self) -> u8 {
        0xe2
    }
}
