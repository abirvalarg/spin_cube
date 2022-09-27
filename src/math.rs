use ultraviolet::Vec3;

#[derive(Debug)]
pub struct Lin {
    k: f32,
    b: f32
}

impl From<([f32; 2], [f32; 2])> for Lin {
    fn from((p0, p1): ([f32; 2], [f32; 2])) -> Self {
        let dx = p1[0] - p0[0];
        let dy = p1[1] - p0[1];
        Lin {
            k: dy / dx,
            b: p0[1] - p0[0] * (dy / dx)
        }
    }
}

impl Lin {
    pub fn at(&self, x : f32) -> f32 {
        x * self.k + self.b
    }
}

#[derive(Debug)]
pub struct Surface {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl From<[Vec3; 3]> for Surface {
    fn from([p0, p1, p2]: [Vec3; 3]) -> Self {
        let v1 = p1 - p0;
        let v2 = p2 - p0;
        let n = v1.cross(v2);
        let n = n.normalized();
        let [a, b, c] = n.as_array().clone();
        let d = -n.dot(p0);
        Surface { a, b, c, d }
    }
}

impl Surface {
    pub fn at_x_y(&self, x: f32, y: f32) -> f32 {
        -(self.d + self.a * x + self.b * y) / self.c
    }
}
