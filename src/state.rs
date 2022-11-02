use ref_cast::RefCast;

#[derive(RefCast)]
#[repr(transparent)]
pub struct Vec3(rlbot_bm_sys::Vec3);

impl Vec3 {
    pub fn data(&self) -> [f32; 3] {
        [self.0.x, self.0.y, self.0.z]
    }
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct Quat(rlbot_bm_sys::Quat);

impl Quat {
    pub fn data(&self) -> (f32, [f32; 3]) {
        (self.0.w, [self.0.x, self.0.y, self.0.z])
    }
}

pub trait Physics {
    fn orientation(&self) -> &Quat;
    fn position(&self) -> &Vec3;
    fn velocity(&self) -> &Vec3;
    fn angular_velocity(&self) -> &Vec3;
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct Ball(rlbot_bm_sys::Ball);

impl Physics for Ball {
    fn orientation(&self) -> &Quat {
        Quat::ref_cast(&self.0.orientation)
    }

    fn position(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.position)
    }

    fn velocity(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.velocity)
    }

    fn angular_velocity(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.angularVelocity)
    }
}

impl Ball {
    pub fn radius(&self) -> f32 {
        self.0.radius
    }
}

pub struct Car(rlbot_bm_sys::Car);

impl Physics for Car {
    fn orientation(&self) -> &Quat {
        Quat::ref_cast(&self.0.orientation)
    }

    fn position(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.position)
    }

    fn velocity(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.velocity)
    }

    fn angular_velocity(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.angularVelocity)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Team(u8);

impl Car {
    pub fn boost(&self) -> f32 {
        self.0.boost
    }

    pub fn team(&self) -> Team {
        Team(self.0.team)
    }

    pub fn hitbox_size(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.hitbox)
    }

    pub fn hitbox_offset(&self) -> &Vec3 {
        Vec3::ref_cast(&self.0.hitboxOffset)
    }

    /// tick on which the car was demolished
    pub fn demolished_at(&self) -> u32 {
        self.0.demolishedAt as u32
    }

    pub fn flipped_at(&self) -> u32 {
        self.0.flippedAt as u32
    }
}
