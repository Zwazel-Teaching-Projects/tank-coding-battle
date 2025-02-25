use bevy::prelude::*;

use super::components::Collider;

#[derive(Debug, Clone, Copy)]
pub struct Obb3d {
    pub center: Vec3,
    pub axes: Mat3,
    pub half_extents: Vec3,
}

impl Obb3d {
    pub fn from_transform(transform: &Transform, collider: &Collider) -> Self {
        let rotation_matrix = Mat3::from_quat(transform.rotation);
        let half_extents = collider.half_size * transform.scale;
        Obb3d {
            center: transform.translation,
            axes: rotation_matrix,
            half_extents,
        }
    }

    pub fn collides_with(&self, other: &Obb3d) -> bool {
        let separation_axes = self.get_separation_axes(other);
        let center_diff = other.center - self.center;

        for axis in separation_axes {
            if self.is_separating_axis(other, axis, center_diff) {
                return false;
            }
        }

        true
    }

    fn get_separation_axes(&self, other: &Obb3d) -> Vec<Vec3> {
        let mut axes = Vec::with_capacity(15);

        // Add self axes
        axes.push(self.axes.x_axis);
        axes.push(self.axes.y_axis);
        axes.push(self.axes.z_axis);

        // Add other axes
        axes.push(other.axes.x_axis);
        axes.push(other.axes.y_axis);
        axes.push(other.axes.z_axis);

        // Add cross products
        for &a in [self.axes.x_axis, self.axes.y_axis, self.axes.z_axis].iter() {
            for &b in [other.axes.x_axis, other.axes.y_axis, other.axes.z_axis].iter() {
                let cross = a.cross(b);
                if !cross.is_near_zero() {
                    axes.push(cross.normalize());
                }
            }
        }

        axes
    }

    fn is_separating_axis(&self, other: &Obb3d, axis: Vec3, center_diff: Vec3) -> bool {
        let proj_self = self.project_onto_axis(axis);
        let proj_other = other.project_onto_axis(axis);

        let projected_center_diff = center_diff.dot(axis).abs();
        let sum_half_projections =
            (proj_self.1 - proj_self.0) / 2.0 + (proj_other.1 - proj_other.0) / 2.0;

        projected_center_diff > sum_half_projections
    }

    fn project_onto_axis(&self, axis: Vec3) -> (f32, f32) {
        let projection = self.center.dot(axis);
        let half_projection = self.axes.x_axis.dot(axis).abs() * self.half_extents.x
            + self.axes.y_axis.dot(axis).abs() * self.half_extents.y
            + self.axes.z_axis.dot(axis).abs() * self.half_extents.z;

        (projection - half_projection, projection + half_projection)
    }
}

trait Vec3Ext {
    fn is_near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    fn is_near_zero(&self) -> bool {
        const EPSILON: f32 = 1e-6;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
}
