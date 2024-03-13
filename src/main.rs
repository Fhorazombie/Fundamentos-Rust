fn main() {
    /* 
    let nombre: &str = "Yoshua";
    let edad: u8 = 32;
    println!("Hola soy {} y tengo {} años", nombre, edad);

    let tempmax: u8 = 21;
    let tempmin: i8 = 6;
    println!("En Apizaco, Tlaxcala, la temperatura máxima es de {}°C y la mínima es de {}°C", tempmax, tempmin);
    */
    println!("Ingresa tu nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    
    println!("Ingresa tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Hola bienvenide {} de {} años", nombre, edad_int);

}
