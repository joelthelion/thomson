difference() {
union() {
    include<spheres.scad>;
};
translate([0,0,10]) cube(20,true);
sphere(4.5,$fn=30);
}
