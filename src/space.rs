use std::f64::consts::PI;

/// 5e PHB, p. 204
#[derive(Debug,Clone)]
pub enum Area {
    Line { length: f64, width: f64 },
    Cylinder { height: f64, radius: f64 },
    Sphere { radius: f64 },
    Cone { length: f64 },
    Cube { length: f64 },
}

impl Area {
    /// Determine approximate lateral area subtended by this figure in its default orientation.
    pub fn floor_area(&self) -> f64 {
        match self {
            &Area::Line { length: l, width: w } => l * w,
            &Area::Cylinder { radius: r, .. } => PI * r * r,
            &Area::Sphere { radius: r } => PI * r * r,
            &Area::Cone { length: l } => 0.5 * 3.0f64.sqrt() * l * l,
            &Area::Cube { length: l } => l * l,
        }
    }
}
