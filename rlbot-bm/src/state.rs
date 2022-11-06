use std::{
    mem::transmute,
    ops::{Index, Sub},
};

use ref_cast::RefCast;

pub type Vec3 = [f32; 3];

fn convert_vec3(vec3: &rlbot_bm_sys::Vec3) -> Vec3 {
    [vec3.x, vec3.y, vec3.z]
}

pub type Quat = (f32, Vec3);

fn convert_quat(quat: &rlbot_bm_sys::Quat) -> Quat {
    (quat.w, [quat.x, quat.y, quat.z])
}

pub trait Physics {
    fn orientation(&self) -> Quat;
    fn position(&self) -> Vec3;
    fn velocity(&self) -> Vec3;
    fn angular_velocity(&self) -> Vec3;
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct Ball(rlbot_bm_sys::Ball);

impl Physics for Ball {
    fn orientation(&self) -> Quat {
        convert_quat(&self.0.orientation)
    }

    fn position(&self) -> Vec3 {
        convert_vec3(&self.0.position)
    }

    fn velocity(&self) -> Vec3 {
        convert_vec3(&self.0.velocity)
    }

    fn angular_velocity(&self) -> Vec3 {
        convert_vec3(&self.0.angularVelocity)
    }
}

impl Ball {
    pub fn radius(&self) -> f32 {
        self.0.radius
    }
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct Car(rlbot_bm_sys::Car);

impl Physics for Car {
    fn orientation(&self) -> Quat {
        convert_quat(&self.0.orientation)
    }

    fn position(&self) -> Vec3 {
        convert_vec3(&self.0.position)
    }

    fn velocity(&self) -> Vec3 {
        convert_vec3(&self.0.velocity)
    }

    fn angular_velocity(&self) -> Vec3 {
        convert_vec3(&self.0.angularVelocity)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct TeamId(u8);

/// taking the difference of two instances gives you the number of ticks
pub struct Instant(i32);

impl Sub for Instant {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Car {
    pub fn boost(&self) -> f32 {
        self.0.boost
    }

    pub fn team(&self) -> TeamId {
        TeamId(self.0.team)
    }

    pub fn hitbox_size(&self) -> Vec3 {
        convert_vec3(&self.0.hitbox)
    }

    pub fn hitbox_offset(&self) -> Vec3 {
        convert_vec3(&self.0.hitboxOffset)
    }

    /// Instant on which the car was demolished
    pub fn demolished_at(&self) -> Option<Instant> {
        (self.0.demolished() != 0).then_some(Instant(self.0.demolishedAt))
    }

    pub fn flipped_at(&self) -> Option<Instant> {
        (self.0.hasFlip() == 0).then_some(Instant(self.0.flippedAt))
    }

    pub fn jumped_at(&self) -> Option<Instant> {
        (self.0.jumped() != 0).then_some(unimplemented!())
    }

    pub fn is_supersonic(&self) -> bool {
        self.0.superSonic() != 0
    }
}

#[derive(RefCast, Default)]
#[repr(transparent)]
pub struct GameState(pub(crate) rlbot_bm_sys::GameStateObj);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct CarId(pub(crate) usize);

impl Index<CarId> for [Car] {
    type Output = Car;

    fn index(&self, index: CarId) -> &Self::Output {
        &self[index.0]
    }
}

impl GameState {
    pub fn cars(&self) -> &[Car] {
        // SAFETY: Car implements RefCast
        unsafe { transmute(&self.0.cars[..self.0.numCars as usize]) }
    }

    pub fn balls(&self) -> &[Ball] {
        // SAFETY: Ball implements RefCast
        unsafe { transmute(&self.0.balls[..self.0.numBalls as usize]) }
    }

    pub fn is_round_active(&self) -> bool {
        // SAFETY: both variants of this union are always valid
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1 }.roundActive() != 0
    }
}
