mod crearUsuario;

struct usuario {
    nombre : String,
    apellido : String,
    email : String
}

fn main() {
    
    let mut usuario_n1_01 = crearUsuario::crear_usuatio(
        "pepiito".to_string(),
        "ppipipi".to_string(),
        "pepito333@gmail.com".to_string()
    );


    let mut holapepito : i32 = 1958;

    println!("{:p}", &holapepito);


}


