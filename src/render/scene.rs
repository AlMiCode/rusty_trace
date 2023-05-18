use cgmath::{point3, vec3};
use serde::{Serialize, Deserialize};

use crate::vec_repo::{Id, VecRepo};

use super::{
    camera::CameraSettings,
    hittable::{rect::Rect, sphere::Sphere, HittableVec, modifiers::{Translate, RotateY}},
    material::{Dielectric, DiffuseLight, Lambertian, Material},
    texture::Texture,
    Colour,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Scene {
    pub hittable: HittableVec,
    pub camera: CameraSettings,
    pub background: Id<Texture>,
    pub materials: VecRepo<Material>,
    pub textures: VecRepo<Texture>,
}

impl Default for Scene {
    fn default() -> Self {
        let background: Texture = Colour::new(0.1, 0.65, 0.9).into();
        let mut textures = VecRepo::<Texture>::default();
        let id = textures.insert(background);
        Self {
            hittable: HittableVec::new(),
            camera: CameraSettings::default(),
            background: id,
            materials: Default::default(),
            textures,
        }
    }
}

impl Scene {
    pub fn cornell_box() -> Self {
        // Use this for testing.
        let mut textures = VecRepo::<Texture>::default();

        let red_tex = textures.insert(Colour::new(0.65, 0.05, 0.05));
        let white_tex = textures.insert(Colour::new(0.73, 0.73, 0.73));
        let green_tex = textures.insert(Colour::new(0.12, 0.45, 0.15));
        let light_tex = textures.insert(Colour::new(1.0, 1.0, 1.0));
        let black_tex = textures.insert(Colour::new(0., 0., 0.));

        let mut materials = VecRepo::<Material>::default();

        let red_mat = materials.insert(Lambertian { albedo: red_tex });
        let white_mat = materials.insert(Lambertian { albedo: white_tex });
        let green_mat = materials.insert(Lambertian { albedo: green_tex });
        let light_mat = materials.insert(DiffuseLight {
            emit: light_tex,
            amplify: 15.0,
        });

        let glass_mat = materials.insert(Dielectric {
            refractive_index: 1.5,
        });

        let point000 = point3(0.0, 0.0, 0.0);

        let point500 = point3(555.0, 0.0, 0.0);
        let point050 = point3(0.0, 555.0, 0.0);
        let point005 = point3(0.0, 0.0, 555.0);

        let point505 = point3(555.0, 0.0, 555.0);
        let point055 = point3(0.0, 555.0, 555.0);

        let point555 = point3(555.0, 555.0, 555.0);

        let green_wall = Rect::new(point500, point555, green_mat);
        let red_wall = Rect::new(point000, point055, red_mat);
        let floor = Rect::new(point000, point505, white_mat);
        let ceiling = Rect::new(point050, point555, white_mat);
        let back_wall = Rect::new(point005, point555, white_mat);
        let light_source = Rect::new(
            point3(213.0, 554.0, 227.0),
            point3(343.0, 554.0, 322.0),
            light_mat,
        );

        let ball = Sphere::new(point3(400.0, 100.0, 80.0), 100.0, glass_mat);

        let box1 = Rect::new(point000, point3(165.0, 330.0, 165.0), white_mat);
        let box1 = RotateY::new(Box::new(box1.into()), 15.0);
        let box1 = Translate::new(Box::new(box1.into()), vec3(265.0, 0.0, 295.0));

        let box2 = Rect::new(point000, point3(165.0, 165.0, 165.0), white_mat);
        let box2 = RotateY::new(Box::new(box2.into()), -18.0);
        let box2 = Translate::new(Box::new(box2.into()), vec3(130.0, 0.0, 65.0));

        let camera_set = CameraSettings {
            look_from: point3(278.0, 278.0, -800.0),
            look_at: point3(278.0, 278.0, 0.0),
            fov: 40.0,
            aperture: 1.0 / 16.0,
            ..Default::default()
        };
        let mut scene = Self {
            hittable: vec![
                green_wall.into(),
                red_wall.into(),
                floor.into(),
                ceiling.into(),
                back_wall.into(),
                light_source.into(),
                ball.into(),
                box1.into(),
                box2.into(),
            ],
            camera: camera_set,
            background: black_tex,
            materials,
            textures: textures.into(),
        };
        let serialized = rmp_serde::to_vec(&scene).unwrap();
        scene = rmp_serde::from_slice(&serialized).unwrap();
        scene
    }
}
