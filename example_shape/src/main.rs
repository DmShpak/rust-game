            
use macroquad::prelude::*;
use shape::{circle::Circle, segment::Segment, shape::Shape};
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

pub fn shape_project_point(x:f32, y:f32) {
        draw_text("Example: Shape", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Segment", 20.0, 40.0, 30.0, DARKGRAY);

        let s = Segment{
            location: Vector(60., 60.),
            vector: Vector(200., 300.),
        };

        let a = Vector(x, y);
        let p = s.project_point(&a);

        render_segment(&s, &BLUE);
        render_vector(&a, &GREEN);
        render_vector(&p, &RED);
}


fn render_shape(s:&Shape, color: &Color) {
    match s {
        Shape::Dot(d) => {
            draw_circle(d.0 , d.1, 3., color.clone());
        }
        Shape::Circle(c) => {
            draw_circle_lines(c.location.0 , c.location.1, c.radius, 2., color.clone());
        }
        _ => {}
    }
}


pub fn shape_corcles_collision(x:f32, y:f32) {

        draw_text("Example: Shape", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Circles collision", 20.0, 40.0, 30.0, DARKGRAY);

        let c1 = Shape::Circle(Circle{
            location:Vector(120., 120.),
            radius: 50.
        });

        let c2 = Shape::Circle(Circle{
            location:Vector(x, y),
            radius: 80.
        });


        render_shape(&c1, &GREEN);
        render_shape(&c2, &BLUE);

        let r = c1.collision_with(&c2);

        match r {
            Some(p) => {
                render_vector(&p, &RED);
            }
            _ => {}
        }

}

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        let (x, y) = mouse_position();
        // shape_project_point(x,y);
        shape_corcles_collision(x,y);
        next_frame().await
    }
}
          