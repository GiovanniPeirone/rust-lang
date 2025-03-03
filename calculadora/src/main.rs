fn main() {


    let  opcion :i32 = 3;
    if opcion ==1  {
        
        println!("{}", sumar(3,4));
    }else if  opcion==2 {
        
        println!("{}", resta(2,1));
    }else if opcion== 3 {
        println!("{}",multiplicacion(2, 1));
        
    }else {
        
        println!("{}", divicion(3, 2))
    }
    

    
}
fn sumar(n1:i32,n2:i32) -> i32 {
    let resultado: i32 =n1 +n2;

    return resultado;
}
fn resta(n1:i32,n2:i32) -> i32 {
    let resultado2: i32 =n1 - n2;

    return resultado2;
}
fn multiplicacion (n1:i32,n2:i32) -> i32 {
    let resultado2: i32 =n1 * n2;

    return resultado2;
}
fn divicion(n1:i32,n2:i32) -> i32 {
    let resultado2: i32 =n1 / n2;

    return resultado2;
    
}


