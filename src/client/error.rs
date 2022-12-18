use bevy::prelude::{Quat, Time, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMA {
    smoothing_factor: f64,
    value: f64,
    last_measurement_at: f64,
}

impl EMA {
    pub fn new(smoothing_factor: f64) -> EMA {
        return EMA {
            smoothing_factor,
            value: 0.0,
            last_measurement_at: 0.0,
        };
    }

    pub fn add_value(self: &mut EMA, time: Time, value: f64) {
        if self.last_measurement_at == 0.0 {
            self.value = value;
            return;
        }

        let this_measurement_at = time.elapsed_seconds_f64();

        // credit to bevy_diagnostic-0.9.1/src/diagnostic.rs
        let delta = this_measurement_at - self.last_measurement_at;
        let alpha = (delta / self.smoothing_factor).clamp(0.0, 1.0);
        self.value += alpha * (value - self.value);

        self.last_measurement_at = this_measurement_at;
    }

    fn set_value(self: &mut EMA, value: f64) {
        self.value = value;
        self.last_measurement_at = 0.0;
    }

    pub fn get_value(self: &EMA) -> f64 {
        return self.value;
    }

    pub fn reset(self: &mut EMA) {
        self.set_value(0.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec2EMA {
    x_ema: EMA,
    y_ema: EMA,
}

impl Vec2EMA {
    pub fn new(smoothing_factor: f64) -> Vec2EMA {
        let mut e = Vec2EMA {
            x_ema: EMA::new(smoothing_factor),
            y_ema: EMA::new(smoothing_factor),
        };

        e.set_value(Vec2::ZERO);

        return e;
    }

    pub fn add_value(self: &mut Vec2EMA, time: Time, value: Vec2) {
        self.x_ema.add_value(time.clone(), value.x as f64);
        self.y_ema.add_value(time.clone(), value.y as f64);
    }

    fn set_value(self: &mut Vec2EMA, value: Vec2) {
        self.x_ema.set_value(value.x as f64);
        self.y_ema.set_value(value.y as f64);
    }

    pub fn get_value(self: &Vec2EMA) -> Vec2 {
        return Vec2::new(self.x_ema.get_value() as f32, self.y_ema.get_value() as f32);
    }

    pub fn reset(self: &mut Vec2EMA) {
        self.set_value(Vec2::ZERO);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3EMA {
    x_ema: EMA,
    y_ema: EMA,
    z_ema: EMA,
}

impl Vec3EMA {
    pub fn new(smoothing_factor: f64) -> Vec3EMA {
        let mut e = Vec3EMA {
            x_ema: EMA::new(smoothing_factor),
            y_ema: EMA::new(smoothing_factor),
            z_ema: EMA::new(smoothing_factor),
        };

        e.set_value(Vec3::ZERO);

        return e;
    }

    pub fn add_value(self: &mut Vec3EMA, time: Time, value: Vec3) {
        self.x_ema.add_value(time.clone(), value.x as f64);
        self.y_ema.add_value(time.clone(), value.y as f64);
        self.z_ema.add_value(time.clone(), value.z as f64);
    }

    fn set_value(self: &mut Vec3EMA, value: Vec3) {
        self.x_ema.set_value(value.x as f64);
        self.y_ema.set_value(value.y as f64);
        self.z_ema.set_value(value.z as f64);
    }

    pub fn get_value(self: &Vec3EMA) -> Vec3 {
        return Vec3::new(
            self.x_ema.get_value() as f32,
            self.y_ema.get_value() as f32,
            self.z_ema.get_value() as f32,
        );
    }

    pub fn reset(self: &mut Vec3EMA) {
        self.set_value(Vec3::ZERO);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuatEMA {
    axis_ema: Vec3EMA,
    angle_ema: EMA,
}

impl QuatEMA {
    pub fn new(smoothing_factor: f64) -> QuatEMA {
        let mut e = QuatEMA {
            axis_ema: Vec3EMA::new(smoothing_factor),
            angle_ema: EMA::new(smoothing_factor),
        };

        e.set_value(Quat::IDENTITY);

        return e;
    }

    pub fn add_value(self: &mut QuatEMA, time: Time, value: Quat) {
        let (axis, angle) = value.to_axis_angle();

        self.axis_ema.add_value(time.clone(), axis);
        self.angle_ema.add_value(time.clone(), angle as f64);
    }

    fn set_value(self: &mut QuatEMA, value: Quat) {
        let (axis, angle) = value.to_axis_angle();

        self.axis_ema.set_value(axis);
        self.angle_ema.set_value(angle as f64);
    }

    pub fn get_value(self: &QuatEMA) -> Quat {
        return Quat::from_axis_angle(self.axis_ema.get_value(), self.angle_ema.get_value() as f32);
    }

    pub fn reset(self: &mut QuatEMA) {
        self.set_value(Quat::IDENTITY);
    }
}
