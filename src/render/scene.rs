use cgmath::point3;

use crate::vec_repo::{Id, VecRepo};

use super::{
    camera::CameraSettings,
    hittable::{
        plane::{Plane, Rect},
        HittableVec, sphere::Sphere,
    },
    material::{DiffuseLight, Lambertian, Material, Dielectric},
    texture::Texture,
    Colour,
};

#[derive(Clone)]
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

        let glass_mat = materials.insert(Dielectric { refractive_index: 1.5 });

        let green_wall = Rect::new(&point3(555.0, 0.0, 0.0), 555.0, 555.0, Plane::YZ, green_mat);
        let red_wall = Rect::new(&point3(0.0, 0.0, 0.0), 555.0, 555.0, Plane::YZ, red_mat);
        let floor = Rect::new(&point3(0.0, 0.0, 0.0), 555.0, 555.0, Plane::XZ, white_mat);
        let ceiling = Rect::new(&point3(0.0, 555.0, 0.0), 555.0, 555.0, Plane::XZ, white_mat);
        let back_wall = Rect::new(&point3(0.0, 0.0, 555.0), 555.0, 555.0, Plane::XY, white_mat);
        let light_source = Rect::new(
            &point3(213.0, 554.0, 227.0),
            130.0,
            105.0,
            Plane::XZ,
            light_mat,
        );

        let ball = Sphere::new(point3(150.0, 110.0, 150.0), 110.0, glass_mat);

        let camera_set = CameraSettings {
            look_from: point3(278.0, 278.0, -800.0),
            look_at: point3(278.0, 278.0, 0.0),
            fov: 40.0,
            aperture: 1.0 / 16.0,
            ..Default::default()
        };
        Self {
            hittable: vec![
                Box::new(green_wall),
                Box::new(red_wall),
                Box::new(floor),
                Box::new(ceiling),
                Box::new(back_wall),
                Box::new(light_source),
                Box::new(ball)
            ],
            camera: camera_set,
            background: black_tex,
            materials,
            textures: textures.into(),
        }
    }
}
