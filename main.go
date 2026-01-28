package main
import (
  "fmt"
  "math"

)



type Shape interface{
  Area() float64
  Perimeter() float64
  Info()
  
}

type Rectangle struct{
  width float64
  height float64
}

func (r Rectangle) Area() float64{
  return r.width * r.height
}
func (r Rectangle) Perimeter() float64{
  return 2 * (r.width + r.height)
}
func (r Rectangle) Info(){
  fmt.Printf("Rectangle width:%.2f Height:%.2f Area: %.2f Perimeter:%.2f \n",r.width,r.height,r.Area(),r.Perimeter())
}

type Circle struct{
  
  radius float64
  
}

func (c Circle) Area() float64{
  return math.Pi * c.radius * c.radius 
}
func (c Circle) Perimeter() float64{
  return 2 * math.Pi * c.radius
}
func (c Circle) Info() {
  fmt.Printf("Circle radius:%.2f Area : %.2f Perimeter : %.2f\n",c.radius,c.Area(),c.Perimeter())
}

func main() {
	r:=Rectangle{5,5}
	r.Info()
	
	c := Circle{5}
	c.Info()
}