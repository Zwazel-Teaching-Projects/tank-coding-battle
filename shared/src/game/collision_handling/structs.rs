use bevy::prelude::*;

use super::components::Collider;

pub struct Obb3d {
    pub center: Vec3,
    pub half_size: Vec3,
    pub orientation: Mat3,
}

impl Obb3d {
    pub fn new(transform: Transform, collider: &Collider) -> Self {
        let half_size = collider.half_size;
        let orientation = Mat3::from_quat(transform.rotation);
        Obb3d {
            center: transform.translation,
            half_size: half_size * transform.scale,
            orientation,
        }
    }

    /// Projects the OBB onto the given axis and returns the interval (min, max)
    /// of the projection. Assumes the axis is normalized.
    pub fn project(&self, axis: Vec3) -> (f32, f32) {
        // Project the center onto the axis.
        let center_proj = self.center.dot(axis);

        // Compute the contribution of each local axis scaled by the half-size.
        let extents = [
            self.orientation.x_axis * self.half_size.x,
            self.orientation.y_axis * self.half_size.y,
            self.orientation.z_axis * self.half_size.z,
        ];

        // The projection radius is the sum of the absolute dot products.
        let radius = extents.iter().map(|e| e.dot(axis).abs()).sum::<f32>();

        (center_proj - radius, center_proj + radius)
    }

    /// Tests whether the projections of self and other overlap on the given axis.
    pub fn intersects_axis(&self, other: &Obb3d, axis: Vec3) -> bool {
        let (min_a, max_a) = self.project(axis);
        let (min_b, max_b) = other.project(axis);
        // Overlap exists if the maximum of one is greater than or equal to the minimum of the other.
        max_a >= min_b && max_b >= min_a
    }

    /// Checks for intersection with another OBB using the Separating Axis Theorem (SAT).
    /// This implementation tests both the face normals and the edge-edge cross product axes.
    pub fn intersects(&self, other: &Obb3d) -> bool {
        // Array of the 6 face normals from both OBBs.
        let face_axes = [
            self.orientation.x_axis,
            self.orientation.y_axis,
            self.orientation.z_axis,
            other.orientation.x_axis,
            other.orientation.y_axis,
            other.orientation.z_axis,
        ];

        // Test the face normals.
        for &axis in face_axes.iter() {
            if !self.intersects_axis(other, axis) {
                return false;
            }
        }

        // Test the 9 axes derived from the cross products of edges.
        let self_axes = [
            self.orientation.x_axis,
            self.orientation.y_axis,
            self.orientation.z_axis,
        ];
        let other_axes = [
            other.orientation.x_axis,
            other.orientation.y_axis,
            other.orientation.z_axis,
        ];

        for &axis_a in self_axes.iter() {
            for &axis_b in other_axes.iter() {
                // Compute the cross product to get a potential separating axis.
                let axis = axis_a.cross(axis_b);
                // Skip axes that are nearly zero length (i.e., the edges are parallel).
                if axis.length_squared() < 1e-6 {
                    continue;
                }
                // Normalize the axis for proper projection.
                let axis = axis.normalize();
                if !self.intersects_axis(other, axis) {
                    return false;
                }
            }
        }
        true
    }
}
