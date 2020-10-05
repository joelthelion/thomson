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
const SPHERES : usize = 79;
const SPHERE_SIZE : f32 = 0.9;

struct Node {
    node : SceneNode,
    weight : f32
}

fn rnd_vec() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5)
}

fn relax(nodes: &mut Vec<Node>, brown:f32) {
    for i in 0..nodes.len() {
        let x = nodes[i].node.data().local_translation().vector;
        // let mut delta : Vector3<f32> = Vector3::zeros();
        let mut delta : Vector3<f32> = rnd_vec()*brown;
        delta += 0.5 *(BIG_SPHERE-x.norm()) * x / x.norm();
        // delta += 0.2 *(-x.norm()) * x / x.norm();
        for j in 0..nodes.len() {
            let y = nodes[j].node.data().local_translation().vector;
            let dist = (x-y).norm();
            if dist != 0. {
                // delta += (0.02*(x-y)) * nodes[i].weight * nodes[j].weight / (dist*dist);
                delta += (0.05*(x-y)) * nodes[j].weight / (dist*dist);
            }
        }
        // println!("{}: {}", i, delta);
        // let mut new_point = x+delta;
        // new_point.unscale_mut(new_point.norm() / BIG_SPHERE);
        // delta = new_point - x;
        nodes[i].node.append_translation(&Translation3::from(delta));
    }
}

fn write_scad(nodes: &Vec<Node>) {
    let mut file = File::create("spheres.scad").unwrap();
    writeln!(file, "difference() {{ union() {{").unwrap();
    for i in 0..nodes.len() {
        let x = nodes[i].node.data().local_translation().vector;
        writeln!(file, "translate([{},{},{}]) {{sphere({},$fn=30);}};", x[0], x[1], x[2], SPHERE_SIZE*nodes[i].weight.powf(0.33)).unwrap();
    }
    writeln!(file, "}}; union() {{ ").unwrap();
    for i in 0..nodes.len() {
        let x = nodes[i].node.data().local_translation().vector;
        writeln!(file, "translate([{},{},{}]) {{sphere({},$fn=30);}};", x[0], x[1], x[2], SPHERE_SIZE*nodes[i].weight.powf(0.33)-0.2).unwrap();
    }
    writeln!(file, "}}; }};").unwrap();
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
    let factor = 0.5 * f32::exp(-(i as f32) * 0.00003);
    if factor > 0.0003 {
        factor }
    else { 0. }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut window = Window::new("Thomson problem");
    window.set_background_color(0.05, 0.05, 0.05);
    window.set_framerate_limit(Some(60));
    let mut node_group = window.add_group();

    let mut nodes : Vec<Node> = Vec::new();
    for _ in 0..SPHERES {
        // let weight : f32 = if i<12  { 10. } else { 1. };
        let weight : f32 = 1. + 3. * rng.gen::<f32>();
        let mut c = node_group.add_sphere(SPHERE_SIZE*weight.powf(0.33));
        c.set_color(1.0, 0.0, 0.0);
        let mut pos = rnd_vec();
        pos.unscale_mut(pos.norm());
        pos.scale_mut(BIG_SPHERE);
        c.set_local_translation(Translation3::from(pos));
        nodes.push(Node{ node : c, weight : weight });
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
        if i>0 && i % 100_000 == 0 {
            write_scad(&nodes);
            println!("Wrote to disk!");
        }
    }
}
