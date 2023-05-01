use egui::Ui;

use crate::render::hittable::sphere::Sphere;

use super::point3_editor;

fn sphere_editor(ui: &mut Ui, sphere: &mut Sphere) {
    ui.label("Center: ");
    point3_editor(ui, &mut sphere.center);

    ui.label("Radius: ");
    ui.add(egui::DragValue::new(&mut sphere.radius).speed(0.5));
}
