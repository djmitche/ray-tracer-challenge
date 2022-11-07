use crate::{mat4, spaces, Color, Mat, Point, Ray, Vector, World};
use image::RgbImage;
use rayon::prelude::*;

/// Camera represents a view onto the world space.
///
/// The "image" is assumed to be a rectangle centered on a point one unit
/// in front of the eye.
pub struct Camera {
    /// Horizontal pixel count
    hsize: u32,

    /// Vertical pixel count
    vsize: u32,

    /// matrix translating camera coordinates to world coordinates
    inv_transform: Mat<4, spaces::Camera, spaces::World>,

    /// half the width of the image
    half_width: f64,

    /// half the height of the image
    half_height: f64,

    /// world-space size of a pixel
    pixel_size: f64,

    /// number of times to oversample each pixel
    oversample: u32,
}

impl Camera {
    pub fn new(
        hsize: u32,
        vsize: u32,
        fov: f64,
        from: Point<spaces::World>,
        to: Point<spaces::World>,
        up: Vector<spaces::World>,
        oversample: u32,
    ) -> Self {
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
            inv_transform: Self::view_transform(from, to, up).inverse(),
            pixel_size,
            half_width,
            half_height,
            oversample,
        }
    }

    fn view_transform(
        from: Point<spaces::World>,
        to: Point<spaces::World>,
        up: Vector<spaces::World>,
    ) -> Mat<4, spaces::World, spaces::Camera> {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let xform: Mat<4, spaces::World, spaces::Camera> = mat4![
            left.x, left.y, left.z, 0;
            true_up.x, true_up.y, true_up.z, 0;
            -forward.x, -forward.y, -forward.z, 0;
            0, 0, 0, 1;
        ];
        // apply transform
        xform
            * mat4![
                1, 0, 0, -from.x;
                0, 1, 0, -from.y;
                0, 0, 1, -from.z;
                0, 0, 0, 1;
            ]
    }

    fn ray_for_pixel(&self, x: u32, y: u32, xover: f64, yover: f64) -> Ray<spaces::World> {
        // offset from edge of image to the _center_ of the pixel
        let xoffset = (x as f64 + xover) * self.pixel_size;
        let yoffset = (y as f64 + yover) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inv_transform * Point::new(world_x, world_y, -1);
        let origin = self.inv_transform * Point::new(0, 0, 0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    /// Determine the color at the given x and y coordinates of the image.
    pub fn color_at(&self, x: u32, y: u32, world: &World) -> Color {
        let mut acc = Color::black();

        let overfactor = 1.0 / (self.oversample + 1) as f64;
        for xo in 1..=self.oversample {
            for yo in 1..=self.oversample {
                let ray = self.ray_for_pixel(x, y, overfactor * xo as f64, overfactor * yo as f64);
                acc = acc + world.color_at(&ray, 1.0);
            }
        }

        acc / (self.oversample * self.oversample) as f64
    }

    /// Create an image
    pub fn render(&self, world: &World) -> RgbImage {
        let mut img = RgbImage::new(self.hsize, self.vsize);
        img.enumerate_rows_mut().par_bridge().for_each(|(y, row)| {
            for (x, _, p) in row {
                *p = self.color_at(x, y, world).into();
            }
        });
        img
    }

    /// Render the image as a buffer of u32's
    pub fn u32_buffer(&self, world: &World, buf: &mut [u32]) {
        let hsize = self.hsize as usize;
        let vsize = self.vsize as usize;
        debug_assert_eq!(buf.len(), hsize * vsize);

        // parallelize over rows
        buf.chunks_mut(hsize)
            .enumerate()
            .par_bridge()
            .for_each(|(y, row)| {
                for x in 0..hsize {
                    row[x] = self.color_at(x as u32, y as u32, world).into();
                }
            });
    }
}

impl IntoIterator for &Camera {
    type Item = (u32, u32);
    type IntoIter = PixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            hsize: self.hsize,
            vsize: self.vsize,
            nextx: 0,
            nexty: 0,
        }
    }
}

pub struct PixelIterator {
    hsize: u32,
    vsize: u32,
    nextx: u32,
    nexty: u32,
}

impl Iterator for PixelIterator {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let rv = if self.nexty < self.vsize {
            (self.nextx, self.nexty)
        } else {
            return None;
        };

        self.nextx += 1;
        if self.nextx >= self.hsize {
            self.nextx = 0;
            self.nexty += 1;
        }
        Some(rv)
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
                Point::new(0, 0, 0),
                Point::new(0, 0, -1),
                Vector::new(0, 1, 0)
            ),
            Mat::identity()
        );
    }

    #[test]
    fn view_transform_pos_z() {
        assert_relative_eq!(
            Camera::view_transform(
                Point::new(0, 0, 0),
                Point::new(0, 0, 1),
                Vector::new(0, 1, 0)
            ),
            Mat::identity().scale(-1, 1, -1)
        );
    }

    #[test]
    fn view_transform_moves_world() {
        assert_relative_eq!(
            Camera::view_transform(
                Point::new(0, 0, 8),
                Point::new(0, 0, 0),
                Vector::new(0, 1, 0)
            ),
            Mat::identity().translate(0, 0, -8)
        );
    }

    #[test]
    fn view_transform_arbitrary() {
        assert_relative_eq!(
            Camera::view_transform(
                Point::new(1, 3, 2),
                Point::new(4, -2, 8),
                Vector::new(1, 1, 0)
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
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
            1,
        );
        //assert_eq!(cam.hsize, 160);
        //assert_eq!(cam.vsize, 120);
        //assert_relative_eq!(cam.fov, PI / 2.0);
        assert_relative_eq!(cam.inv_transform, Mat::identity());
    }

    #[test]
    fn camera_pixel_size_h() {
        let cam = Camera::new(
            200,
            125,
            PI / 2.0,
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
            1,
        );
        assert_relative_eq!(cam.pixel_size, 0.01);
    }

    #[test]
    fn camera_pixel_size_v() {
        let cam = Camera::new(
            125,
            200,
            PI / 2.0,
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
            1,
        );
        assert_relative_eq!(cam.pixel_size, 0.01);
    }

    #[test]
    fn ray_through_center() {
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
            1,
        );
        let r = cam.ray_for_pixel(100, 50, 0.5, 0.5);
        assert_relative_eq!(r.origin, Point::new(0, 0, 0));
        assert_relative_eq!(r.direction, Vector::new(0, 0, -1));
    }

    #[test]
    fn ray_through_corner() {
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
            1,
        );
        let r = cam.ray_for_pixel(0, 0, 0.5, 0.5);
        assert_relative_eq!(r.origin, Point::new(0, 0, 0));
        assert_relative_eq!(
            r.direction,
            Vector::new(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
        );
    }

    #[test]
    fn ray_transformed_camera() {
        let halfsqrt2 = 2f64.sqrt() / 2.0;
        let cam = Camera::new(
            201,
            101,
            PI / 2.0,
            Point::new(0, 2, -5),
            Point::new(halfsqrt2, 2, -halfsqrt2 - 5.0),
            Vector::new(0, 1, 0),
            1,
        );
        let r = cam.ray_for_pixel(100, 50, 0.5, 0.5);
        assert_relative_eq!(r.origin, Point::new(0, 2, -5));
        // the math here doesn't quite work out to the level of precision
        // expected by assert_relative_eq's default epsilon
        assert!(Relative {
            epsilon: 0.00001,
            max_relative: 0.00001,
        }
        .eq(&r.direction, &Vector::new(halfsqrt2, 0, -halfsqrt2)));
    }
}
