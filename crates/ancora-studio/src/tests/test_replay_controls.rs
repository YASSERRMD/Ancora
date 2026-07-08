use crate::replay::{PlaybackState, ReplayControls};

#[test]
fn test_replay_controls_drive_backend_step_forward() {
    let mut ctrl = ReplayControls::new("r1", 5);
    ctrl.play();
    assert_eq!(ctrl.state, PlaybackState::Playing);
    let result = ctrl.step_forward();
    assert!(result.is_ok());
    assert_eq!(ctrl.current_step, 1);
}

#[test]
fn test_replay_seek_to_step() {
    let mut ctrl = ReplayControls::new("r1", 10);
    assert!(ctrl.seek(7).is_ok());
    assert_eq!(ctrl.current_step, 7);
    assert!((ctrl.progress_pct() - 70.0).abs() < 0.01);
}

#[test]
fn test_replay_out_of_bounds_seek() {
    let mut ctrl = ReplayControls::new("r1", 5);
    assert!(ctrl.seek(5).is_err());
    assert!(ctrl.seek(100).is_err());
}

#[test]
fn test_replay_at_end() {
    let mut ctrl = ReplayControls::new("r1", 3);
    ctrl.seek(2).unwrap();
    assert!(ctrl.is_at_end());
}

#[test]
fn test_replay_speed() {
    let mut ctrl = ReplayControls::new("r1", 5);
    assert!(ctrl.set_speed(2.0).is_ok());
    assert!((ctrl.speed - 2.0).abs() < 1e-6);
    assert!(ctrl.set_speed(0.0).is_err());
    assert!(ctrl.set_speed(32.0).is_err());
}

#[test]
fn test_replay_stop_resets() {
    let mut ctrl = ReplayControls::new("r1", 5);
    ctrl.seek(3).unwrap();
    ctrl.play();
    ctrl.stop();
    assert_eq!(ctrl.current_step, 0);
    assert_eq!(ctrl.state, PlaybackState::Stopped);
}
