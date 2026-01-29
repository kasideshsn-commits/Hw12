use std::f64::consts::PI;

trait Shape{
    fn area(&self) ->f64;
    fn perimeter(&self) -> f64;
    fn info(&self);
    
}

struct Rectangle{
    
    width :f64,
    height :f64,
    
}
impl Rectangle{
    fn new(width:f64, height:f64) -> Self{
        Rectangle{width , height}
    }
}
impl Shape for Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64{
        2.0 * (self.width+self.height)
    }
    fn info(&self){
        println!("Rectangle width:{} ,Heigth: {}",self.width,self.height);
        println!("Area:{} ,Perimeter:{}",self.area(),self.perimeter());
    }
}



struct Circle{
    radius :f64,
}

impl Circle{
    fn new(radius:f64)->Self{
        Circle{radius}
    }
}
impl Shape for Circle{
    fn area(&self)->f64{
        PI * self.radius * self.radius 
    }
    
    fn perimeter(&self) -> f64{
        2.0 * PI * self.radius
    }
    fn info(&self){
        println!("Circle radius is {:.2}",self.radius);
        println!("Area is :{:.2} Perimeter is {:.2}",self.area(),self.perimeter())
    }
    
    
}


fn main() {
    let r1 = Rectangle::new(5.0,5.0);
    r1.info();
    
    let c1 = Circle::new(5.0);
    c1.info();
    
}
