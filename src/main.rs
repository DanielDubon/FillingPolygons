mod color;
mod framebuffer;
mod bmp;
mod line_impl;
mod vertex;

use framebuffer::Framebuffer;
use line_impl::Line;
use crate::vertex::Vertex;
use nalgebra_glm::{Vec3};
use crate::color::Color;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(0x0000000);
    framebuffer.clear();

    // Usa Vec3 para definir los vértices
    let vertices_polygon_1 = [
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

   
     let vertices_polygon_2 = [
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];


    let vertices_polygon_3 = [
    Vec3::new(377.0, 249.0, 0.0),
    Vec3::new(411.0, 197.0, 0.0),
    Vec3::new(436.0, 249.0, 0.0),
];

let vertices_polygon_4 = [
    Vec3::new(413.0, 177.0, 0.0),
    Vec3::new(448.0, 159.0, 0.0),
    Vec3::new(502.0, 88.0, 0.0),
    Vec3::new(553.0, 53.0, 0.0),
    Vec3::new(535.0, 36.0, 0.0),
    Vec3::new(676.0, 37.0, 0.0),
    Vec3::new(660.0, 52.0, 0.0),
    Vec3::new(750.0, 145.0, 0.0),
    Vec3::new(761.0, 179.0, 0.0),
    Vec3::new(672.0, 192.0, 0.0),
    Vec3::new(659.0, 214.0, 0.0),
    Vec3::new(615.0, 214.0, 0.0),
    Vec3::new(632.0, 230.0, 0.0),
    Vec3::new(580.0, 230.0, 0.0),
    Vec3::new(597.0, 215.0, 0.0),
    Vec3::new(552.0, 214.0, 0.0),
    Vec3::new(517.0, 144.0, 0.0),
    Vec3::new(466.0, 180.0, 0.0)
];

let vertices_polygon_5 = [
    Vec3::new(682.0, 175.0, 0.0),
    Vec3::new(708.0, 120.0, 0.0),
    Vec3::new(735.0, 148.0, 0.0),
    Vec3::new(739.0, 170.0, 0.0)
];

    // Definir colores
    let yellow = Color::new(255, 255, 0);
    let white = Color::new(255, 255, 255);
    let blue = Color::new(0, 0, 255);
    let red = Color::new(255, 0, 0);
    let black = Color::new(0, 0, 0);
    let green = Color::new(0, 255, 0);


    framebuffer.fill_polygon(&vertices_polygon_1, yellow);
    framebuffer.draw_polygon(&vertices_polygon_1, white);

    framebuffer.fill_polygon(&vertices_polygon_2, blue);
    framebuffer.draw_polygon(&vertices_polygon_2, white);

    framebuffer.fill_polygon(&vertices_polygon_3, red);
    framebuffer.draw_polygon(&vertices_polygon_3, white);


    framebuffer.fill_polygon(&vertices_polygon_4, green);
    framebuffer.draw_polygon(&vertices_polygon_4, white);

    framebuffer.fill_polygon(&vertices_polygon_5, black);
    framebuffer.draw_polygon(&vertices_polygon_5, white);

    framebuffer.render_buffer("out.bmp").unwrap();
    
}
