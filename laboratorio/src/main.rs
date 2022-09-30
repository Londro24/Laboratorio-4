use std::io::stdin;
use rand::Rng;

fn juego_jugador_1(mut tabla_jugador_1: [i32; 10], contador: i32,) -> [i32; 10] {
    let turno: usize = ((contador / 2 + contador % 2) - 1).try_into().unwrap();
    let mut suma: i32 = 0;
    let mut tiro = rand::thread_rng();
      
    
    let num_1: i32 = tiro.gen_range(0..=10);
    suma += num_1;
    let mut num_2: i32 = 0;
    if num_1 < 10 {
        num_2 = tiro.gen_range(0..=(10 - num_1));
        suma += num_2;
    }

    if turno != 0 && tabla_jugador_1[turno - 1] == 10  {
        tabla_jugador_1[turno - 1] += suma;
    }
    
    tabla_jugador_1[turno] = suma;

    if num_1 == 10 {
        println!("El primer jugador hizo una chuza");
        println!("Dejando su marcador en:{:?}", tabla_jugador_1);
    } else if suma == 10 && num_1 != 10 {
        println!("El primer jugador hizo un media chuza, siendo su primer tiro {} puntos y el segund0 {} puntos", num_1, num_2);
        println!("Dejando su marcador en:{:?}", tabla_jugador_1);
    } else {
        println!("El primer jugador en el primer lanzamiento hizo {} puntos, en el segundo hizo {} puntos", num_1, num_2);
        println!("Dejando su marcador en:{:?}", tabla_jugador_1);
    }

    return tabla_jugador_1
}

fn juego_jugador_2(mut tabla_jugador_2: [i32; 10], contador: i32) -> [i32; 10] {
    let new_contador: i32 = contador -1;
    let turno: usize = ((new_contador / 2 + new_contador % 2) - 1).try_into().unwrap();
    let mut suma: i32 = 0;
    let mut tiro = rand::thread_rng();
      
    
    let num_1: i32 = tiro.gen_range(0..=10);
    suma += num_1;
    let mut num_2: i32 = 0;
    if num_1 < 10 {
        num_2 = tiro.gen_range(0..=(10 - num_1));
        suma += num_2;
    }

    if turno != 0 && tabla_jugador_2[turno - 1] == 10  {
        tabla_jugador_2[turno - 1] += suma;
    }
    
    tabla_jugador_2[turno] = suma;

    if num_1 == 10 {
        println!("El segundo jugador hizo una chuza");
        println!("Dejando su marcador en:{:?}", tabla_jugador_2);
    } else if suma == 10 && num_1 != 10 {
        println!("El segundo jugador hizo un media chuza, siendo su primer tiro {} puntos y el segund0 {} puntos", num_1, num_2);
        println!("Dejando su marcador en:{:?}", tabla_jugador_2);
    } else {
        println!("El segundo jugador en el primer lanzamiento hizo {} puntos, en el segundo hizo {} puntos", num_1, num_2);
        println!("Dejando su marcador en:{:?}", tabla_jugador_2);
    }

    return tabla_jugador_2;
}

fn ganador(tabla_jugador_1: [i32; 10], tabla_jugador_2: [i32; 10], jugador1: String, jugador2: String) {
    let mut suma1: i32 = 0;
    let mut suma2: i32 = 0;

    for i in tabla_jugador_1.iter() {
        suma1 += i;
    }
    for i in tabla_jugador_2.iter() {
        suma2 += i;
    }

    if suma1 > suma2 {
        println!("Felicidades {}, le ganaste a {}", &jugador1.trim(), &jugador2.trim())
    } else {
        println!("Felicidades {}, le ganaste a {}", &jugador2.trim(), &jugador1.trim())
    }
    println!("\nEl marcador es:");
    println!("El jugador '{}' consiguio {} puntos", &jugador1.trim(), suma1);
    println!("El jugador '{}' consiguio {} puntos", &jugador2.trim(), suma2);
}

fn main() {
    println!("Bienvenido, porfavor ingrese el nombre del 1er y 2do jugador respectivamente");
    let mut jugador1: String = String::new();
    stdin().read_line(&mut jugador1).unwrap();
    let mut jugador2: String = String::new();
    stdin().read_line(&mut jugador2).unwrap();

    if jugador1.trim() == "" || jugador2.trim() == "" {
        panic!("Se requiere un nombre.")
    }
    println!("");

    let mut contador: i32 = 1;
    let mut _tiro1: i32 = 0;
    let mut _tiro2: i32 = 0;
    let mut tabla_jugador_1: [i32; 10] = [0; 10];
    let mut tabla_jugador_2: [i32; 10] = [0; 10];
    loop {
        if contador % 2 != 0 && contador < 21 {
            tabla_jugador_1 = juego_jugador_1(tabla_jugador_1, contador);
            contador += 1;
            println!("");
        } else if contador % 2 == 0 && contador < 21 {
            tabla_jugador_2 = juego_jugador_2(tabla_jugador_2, contador);
            contador += 1;
            println!("");
        } else if contador == 21 {
            ganador(tabla_jugador_1, tabla_jugador_2, jugador1, jugador2);
            break;
        }   
    }
}
