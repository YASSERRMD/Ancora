use crate::bounds::ScaleBounds;
use crate::cooldown::Cooldown;
use crate::decision::ScaleDecision;
use crate::metrics::AutoscaleMetrics;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleDirection {
    Up,
    Down,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePolicy {
    /// Queue depth that triggers scale-up.
    pub scale_up_queue_depth: usize,
    /// Utilization fraction at or above which to scale up.
    pub scale_up_utilization: f64,
    /// Utilization fraction at or below which to scale down.
    pub scale_down_utilization: f64,
    /// Workers added per scale-up event.
    pub scale_up_step: usize,
    /// Workers removed per scale-down event.
    pub scale_down_step: usize,
    pub bounds: ScaleBounds,
    pub cooldown: Cooldown,
}

impl ScalePolicy {
    pub fn new(bounds: ScaleBounds) -> Self {
        ScalePolicy {
            scale_up_queue_depth: 5,
            scale_up_utilization: 0.8,
            scale_down_utilization: 0.2,
            scale_up_step: 1,
            scale_down_step: 1,
            bounds,
            cooldown: Cooldown::new(60, 120),
        }
    }

    pub fn evaluate(&mut self, m: &AutoscaleMetrics) -> ScaleDecision {
        let current = m.worker_count;

        // Scale-up conditions
        if (m.queue_depth >= self.scale_up_queue_depth
            || m.utilization >= self.scale_up_utilization)
            && !self.bounds.at_max(current)
            && self.cooldown.can_scale_up()
        {
            let by = self.bounds.clamp(current + self.scale_up_step) - current;
            if by > 0 {
                self.cooldown.record_scale_up();
                info!(by, "scale-up decision");
                return ScaleDecision::ScaleUp { by };
            }
        }

        // Scale-down conditions
        if m.utilization <= self.scale_down_utilization
            && m.queue_depth == 0
            && !self.bounds.at_min(current)
            && self.cooldown.can_scale_down()
        {
            let by = current - self.bounds.clamp(current.saturating_sub(self.scale_down_step));
            if by > 0 {
                self.cooldown.record_scale_down();
                info!(by, "scale-down decision");
                return ScaleDecision::ScaleDown { by };
            }
        }

        ScaleDecision::NoOp {
            reason: "within target range".to_string(),
        }
    }
}
