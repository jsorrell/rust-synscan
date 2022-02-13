use crate::port::mock::MockSynScanPort;
use crate::port::SynScanPort;
use crate::util::*;
use crate::Direction::*;
use crate::*;

use crate::port::commands::*;
use crate::MultiChannel::*;
use crate::SingleChannel::*;

fn get_mc<T: SerialPort>(mock: T, params: Option<MotorParameters>) -> MotorController<T> {
    let params = params.unwrap_or(MotorParameters {
        counts_per_revolution: BiChannelValue::new(169499, 180),
        timer_interrupt_freq: 1000,
        high_speed_ratio: BiChannelValue::new(16, 2),
    });

    MotorController {
        port: SynScanPort(mock),
        motor_parameters: params,
    }
}

#[test]
fn test_set_pos() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);

    mc.set_pos(MultiChannel::Both, -1234).unwrap();
    mock.check_correct_number_written(SET_POSITION, MultiChannel::Both, 0x800000 - 1234, 6);

    mc.set_pos_degrees(SingleChannel::Channel2, 90.).unwrap();
    mock.check_correct_number_written(SET_POSITION, SingleChannel::Channel2, 0x800000 + 45, 6);
}

#[test]
fn test_set_motion_mode() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mc.set_motion_mode(SingleChannel::Channel1, DriveMode::Goto, true, Clockwise)
        .unwrap();
    mock.check_correct_query_written(SET_MOTION_MODE, Channel1, &[b'0', b'0']);
    mc.set_motion_mode(
        SingleChannel::Channel2,
        DriveMode::Tracking,
        true,
        CounterClockwise,
    )
    .unwrap();
    mock.check_correct_query_written(SET_MOTION_MODE, Channel2, &[b'3', b'1']);
}

#[test]
fn test_set_goto_target() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mc.set_goto_target(MultiChannel::Both, -1234).unwrap();
    mock.check_correct_number_written(SET_GOTO_TARGET, Both, 0x800000 - 1234, 6);
    mc.set_goto_target_degrees(SingleChannel::Channel2, 90.)
        .unwrap();
    mock.check_correct_number_written(SET_GOTO_TARGET, Channel2, 0x800000 + 45, 6);
}

#[test]
fn test_set_step_period() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);

    mc.set_step_period(MultiChannel::Both, 999).unwrap();
    mock.check_correct_number_written(SET_STEP_PERIOD, Both, 999, 6);
}

#[test]
fn test_start_stop_motion() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mc.start_motion(SingleChannel::Channel1).unwrap();
    mock.check_correct(START_MOTION, Channel1);
    mc.stop_motion(MultiChannel::Both).unwrap();
    mock.check_correct(STOP_MOTION, Both);
    mc.instant_stop(SingleChannel::Channel2).unwrap();
    mock.check_correct(INSTANT_STOP, Channel2);
}

#[test]
fn test_set_autoguide_speed() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mc.set_autoguide_speed(SingleChannel::Channel1, AutoGuideSpeed::ThreeQuarters)
        .unwrap();
    mock.check_correct_query_written(SET_AUTOGUIDE_SPEED, Channel1, &[b'1'])
}

#[test]
fn test_get_goto_target() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mock.add_valid_number(0x800000 + 25, 6);
    assert_eq!(mc.inquire_goto_target(SingleChannel::Channel1).unwrap(), 25);
    mock.check_correct(INQUIRE_GOTO_TARGET_POSITION, Channel1);

    mock.add_valid_number(0x800000 - 45, 6);
    assert_eq!(
        mc.inquire_goto_target_degrees(SingleChannel::Channel2)
            .unwrap(),
        -90.
    );
    mock.check_correct(INQUIRE_GOTO_TARGET_POSITION, Channel2);
}

#[test]
fn test_get_step_period() {
    let mock = MockSynScanPort::new();
    mock.add_valid_number(800, 6);
    let mut mc = get_mc(mock.clone(), None);
    assert_eq!(
        mc.inquire_step_period(SingleChannel::Channel2).unwrap(),
        800
    );
    mock.check_correct(INQUIRE_STEP_PERIOD, Channel2);
}

#[test]
fn test_get_pos() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mock.add_valid_number(0x800000 + 25, 6);
    assert_eq!(mc.inquire_pos(Channel1).unwrap(), 25);
    mock.check_correct(INQUIRE_POSITION, Channel1);

    mock.add_valid_number(0x800000 - 45, 6);
    assert_eq!(mc.inquire_pos_degrees(Channel2).unwrap(), -90.);
    mock.check_correct(INQUIRE_POSITION, Channel2);
}

#[test]
fn test_get_status() {
    let mock = MockSynScanPort::new();
    let mut mc = get_mc(mock.clone(), None);
    mock.add_valid_response(&[b'7', b'1', b'1']);
    assert_eq!(
        mc.inquire_status(Channel1).unwrap(),
        MotorStatus {
            mode: DriveMode::Tracking,
            direction: Direction::CounterClockwise,
            fast: true,
            running: true,
            blocked: false,
            inited: true,
            level_switch: false,
        }
    );
    mock.check_correct(INQUIRE_STATUS, Channel1);

    mock.add_valid_response(&[b'0', b'2', b'3']);
    assert_eq!(
        mc.inquire_status(SingleChannel::Channel2).unwrap(),
        MotorStatus {
            mode: DriveMode::Goto,
            direction: Direction::Clockwise,
            fast: false,
            running: false,
            blocked: true,
            inited: true,
            level_switch: true,
        }
    );
    mock.check_correct(INQUIRE_STATUS, Channel2);

    mock.add_valid_response(&[b'7', b'3', b'3']);
    assert_eq!(
        mc.inquire_status(Channel2).unwrap(),
        MotorStatus {
            mode: DriveMode::Tracking,
            direction: Direction::CounterClockwise,
            fast: true,
            running: true,
            blocked: true,
            inited: true,
            level_switch: true,
        }
    );
    mock.check_correct(INQUIRE_STATUS, Channel2);
}
