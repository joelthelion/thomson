extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{UnitQuaternion, Vector3, Translation3};
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

const BIG_SPHERE : f32 = 2.;
const SPHERE_SIZE : f32 = 0.5;

fn rnd_vec() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5)
}

fn relax(nodes: &mut Vec<SceneNode>, brown:f32) {
    for i in 0..nodes.len() {
        let x = nodes[i].data().local_translation().vector;
        // let mut delta : Vector3<f32> = Vector3::zeros();
        let mut delta : Vector3<f32> = rnd_vec()*brown;
        for j in 0..nodes.len() {
            let y = nodes[j].data().local_translation().vector;
            let dist = (x-y).norm();
            if dist != 0. {
                delta += (0.5*(x-y)) / (dist*dist);
            }
        }
        // println!("{}: {}", i, delta);
        let mut new_point = x+delta;
        new_point.unscale_mut(new_point.norm() / BIG_SPHERE);
        delta = new_point - x;
        nodes[i].append_translation(&Translation3::from(delta));
    }
}

fn write_scad(nodes: &Vec<SceneNode>) {
    let mut file = File::create("spheres.scad").unwrap();
    for i in 0..nodes.len() {
        let x = nodes[i].data().local_translation().vector;
        writeln!(file, "translate([{},{},{}]) {{sphere({},$fn=100);}};", x[0], x[1], x[2], SPHERE_SIZE).unwrap();
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

fn relaxation_scheme(i:usize) -> f32 {
    let factor = 10. * f32::exp(-(i as f32) * 0.0005);
    if factor > 0.03 {
        factor }
    else { 0. }
}

fn main() {
    let mut window = Window::new("Thomson problem");
    window.set_background_color(0.05, 0.05, 0.05);
    window.set_framerate_limit(Some(60));
    let mut node_group = window.add_group();

    let mut nodes : Vec<SceneNode> = Vec::new();
    for i in 0..100 {
        let mut c = node_group.add_sphere(SPHERE_SIZE);
        if i%2>=0 {
            c.set_color(1.0, 0.0, 0.0);
        } else {
            c.set_color(1.0, 1.0, 1.0);
        }
        let mut pos = rnd_vec();
        pos.unscale_mut(pos.norm());
        pos.scale_mut(BIG_SPHERE);
        c.set_local_translation(Translation3::from(pos));
        nodes.push(c);
    }

    window.set_light(Light::StickToCamera);

    let _small_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.004);
    let _rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -0.014);

    let mut i = 0;
    while window.render() {
        // node_group.prepend_to_local_rotation(&small_rot);
        // for c in nodes.iter_mut() {
            // c.prepend_to_local_rotation(&rot);
        // }
        for _ in 0..100 {
            relax(&mut nodes, relaxation_scheme(i));
            i+=1;
        }
        println!("{}", i);
        if i>30000 {
            break;
        }
    }
    write_scad(&nodes);

}
