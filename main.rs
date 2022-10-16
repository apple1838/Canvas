use rand::Rng;

fn rect_area() -> f32{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s:f32 = s.trim().parse().unwrap();
    s
}

fn sum_area() -> f32 {
    println!("standard sqaure height :");
    let h:f32 = rect_area();
    println!("standard square width :");
    let w:f32 = rect_area();
    h * w
}//Square area

fn input_point() -> (f32, f32){
    println!("x :");
    let x = rect_area();
    println!("y :");
    let y = rect_area();
    println!("point is ({}, {})",x,y);
    (x, y)
}//Make point

#[derive(Debug)]
struct Point {
    p1 : (f32, f32),
    p2 : (f32, f32),
    p3 : (f32, f32),
    p4 : (f32, f32),
}
fn make_square( sq: f32) -> (f32, f32){
    let height = rand::thread_rng().gen_range(1.0, 1001.0);
    let width = sq / height;
    (height, width)
}


fn main(){
    let square:f32 = sum_area(); 
    println!("{}",square);
    let p1 = input_point();

    let new_square = make_square(square);
    let mut point = Point {
        p1 : p1,
        p2 : (p1.0, p1.1 + new_square.1),
        p3 : (p1.0 + new_square.0, p1.1 + new_square.1),
        p4 : (p1.0 + new_square.0, p1.1),
    };
    println!("{:?}",point);
}

