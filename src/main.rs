fn main(){


/*
    Desarrollar formulario para pedir datos al usuario a través de una interfaz gráfica.
    El formulario debe tener una serie de campos de texto, un campo de texto para la contraseña y un botón de envío.
    El formulario debe validar que los datos sean correctos y mostrar un mensaje de error en caso contrario.
*/
    //Formulario
    println!("Por favor introduce tu nombre: ");
    let mut nombre: String = String ::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    


    //Obtener la edad de la consola
    println!("Por favor introduce tu edad: ");
    let mut edad: String = String ::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    edad = edad.trim().to_string();

    //Convertir esa edad en un número entero
    let edad_int: u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int <= 100 {
         
        loop{
        //login del usuario
        println!("Por favor introduce tu contraseña: ");
        let mut contraseña: String = String ::new();
        std::io::stdin().read_line(&mut contraseña).unwrap();
        contraseña = contraseña.trim().to_string();

        print!("¿OLvidaste tu contraseña? (s/n): ");

        let mut respuesta: String = String ::new();
        std::io::stdin().read_line(&mut respuesta).unwrap();
        respuesta = respuesta.trim().to_string();
        if respuesta == "s" {
            println!("Por favor introduce tu contraseña: ");
            let mut contraseña: String = String ::new();
            std::io::stdin().read_line(&mut contraseña).unwrap();
            contraseña = contraseña.trim().to_string();
        }
        else if respuesta == "n" {
            println!("Bienvenido {}", nombre);
            break;
        }
        else {
            println!("Por favor introduce tu contraseña: ");
            let mut contraseña: String = String ::new();
            std::io::stdin().read_line(&mut contraseña).unwrap();
            contraseña = contraseña.trim().to_string();
        }

        //Comprobar que la contraseña es correcta
        if contraseña == "12345" {
            println!("Bienvenido {}", nombre);
            break;
        } else {
            println!("La contraseña es incorrecta"); 
    }
    //Obtener el pais de la consola
        println!("Por favor introduce tu pais: ");
        let mut pais: String = String ::new();
        std::io::stdin().read_line(&mut pais).unwrap();
        pais = pais.trim().to_string();

    //Obtener el idioma de la consola
        println!("Por favor introduce tu idioma: ");
        let mut idioma: String = String ::new();
        std::io::stdin().read_line(&mut idioma).unwrap();
        idioma = idioma.trim().to_string();
         if edad_int >= 65 {
            println!("Puedes pasar sin tarjeta");
        }
        println!("Hola Bienvenido(a) de nuevo {} tienes {} años vives en {} y hablas {}", nombre, edad_int, pais, idioma)
    
    } else {
        println!("No puedes pasar");
    }
    }
    
}