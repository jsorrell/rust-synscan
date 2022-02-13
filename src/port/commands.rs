#![allow(unused)]

pub const QUERY_BYTE: u8 = b':';
pub const TERMINATION_BYTE: u8 = b'\r';
pub const SUCCESS_BYTE: u8 = b'=';
pub const ERROR_BYTE: u8 = b'!';

// Commands
pub const SET_POSITION: u8 = b'E';
pub const INITIALIZATION_DONE: u8 = b'F';
pub const SET_MOTION_MODE: u8 = b'G';
pub const SET_GOTO_TARGET_INCREMENT: u8 = b'H';
pub const SET_BRAKE_POINT_INCREMENT: u8 = b'M';
pub const SET_GOTO_TARGET: u8 = b'S';
pub const SET_STEP_PERIOD: u8 = b'I';
pub const SET_LONG_GOTO_STEP: u8 = b'T';
pub const SET_BRAKE_STEPS: u8 = b'U';
pub const START_MOTION: u8 = b'J';
pub const STOP_MOTION: u8 = b'K';
pub const INSTANT_STOP: u8 = b'L';
pub const SET_SLEEP: u8 = b'B';
pub const SET_AUX_SWITCH: u8 = b'O';
pub const SET_AUTOGUIDE_SPEED: u8 = b'P';
pub const RUN_BOOTLOADER_MODE: u8 = b'Q';
pub const SET_POLAR_SCOPE_BRIGHTNESS: u8 = b'V';
pub const INQUIRE_COUNTS_PER_REVOLUTION: u8 = b'a';
pub const INQUIRE_TIMER_INTERRUPT_FREQUENCY: u8 = b'b';
pub const INQUIRE_BRAKE_STEPS: u8 = b'c';
pub const INQUIRE_GOTO_TARGET_POSITION: u8 = b'h';
pub const INQUIRE_STEP_PERIOD: u8 = b'i';
pub const INQUIRE_POSITION: u8 = b'j';
pub const INQUIRE_INCREMENT: u8 = b'k';
pub const INQUIRE_BRAKE_POINT: u8 = b'm';
pub const INQUIRE_STATUS: u8 = b'f';
pub const INQUIRE_HIGH_SPEED_RATIO: u8 = b'g';
pub const INQUIRE_1X_TRACKING_PERIOD: u8 = b'D';
pub const INQUIRE_TELE_AXIS_POSITION: u8 = b'd';
pub const INQUIRE_MOTOR_BOARD_VERSION: u8 = b'e';
pub const INQUIRE_PEC_PERIOD: u8 = b's';
pub const SET_DEBUG_FLAG: u8 = b'z';
pub const EXTENDED_SETTING: u8 = b'W';
pub const EXTENDED_INQUIRE: u8 = b'q';
pub const SET_EEPROM_ADDRESS: u8 = b'C';
pub const SET_EEPROM_VALUE: u8 = b'N';
pub const INQUIRE_EEPROM_VALUE: u8 = b'n';
pub const SET_REGISTER_ADDRESS: u8 = b'A';
pub const SET_REGISTER_VALUE: u8 = b'R';
pub const INQUIRE_REGISTER_VALUE: u8 = b'r';
