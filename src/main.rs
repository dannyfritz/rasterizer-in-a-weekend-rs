use nalgebra_glm as glm;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
type Float = f32;
type Vec3F = glm::TVec3<Float>;
type Vec4F = glm::TVec4<Float>;
type Vec3U8 = glm::TVec3<u8>;
type Mat4F = glm::TMat4<Float>;

//TODO: Use scnee object
struct Scene {
    proj: Mat4F,
    view: Mat4F,
    vertices: Vec<Vec3F>,
    indices: Vec<usize>,
    colors: Vec<Vec3U8>,
    objects: Vec<Mat4F>,
}

fn main() {
    let obj =
        obj::Obj::<obj::SimplePolygon>::load(&std::path::Path::new("./teapot/teapot.obj")).unwrap();
    let near_plane = 0.1;
    let far_plane = 100.0;
    let eye = glm::vec3(15.0, 5.0, 10.5);
    let lookat = glm::vec3(0.0, 0.0, 0.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view = glm::look_at(&eye, &lookat, &up);
    let proj = glm::perspective(
        (SCREEN_WIDTH / SCREEN_HEIGHT) as Float,
        30.0,
        near_plane,
        far_plane,
    );
    let framebuffer = render(proj, view, obj);
    framebuffer.save("image.png").unwrap();
}

fn render(
    proj: Mat4F,
    view: Mat4F,
    obj: obj::Obj<obj::SimplePolygon>,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>> {
    let mut framebuffer = image::ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut depthbuffer = vec![0.0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];
    for object in obj.objects {
        for group in object.groups {
            for poly in group.polys {
                match poly.len() {
                    3 => {
                        let v0 = glm::make_vec3::<Float>(obj.position.get(poly.get(0).unwrap().0).unwrap());
                        let v1 = glm::make_vec3::<Float>(obj.position.get(poly.get(1).unwrap().0).unwrap());
                        let v2 = glm::make_vec3::<Float>(obj.position.get(poly.get(2).unwrap().0).unwrap());
                        let v0_clip = vs(&v0, glm::identity(), view, proj);
                        let v1_clip = vs(&v1, glm::identity(), view, proj);
                        let v2_clip = vs(&v2, glm::identity(), view, proj);
                        let v0_homogen = to_raster(&v0_clip);
                        let v1_homogen = to_raster(&v1_clip);
                        let v2_homogen = to_raster(&v2_clip);
                        let mut m = glm::mat3(
                            v0_homogen.x,
                            v0_homogen.y,
                            v0_homogen.w,
                            v1_homogen.x,
                            v1_homogen.y,
                            v1_homogen.w,
                            v2_homogen.x,
                            v2_homogen.y,
                            v2_homogen.w,
                        );
                        let determinant = glm::determinant(&m);
                        if determinant >= 0.0 {
                            continue;
                        }
                        m = glm::inverse(&m);
                        let e0 = glm::vec3(m[0], m[1], m[2]);
                        let e1 = glm::vec3(m[3], m[4], m[5]);
                        let e2 = glm::vec3(m[6], m[7], m[8]);
                        let c = m * glm::vec3(1.0, 1.0, 1.0);
                        for (x, y, pixel) in framebuffer.enumerate_pixels_mut() {
                            let sample = glm::vec3(x as Float + 0.5, y as Float + 0.5, 1.0);
                            let inside0 = evaluate_edge_function(&e0, &sample);
                            let inside1 = evaluate_edge_function(&e1, &sample);
                            let inside2 = evaluate_edge_function(&e2, &sample);
                            if inside0 && inside1 && inside2 {
                                let one_over_w = (c.x * sample.x) + (c.y * sample.y) + c.z;
                                if one_over_w >= depthbuffer[(x + y * SCREEN_WIDTH) as usize] {
                                    depthbuffer[(x + y * SCREEN_WIDTH) as usize] = one_over_w;
                                    let alpha = glm::dot(&e0, &sample);
                                    let beta = glm::dot(&e1, &sample);
                                    let charlie = glm::dot(&e2, &sample);
                                    *pixel =
                                        image::Rgb([(alpha * 20055.0) as u8, (beta * 20055.0) as u8, (charlie * 20055.0) as u8]);
                                }
                            }
                        }
                    }
                    //_ => println!("{}", poly.len()),
                    _ => {}
                }
            }
        }
    }
    framebuffer
}

fn evaluate_edge_function(e: &Vec3F, sample: &Vec3F) -> bool {
    let result = (e.x * sample.x) + (e.y * sample.y) + e.z;

    if result > 0.0 {
        return true;
    } else if result < 0.0 {
        return false;
    };

    if e.x > 0.0 {
        return true;
    } else if e.x < 0.0 {
        return false;
    };

    if e.x == 0.0 && e.y < 0.0 {
        return false;
    }
    return true;
}

fn vs(pos: &Vec3F, m: Mat4F, v: Mat4F, p: Mat4F) -> Vec4F {
    p * v * m * glm::vec4(pos.x, pos.y, pos.z, 1.0)
}

fn to_raster(v: &Vec4F) -> Vec4F {
    glm::vec4(
        SCREEN_WIDTH as Float * (v.x + v.w) / 2.0,
        SCREEN_HEIGHT as Float * (v.w - v.y) / 2.0,
        v.z,
        v.w,
    )
}
