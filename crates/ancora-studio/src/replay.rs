//! Replay controls - step-through playback of a recorded run.

#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub struct ReplayControls {
    pub run_id: String,
    pub total_steps: usize,
    pub current_step: usize,
    pub state: PlaybackState,
    pub speed: f32,
}

impl ReplayControls {
    pub fn new(run_id: impl Into<String>, total_steps: usize) -> Self {
        Self {
            run_id: run_id.into(),
            total_steps,
            current_step: 0,
            state: PlaybackState::Stopped,
            speed: 1.0,
        }
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.current_step = 0;
    }

    pub fn step_forward(&mut self) -> Result<usize, &'static str> {
        if self.current_step + 1 < self.total_steps {
            self.current_step += 1;
            Ok(self.current_step)
        } else {
            Err("already at last step")
        }
    }

    pub fn step_back(&mut self) -> Result<usize, &'static str> {
        if self.current_step > 0 {
            self.current_step -= 1;
            Ok(self.current_step)
        } else {
            Err("already at first step")
        }
    }

    pub fn seek(&mut self, step: usize) -> Result<(), &'static str> {
        if step < self.total_steps {
            self.current_step = step;
            Ok(())
        } else {
            Err("step index out of range")
        }
    }

    pub fn set_speed(&mut self, speed: f32) -> Result<(), &'static str> {
        if speed > 0.0 && speed <= 16.0 {
            self.speed = speed;
            Ok(())
        } else {
            Err("speed must be between 0 and 16")
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current_step + 1 >= self.total_steps
    }

    pub fn progress_pct(&self) -> f32 {
        if self.total_steps == 0 {
            return 0.0;
        }
        self.current_step as f32 / self.total_steps as f32 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_forward_and_back() {
        let mut ctrl = ReplayControls::new("r1", 5);
        assert!(ctrl.step_forward().is_ok());
        assert_eq!(ctrl.current_step, 1);
        assert!(ctrl.step_back().is_ok());
        assert_eq!(ctrl.current_step, 0);
    }

    #[test]
    fn test_seek() {
        let mut ctrl = ReplayControls::new("r1", 10);
        assert!(ctrl.seek(5).is_ok());
        assert_eq!(ctrl.current_step, 5);
        assert!(ctrl.seek(10).is_err());
    }

    #[test]
    fn test_play_pause_stop() {
        let mut ctrl = ReplayControls::new("r1", 5);
        ctrl.play();
        assert_eq!(ctrl.state, PlaybackState::Playing);
        ctrl.pause();
        assert_eq!(ctrl.state, PlaybackState::Paused);
        ctrl.stop();
        assert_eq!(ctrl.state, PlaybackState::Stopped);
        assert_eq!(ctrl.current_step, 0);
    }
}
