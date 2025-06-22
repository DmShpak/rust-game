            
use macroquad::prelude::*;
use vector::vector::Vector;


fn render_vector(v: &Vector, color: &Color) {
    draw_circle(v.1, v.0, 10., color.clone());
    
}
#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        let (x, y) =mouse_position();

        draw_text("Example: Vector", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("GREEN + BLUE = RED", 20.0, 40.0, 30.0, DARKGRAY);

        let a = Vector(60., 100.);
        let b = Vector(y, x);
        let c = a.clone_add(&b);
        render_vector(&a, &GREEN);
        render_vector(&b, &BLUE);
        render_vector(&c, &RED);

        next_frame().await
    }
}
          