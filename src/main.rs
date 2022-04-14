use std::any::Any;

//abtract parent
trait GetName {
    fn get_name(self: &Self) -> &'static str;
}
trait Rectangle<T> {
    fn cal_area(self: &Self) -> f64;
} 
impl<T> GetName for dyn Rectangle<T> {
    fn get_name(&self) ->  &'static str { std::any::type_name::<T>().split("::").last().unwrap() }
}

//abstract function
impl<T> std::fmt::Display for Rectangle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "The area of the {} is {}",self.get_name(), self.cal_area()) }
}
//child
struct Circle {
    r: u32
}
impl Rectangle<Circle> for Circle {
    fn cal_area(self: &Self) -> f64 { f64::from(self.r) * 3.14 }
}

//child
struct Square {
   pub x : u32
}
impl Square {
    fn update_x(&mut self, x: u32) {
        self.x += u32::from(x);
    }
}

impl Rectangle<Square> for Square {
    fn cal_area(self: &Self) -> f64 { f64::from(self.x) * f64::from(self.x) }
}



fn main (){
    let now = std::time::Instant::now();
    let mut s: Square =  Square { x: 19 };
    s.update_x(10);
    let c: Circle = Circle { r: 30 };
    println!("{}", c);
    let elapsed = now.elapsed().as_secs_f64();
    println!("Elapsed time {}s", elapsed);
}