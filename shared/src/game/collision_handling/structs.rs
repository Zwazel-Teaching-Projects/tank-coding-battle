use bevy::{
    math::{Mat3A, Vec3A},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use super::components::Collider;

#[derive(Debug, Default, Clone, Copy, Reflect, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    #[default]
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Obb3d {
    pub center: Vec3A,
    pub basis: Mat3A,
    pub half_size: Vec3A,
}

impl Obb3d {
    pub fn from_transform(transform: &Transform, collider: &Collider) -> Self {
        let rotation_matrix = Mat3::from_quat(transform.rotation);
        let half_size = collider.half_size * transform.scale;
        Obb3d {
            center: transform.translation.into(),
            basis: rotation_matrix.into(),
            half_size: half_size.into(),
        }
    }

    pub fn intersects_obb(&self, other: &Obb3d) -> bool {
        // OBB-OBB intersection test using the Separating Axis Theorem (SAT).
        //
        // Reference: Real-Time Collision Detection by Christer Ericson, chapter 4.4.1.

        // Compute the translation between the centers of the OBBs in the frame of `self`.
        let t = self.basis * (other.center - self.center);

        // Compute the rotation matrix expressing `other` in the frame of `self`.
        let mut r = Mat3A::ZERO;
        let mut abs_r = Mat3A::ZERO;

        for i in 0..3 {
            for j in 0..3 {
                let element = self.basis.col(i).dot(other.basis.col(j));
                r.col_mut(i)[j] = element;
                // Add an epsilon term to account for errors when two edges
                // are parallel and their cross product is close to null.
                abs_r.col_mut(i)[j] = element.abs() + 1e-5;
            }
        }

        // We will test at most 15 axes:
        //
        // - The 3 coordinate axes of `self`
        // - The 3 coordinate axes of `other`
        // - The 9 axes perpendicular to an axis from each OBB
        //
        // The OBBs are separated if for *any* axis the sum of their
        // projected radii `ra` and `rb` is less than the distance
        // between their projected centers.

        let mut ra;
        let mut rb;

        // Test axes A0, A1, A2
        for i in 0..3 {
            ra = self.half_size[i];
            rb = other.half_size.dot(abs_r.col(i));
            if t[i].abs() > ra + rb {
                return false;
            }
        }

        // Test axes B0, B1, B2
        for i in 0..3 {
            ra = self.half_size.dot(abs_r.row(i));
            rb = other.half_size[i];
            if (t.dot(r.row(i))).abs() > ra + rb {
                return false;
            }
        }

        // Test axis A0 x B0
        ra = self.half_size.y * abs_r.z_axis.x + self.half_size.z * abs_r.y_axis.x;
        rb = other.half_size.y * abs_r.x_axis.z + other.half_size.z * abs_r.x_axis.y;
        if (t.z * r.y_axis.x - t.y * r.z_axis.x).abs() > ra + rb {
            return false;
        }

        // Test axis A0 x B1
        ra = self.half_size.y * abs_r.z_axis.y + self.half_size.z * abs_r.y_axis.y;
        rb = other.half_size.x * abs_r.x_axis.z + other.half_size.z * abs_r.x_axis.x;
        if (t.z * r.y_axis.y - t.y * r.z_axis.y).abs() > ra + rb {
            return false;
        }

        // Test axis A0 x B2
        ra = self.half_size.y * abs_r.z_axis.z + self.half_size.z * abs_r.y_axis.z;
        rb = other.half_size.x * abs_r.x_axis.y + other.half_size.y * abs_r.x_axis.x;
        if (t.z * r.y_axis.z - t.y * r.z_axis.z).abs() > ra + rb {
            return false;
        }

        // Test axis A1 x B0
        ra = self.half_size.x * abs_r.z_axis.x + self.half_size.z * abs_r.x_axis.x;
        rb = other.half_size.y * abs_r.y_axis.z + other.half_size.z * abs_r.y_axis.y;
        if (t.x * r.z_axis.x - t.z * r.x_axis.x).abs() > ra + rb {
            return false;
        }

        // Test axis A1 x B1
        ra = self.half_size.x * abs_r.z_axis.y + self.half_size.z * abs_r.x_axis.y;
        rb = other.half_size.x * abs_r.y_axis.z + other.half_size.z * abs_r.y_axis.x;
        if (t.x * r.z_axis.y - t.z * r.x_axis.y).abs() > ra + rb {
            return false;
        }

        // Test axis A1 x B2
        ra = self.half_size.x * abs_r.z_axis.z + self.half_size.z * abs_r.x_axis.z;
        rb = other.half_size.x * abs_r.y_axis.y + other.half_size.y * abs_r.y_axis.x;
        if (t.x * r.z_axis.z - t.z * r.x_axis.z).abs() > ra + rb {
            return false;
        }

        // Test axis A2 x B0
        ra = self.half_size.x * abs_r.y_axis.x + self.half_size.y * abs_r.x_axis.x;
        rb = other.half_size.y * abs_r.z_axis.z + other.half_size.z * abs_r.z_axis.y;
        if (t.y * r.x_axis.x - t.x * r.y_axis.x).abs() > ra + rb {
            return false;
        }

        // Test axis A2 x B1
        ra = self.half_size.x * abs_r.y_axis.y + self.half_size.y * abs_r.x_axis.y;
        rb = other.half_size.x * abs_r.z_axis.z + other.half_size.z * abs_r.z_axis.x;
        if (t.y * r.x_axis.y - t.x * r.y_axis.y).abs() > ra + rb {
            return false;
        }

        // Test axis A2 x B2
        ra = self.half_size.x * abs_r.y_axis.z + self.half_size.y * abs_r.x_axis.z;
        rb = other.half_size.x * abs_r.z_axis.y + other.half_size.y * abs_r.z_axis.x;
        if (t.y * r.x_axis.z - t.x * r.y_axis.z).abs() > ra + rb {
            return false;
        }

        // Because no separating axos was found, the OBBs must be intersecting.
        true
    }
}
