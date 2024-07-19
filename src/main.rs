mod framebuffer;
mod line;
mod bmp;

use framebuffer::Framebuffer;
use line::Line;

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, 0x000000); // Fondo negro

    // Limpiar el framebuffer con un fondo negro
    framebuffer.clear();

    // Definir los puntos del Polígono 1
    let polygon1_points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), 
        (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    // Dibujar el Polígono 1 (amarillo con orilla blanca)
    framebuffer.draw_polygon(
        &polygon1_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0xFFFF00  // Color de relleno (amarillo)
    );

    // Definir los puntos del Polígono 2
    let polygon2_points = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    // Dibujar el Polígono 2 (azul con orilla blanca)
    framebuffer.draw_polygon(
        &polygon2_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0x0000FF  // Color de relleno (azul)
    );

    // Definir los puntos del Polígono 3
    let polygon3_points = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    // Dibujar el Polígono 3 (rojo con orilla blanca)
    framebuffer.draw_polygon(
        &polygon3_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0xFF0000  // Color de relleno (rojo)
    );

    // Definir los puntos del Polígono 4
    let polygon4_points = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180)
    ];

    // Dibujar el Polígono 4 (verde con orilla blanca)
    framebuffer.draw_polygon(
        &polygon4_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0x00FF00  // Color de relleno (verde)
    );

    // Definir los puntos del Polígono 5 (Agujero dentro del Polígono 4)
    let polygon5_points = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    // Dibujar el Polígono 5 (agujero negro)
    framebuffer.draw_polygon(
        &polygon5_points, 
        0xFFFFFF, // Color de la orilla (blanco)
        0x000000  // Color de relleno (negro)
    );

    // Guardar el framebuffer como un archivo BMP
    framebuffer.render_buffer("out.bmp")?;

    println!("Framebuffer rendered to out.bmp");

    // Mostrar la imagen en una ventana
    framebuffer.display();

    Ok(())
}
