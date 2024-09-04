
use nalgebra as na;
pub struct UnitSphere {
    pub bbox: tessellation::BoundingBox<f64>,
}

impl UnitSphere {
    pub fn new() -> UnitSphere {
        UnitSphere {
            bbox: tessellation::BoundingBox::new(
                &na::geometry::Point3::new(0., 0., 0.),
                &na::geometry::Point3::new(1., 2., 3.)
            ),
        }
    }
}
