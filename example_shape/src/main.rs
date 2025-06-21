            
use macroquad::prelude::*;
use shape::segment::Segment;
use vector::vector::Vector;


fn render_vector(v: &Vector, color: &Color) {
    draw_circle(v.0, v.1, 10., color.clone());
}


fn render_segment(s: &Segment, color: &Color) {
    let end = s.end();

    draw_line(s.location.0 , s.location.1, end.0, end.1, 2., color.clone());
    draw_circle(s.location.0 , s.location.1, 10., color.clone());
    
    draw_circle(end.0, end.1, 10., color.clone());
}

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Example: Shape", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Segment", 20.0, 40.0, 30.0, DARKGRAY);

        let s = Segment{
            location: Vector(60., 60.),
            vector: Vector(200., 300.),
        };
        let a = Vector(200., 150.);
        let p = s.project_point(&a);

        render_segment(&s, &BLUE);
        render_vector(&a, &GREEN);
        render_vector(&p, &RED);

        next_frame().await
    }
}
          