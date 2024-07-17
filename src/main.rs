mod color;
mod framebuffer;
mod bmp;
mod line_impl;
mod vertex;

use framebuffer::Framebuffer;
use line_impl::Line;
use crate::vertex::Vertex;
use nalgebra_glm::{Vec3};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Usa Vec3 para definir los vértices
    let vertices = [
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    framebuffer.draw_polygon(&vertices);
    framebuffer.fill_polygon(&vertices);
    framebuffer.render_buffer("filled_polygonPR.bmp");
    
}
