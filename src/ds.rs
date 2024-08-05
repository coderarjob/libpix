pub struct Vec2(pub f32, pub f32);
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec2 {
    pub fn sum(mut self, n: f32) -> Self {
        self.0 += n;
        self.1 += n;
        self
    }
    pub fn sub(mut self, n: f32) -> Self {
        self.0 -= n;
        self.1 -= n;
        self
    }
    pub fn mul(mut self, n: f32) -> Self {
        self.0 *= n;
        self.1 *= n;
        self
    }
    pub fn div(mut self, n: f32) -> Self {
        self.0 /= n;
        self.1 /= n;
        self
    }
    pub fn fract(mut self) -> Self {
        self.0 = self.0.fract();
        self.1 = self.1.fract();
        self
    }

    pub fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl Vec3 {
    pub fn sum(mut self, n: f32) -> Self {
        self.0 += n;
        self.1 += n;
        self.2 += n;
        self
    }
    pub fn sub(mut self, n: f32) -> Self {
        self.0 -= n;
        self.1 -= n;
        self.2 -= n;
        self
    }
    pub fn mul(mut self, n: f32) -> Self {
        self.0 *= n;
        self.1 *= n;
        self.2 *= n;
        self
    }
    pub fn div(mut self, n: f32) -> Self {
        self.0 /= n;
        self.1 /= n;
        self.2 /= n;
        self
    }
    pub fn fract(mut self) -> Self {
        self.0 = self.0.fract();
        self.1 = self.1.fract();
        self.2 = self.2.fract();
        self
    }

    pub fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}
