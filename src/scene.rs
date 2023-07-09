use std::sync::Arc;

use crate::camera::Camera;
use crate::hitable::hitable_list::HitableList;
use crate::hitable::sphere::Sphere;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::utils::color::Color;
use crate::utils::{obj_import_as_triangles, random_double};
use crate::vec3::Vec3;
use crate::{hitable, texture};
use hitable::moving_sphere::MovingSphere;
use hitable::xy_rectangle::XYRectangle;
use hitable::xz_rectangle::XZRectangle;
use hitable::yz_rectangle::YZRectangle;
use texture::checker_texture::CheckerTexture;

pub fn random_scene() -> (HitableList, Camera, Color, f64) {
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = HitableList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_double(None);
            let center = Vec3::new(
                a + 0.9 * random_double(None),
                0.2,
                b + 0.9 * random_double(None),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(None) * Color::random(None);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(Some((0.5, 1.0)));
                    let fuzz = random_double(Some((0.0, 0.5)));
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    // Main scene
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; //(look_from - look_at).length();
    let appreture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn random_moving_scene() -> (HitableList, Camera, Color, f64) {
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = HitableList::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_double(None);
            let center = Vec3::new(
                a + 0.9 * random_double(None),
                0.2,
                b + 0.9 * random_double(None),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(None) * Color::random(None);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(random_double(Some((0.0, 0.5))), 0.0, 0.0);
                    world.add(MovingSphere::new(
                        (center, center2),
                        (0.0, 1.0),
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(Some((0.5, 1.0)));
                    let fuzz = random_double(Some((0.0, 0.5)));
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    // Main scene
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(1.0, 0.1, 0.3)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; //(look_from - look_at).length();
    let appreture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        Some((0.0, 1.0)),
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn checker_scene() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let checker_texture = Arc::new(CheckerTexture::new(
        Color::new(0.0, 0.0, 0.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let material_ground = Arc::new(Lambertian::new_from_texture(checker_texture));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn checker_emmisive_material_scene() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let checker_texture = Arc::new(CheckerTexture::new(
        Color::new(0.0, 0.0, 0.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let diffuse_light = Arc::new(DiffuseLight::new(Color::new(10.0, 10.0, 10.0)));

    let material_ground = Arc::new(Lambertian::new_from_texture(checker_texture));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Add emmisive sphere
    world.add(Sphere::new(Vec3::new(2.0, 2.0, -1.0), 0.5, diffuse_light));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(0.0, 0.0, 0.0);
    (world, cam, background, aspect_ratio)
}

pub fn scene1() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));

    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left));

    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 1.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn scene2() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn scene3() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 1.0)));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_ground));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        70.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn scene4() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 1.0)));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(1.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.2;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(1.0, 1.0, 1.0);
    (world, cam, background, aspect_ratio)
}

pub fn rectangular_light_scene() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 1.0)));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Add emmisive box
    let diffuse_light = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(XYRectangle::new(
        (3.0, 5.0),
        (1.0, 3.0),
        -2.0,
        diffuse_light,
    ));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(1.0, 0.5, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.2;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(0.0, 0.0, 0.0);
    (world, cam, background, aspect_ratio)
}

pub fn cornell_box_scene() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let glass = Arc::new(Dielectric::new(1.5));
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let metal = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Empty Cornell Box
    world.add(YZRectangle::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.add(YZRectangle::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.add(XZRectangle::new(
        (213.0, 343.0),
        (227.0, 332.0),
        554.0,
        light,
    ));
    world.add(XZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    ));
    world.add(XZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    world.add(XYRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));

    // Instances
    world.add(Sphere::new(Vec3::new(350.0, 100.0, 165.0), 100.0, glass));
    world.add(Sphere::new(Vec3::new(150.0, 100.0, 265.0), 100.0, metal));

    let aspect_ratio = 1.0;
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.2;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        35.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(0.0, 0.0, 0.0);
    (world, cam, background, aspect_ratio)
}

pub fn teapot_scene() -> (HitableList, Camera, Color, f64) {
    let mut world = HitableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 1.0)));
    let material_right = Arc::new(Metal::new(Color::new(1.0, 0.2, 0.3), 0.1));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let diffuse_light = Arc::new(DiffuseLight::new(Color::new(10.0, 10.0, 10.0)));

    // world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    // world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));

    // Add teapot triangles
    world = obj_import_as_triangles("models/teapot.obj", white);

    world.add(Sphere::new(Vec3::new(-2.0, 5.0, -1.0), 0.5, diffuse_light));

    // Add ground sphere
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground.clone(),
    ));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(0.0, 4.0, -6.0);
    let look_at = Vec3::new(0.0, 0.5, 1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let appreture = 0.05;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        60.0,
        aspect_ratio,
        appreture,
        dist_to_focus,
        None,
    );

    let background = Color::new(0.0, 0.0, 0.0);
    (world, cam, background, aspect_ratio)
}
