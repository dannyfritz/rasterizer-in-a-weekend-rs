use nalgebra_glm as glm;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let vertices = vec![
        glm::vec3(1.0, -1.0, -1.0),
        glm::vec3(1.0, -1.0, 1.0),
        glm::vec3(-1.0, -1.0, 1.0),
        glm::vec3(-1.0, -1.0, -1.0),
        glm::vec3(1.0, 1.0, -1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(-1.0, 1.0, 1.0),
        glm::vec3(-1.0, 1.0, -1.0),
    ];
    let indicies: Vec<usize> = vec![
        1, 3, 0, 7, 5, 4, 4, 1, 0, 5, 2, 1, 2, 7, 3, 0, 7, 4, 1, 2, 3, 7, 6, 5, 4, 5, 1, 5, 6, 2,
        2, 6, 7, 0, 3, 7,
    ];
    let colors = vec![
        glm::vec3(0, 0, 1),
        glm::vec3(0, 1, 0),
        glm::vec3(0, 1, 1),
        glm::vec3(1, 1, 1),
        glm::vec3(1, 0, 1),
        glm::vec3(1, 1, 0),
    ];
    let mut objects = vec![];
    initialize_scene_objects(&mut objects);
    let near_plane = 0.1;
    let far_plane = 100.0;
    let eye = glm::vec3(0.0, -3.75, 6.5);
    let lookat = glm::vec3(0.0, 0.0, 0.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view = glm::look_at(&eye, &lookat, &up);
    let proj = glm::perspective(
        f64::from(SCREEN_WIDTH / SCREEN_HEIGHT),
        30.0,
        near_plane,
        far_plane,
    );
    let mut framebuffer = image::ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut depthbuffer = vec![0.0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];
    for object in objects {
        for i in 0..indicies.len() / 3 {
            let v0 = vertices[indicies[i * 3 + 0]];
            let v1 = vertices[indicies[i * 3 + 1]];
            let v2 = vertices[indicies[i * 3 + 2]];
            let v0_clip = vs(&v0, object, view, proj);
            let v1_clip = vs(&v1, object, view, proj);
            let v2_clip = vs(&v2, object, view, proj);
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
                let sample = glm::vec3(f64::from(x) + 0.5, f64::from(y) + 0.5, 1.0);
                let inside0 = evaluate_edge_function(&e0, &sample);
                let inside1 = evaluate_edge_function(&e1, &sample);
                let inside2 = evaluate_edge_function(&e2, &sample);
                if inside0 && inside1 && inside2 {
                    let one_over_w = (c.x * sample.x) + (c.y * sample.y) + c.z;
                    if one_over_w >= depthbuffer[(x + y * SCREEN_WIDTH) as usize] {
                        depthbuffer[(x + y * SCREEN_WIDTH) as usize] = one_over_w;
                        let color = colors[indicies[(3 * i)] % colors.len()];
                        *pixel = image::Rgb([color.x * 255, color.y * 255, color.z * 255]);
                    }
                }
            }
        }
    }
    framebuffer.save("image.png").unwrap();
}

fn evaluate_edge_function(e: &glm::TVec3<f64>, sample: &glm::TVec3<f64>) -> bool {
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

fn vs(
    pos: &glm::TVec3<f64>,
    m: glm::TMat4<f64>,
    v: glm::TMat4<f64>,
    p: glm::TMat4<f64>,
) -> glm::TVec4<f64> {
    p * v * m * glm::vec4(pos.x, pos.y, pos.z, 1.0)
}

fn to_raster(v: &glm::TVec4<f64>) -> glm::TVec4<f64> {
    glm::vec4(
        f64::from(SCREEN_WIDTH) * (v.x + v.w) / 2.0,
        f64::from(SCREEN_HEIGHT) * (v.w - v.y) / 2.0,
        v.z,
        v.w,
    )
}

fn initialize_scene_objects(objects: &mut Vec<glm::TMat4<f64>>) {
    let identity: glm::TMat4<f64> = glm::identity();

    let mut m0 = glm::translate(&identity, &glm::vec3(0.0, 0.0, 2.0));
    m0 = glm::rotate(&m0, 45.0, &glm::vec3(0.0, 1.0, 0.0));

    let mut m1 = glm::translate(&identity, &glm::vec3(-3.75, 0.0, 0.0));
    m1 = glm::rotate(&m1, 30.0, &glm::vec3(1.0, 0.0, 0.0));

    let mut m2 = glm::translate(&identity, &glm::vec3(3.75, 0.0, 0.0));
    m2 = glm::rotate(&m2, 60.0, &glm::vec3(0.0, 1.0, 0.0));

    let mut m3 = glm::translate(&identity, &glm::vec3(0.0, 0.0, -2.0));
    m3 = glm::rotate(&m3, 90.0, &glm::vec3(0.0, 0.0, 1.0));

    objects.push(m0);
    objects.push(m1);
    objects.push(m2);
    objects.push(m3);
}
