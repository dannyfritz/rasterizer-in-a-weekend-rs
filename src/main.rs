use nalgebra_glm as glm;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut v0 = glm::vec3::<f64>(-0.5, 0.5, 1.0);
    let mut v1 = glm::vec3::<f64>(0.5, 0.5, 1.0);
    let mut v2 = glm::vec3::<f64>(0.0, -0.5, 1.0);
    v0 = to_raster(v0);
    v1 = to_raster(v1);
    v2 = to_raster(v2);
    let mut m = glm::mat3(v0.x, v0.y, v0.z, v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
    m = glm::inverse(&m);
    let e0 = m * glm::vec3(1.0, 0.0, 0.0);
    let e1 = m * glm::vec3(0.0, 1.0, 0.0);
    let e2 = m * glm::vec3(0.0, 0.0, 1.0);
    let mut framebuffer = image::ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    for (x, y, pixel) in framebuffer.enumerate_pixels_mut() {
        let sample = glm::vec3(f64::from(x) + 0.5, f64::from(y) + 0.5, 1.0);
        let alpha = glm::dot(&e0, &sample);
        let beta = glm::dot(&e1, &sample);
        let gamma = glm::dot(&e2, &sample);
        if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
            *pixel = image::Rgb([
                (alpha * 255.0) as u8,
                (beta * 255.0) as u8,
                (gamma * 255.0) as u8,
            ]);
        }
    }
    framebuffer.save("image.png").unwrap();
}

fn to_raster(v: glm::TVec3<f64>) -> glm::TVec3<f64> {
    glm::vec3(
        f64::from(SCREEN_WIDTH) * (v.x + 1.0) / 2.0,
        f64::from(SCREEN_HEIGHT) * (v.y + 1.0) / 2.0,
        1.0,
    )
}
