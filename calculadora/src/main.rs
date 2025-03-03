mod multiplicacion;
mod divicion;
mod resta;
mod suma;
fn main() {


    let  opcion :i32 = 3;
    if opcion ==1  {
        
        println!("{}", suma::sumar(1, 2));
    }else if  opcion==2 {
        
        println!("{}", resta::resta(1, 2));
    }else if opcion== 3 {
        println!("{}",multiplicacion::multiplicacion(1, 2));
        
    }else {
        
        println!("{}", divicion::divicion(1, 2));
    }
    
    
    
    
    

    
}


