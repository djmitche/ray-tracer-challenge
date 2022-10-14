use crate::{mat4, Mat, Ray, Tup};

/// Camera represents a view onto the world space.
///
/// The "image" is assumed to be a rectangle centered on a point one unit
/// in front of the eye.
pub struct Camera {
    /// number of horizontal pixels in image
    hsize: usize,
    /// number of vertical pixels in image
    vsize: usize,
    /// field of view, in radians
    fov: f64,

    /// matrix translating camera coordinates to world coordinates
    inv_transform: Mat<4>,

    /// half the width of the image
    half_width: f64,

    /// half the height of the image
    half_height: f64,

    /// world-space size of a pixel
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64, from: Tup, to: Tup, up: Tup) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0 / hsize as f64) as f64;
        Self {
            hsize,
            vsize,
            fov,
            inv_transform: Self::view_transform(from, to, up).inverse(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    fn view_transform(from: Tup, to: Tup, up: Tup) -> Mat<4> {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        mat4![
            left.x, left.y, left.z, 0;
            true_up.x, true_up.y, true_up.z, 0;
            -forward.x, -forward.y, -forward.z, 0;
            0, 0, 0, 1;
        ] * mat4![
            1, 0, 0, -from.x;
            0, 1, 0, -from.y;
            0, 0, 1, -from.z;
            0, 0, 0, 1;
        ]
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // offset from edge of image to the _center_ of the pixel
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inv_transform * Tup::point(world_x, world_y, -1);
        let origin = self.inv_transform * Tup::point(0, 0, 0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn view_transform_default_orientation() {
        assert_relative_eq!(
            Camera::view_transform(
                Tup::point(0, 0, 0),
                Tup::point(0, 0, -1),
                Tup::vector(0, 1, 0)
            ),
            Mat::identity()
        );
    }

    #[test]
    fn view_transform_pos_z() {
        assert_relative_eq!(
            Camera::view_transform(
                Tup::point(0, 0, 0),
                Tup::point(0, 0, 1),
                Tup::vector(0, 1, 0)
            ),
            Mat::identity().scale(-1, 1, -1)
        );
    }

    #[test]
    fn view_transform_moves_world() {
        assert_relative_eq!(
            Camera::view_transform(
                Tup::point(0, 0, 8),
                Tup::point(0, 0, 0),
                Tup::vector(0, 1, 0)
            ),
            Mat::identity().translate(0, 0, -8)
        );
    }

    #[test]
    fn view_transform_arbitrary() {
        assert_relative_eq!(
            Camera::view_transform(
                Tup::point(1, 3, 2),
                Tup::point(4, -2, 8),
                Tup::vector(1, 1, 0)
            ),
            mat4![
            -0.5070925528371099,  0.5070925528371099, 0.6761234037828132,  -2.366431913239846;
            0.7677159338596801,   0.6060915267313263, 0.12121830534626524, -2.8284271247461894;
            -0.35856858280031806, 0.5976143046671968, -0.7171371656006361, 0;
            0,                    0,                  0,                   1;
                    ]
        );
    }

    #[test]
    fn camera_constr() {
        let cam = Camera::new(
            160,
            120,
            PI / 2.0,
            Tup::point(0, 0, 0),
            Tup::point(0, 0, -1),
            Tup::vector(0, 1, 0),
        );
        assert_eq!(cam.hsize, 160);
        assert_eq!(cam.vsize, 120);
        assert_relative_eq!(cam.fov, PI / 2.0);
        assert_relative_eq!(cam.inv_transform, Mat::identity());
    }

    #[test]
    fn camera_pixel_size_h() {
        let cam = Camera::new(
            200,
            125,
            PI / 2.0,
            Tup::point(0, 0, 0),
            Tup::point(0, 0, -1),
            Tup::vector(0, 1, 0),
        );
        assert_relative_eq!(cam.pixel_size, 0.01);
    }

    #[test]
    fn camera_pixel_size_v() {
        let cam = Camera::new(
            125,
            200,
            PI / 2.0,
            Tup::point(0, 0, 0),
            Tup::point(0, 0, -1),
            Tup::vector(0, 1, 0),
        );
        assert_relative_eq!(cam.pixel_size, 0.01);
    }

    #[test]
    fn ray_through_center() {
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Tup::point(0, 0, 0),
            Tup::point(0, 0, -1),
            Tup::vector(0, 1, 0),
        );
        let r = cam.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin, Tup::point(0, 0, 0));
        assert_relative_eq!(r.direction, Tup::vector(0, 0, -1));
    }

    #[test]
    fn ray_through_corner() {
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Tup::point(0, 0, 0),
            Tup::point(0, 0, -1),
            Tup::vector(0, 1, 0),
        );
        let r = cam.ray_for_pixel(0, 0);
        assert_relative_eq!(r.origin, Tup::point(0, 0, 0));
        assert_relative_eq!(
            r.direction,
            Tup::vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
        );
    }

    #[test]
    fn ray_transformed_camera() {
        let halfsqrt2 = 2f64.sqrt() / 2.0;
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Tup::point(0, 2, -5),
            Tup::point(halfsqrt2, 2, -halfsqrt2 - 5.0),
            Tup::vector(0, 1, 0),
        );
        let r = cam.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin, Tup::point(0, 2, -5));
        // the math here doesn't quite work out to the level of precision
        // expected by assert_relative_eq's default epsilon
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001,
        }
        .eq(&r.direction, &Tup::vector(halfsqrt2, 0, -halfsqrt2)));
    }
}
