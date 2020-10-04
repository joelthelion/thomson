extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{UnitQuaternion, Vector3, Translation3};
use rand::prelude::*;

const SPHERE_SIZE : f32 = 30.;

fn rnd_vec() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5)
}

fn relax(cubes: &mut Vec<SceneNode>) {
    for i in 0..cubes.len() {
        let x = cubes[i].data().local_translation().vector;
        let mut delta : Vector3<f32> = Vector3::zeros();
        for j in 0..cubes.len() {
            let y = cubes[j].data().local_translation().vector;
            let dist = (x-y).norm();
            if dist != 0. {
                delta += (0.1*(x-y)) / (dist*dist);
            }
        }
        // println!("{}: {}", i, delta);
        let mut new_point = x+delta;
        new_point.unscale_mut(new_point.norm() / SPHERE_SIZE);
        delta = new_point - x;
        cubes[i].append_translation(&Translation3::from(delta));
    }
}

struct AppState {
    c: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let mut window = Window::new("Thomson problem");
    window.set_background_color(0.05, 0.05, 0.05);
    window.set_framerate_limit(Some(60));
    let mut cube_group = window.add_group();

    let mut cubes : Vec<SceneNode> = Vec::new();
    for i in 0..100 {
        let mut c = cube_group.add_cube(1.0, 1.0, 1.0);
        if i%2==0 {
            c.set_color(1.0, 0.0, 0.0);
        } else {
            c.set_color(1.0, 1.0, 1.0);
        }
        let mut pos = rnd_vec();
        pos.unscale_mut(pos.norm());
        pos.scale_mut(SPHERE_SIZE);
        c.set_local_translation(Translation3::from(pos));
        cubes.push(c);
    }

    window.set_light(Light::StickToCamera);

    let small_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.004);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -0.014);

    while window.render() {
        cube_group.prepend_to_local_rotation(&small_rot);
        for c in cubes.iter_mut() {
            c.prepend_to_local_rotation(&rot);
        }
        relax(&mut cubes);
    }

}
