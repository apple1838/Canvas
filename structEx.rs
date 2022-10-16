struct Point( f32, f32 );
struct Points {
    p1:Point,
    p2:Point,
    p3:Point,
    p4:Point,
}

fn rect_area() -> f32 {
    let mut width:f32 = Vec::new();
    let mut height:f32 = Vec::new();
    std::io::stdin().read_line(&mut width).unwrap();
    std::io::stdin().read_line(&mut hegith).unwrap();
    
    width * heigth
}


fn main(){
    let a = rect_area();
    println!("{}",a);
}
