import 'dart:math';
mixin Shape{
  double area();
  double perimeter();
  void info();
  
}

class Rectangle with Shape{
  double width;
  double height;
  
  Rectangle(this.width,this.height);
  
  @override
  double area() {
    return width * height ;
  }
  
  @override
  double perimeter() => 2 * (width+height);
  
  @override 
  void info(){
    print("Rectangle Width:$width, Height: $height");
    print("Area : ${area()}");
    print("Perimeter :${perimeter()}");
  }
}
class Circle with Shape{
  double radius;
  
  Circle(this.radius);
  
  
  @override
  double area() => pi * (radius*2) ;
  
  @override
  double perimeter() => 2 * pi * radius ;
  
  @override
  void info(){
    print("Radius : radius : $radius");
    print("Area : ${area()}");
    print("perimeter : ${perimeter()}");
  }
  
  
}





void main() {
  var r1 = Rectangle(5,5);
  r1.info();
  print(r1.area());
  print(r1.perimeter());
  
  
  print("==================================");
    
  var c1 = Circle(4);
  c1.info();
  print(c1.area());
  print(c1.perimeter());
}
