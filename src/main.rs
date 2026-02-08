use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: todo [add|list|done] [argumentos]");
        return;
    }
    println!("Hello, world!");

    let command = &args[1];

    match command.as_str() {
        "add" => println!("Añadiendo tarea: {}", args[2]),
        "list" => println!("Listando tareas"),
        _ => println!("Comando no reconocido"),
    }
}

fn add_task(description: &str) -> std::io::Result<()> {
    // Abrimos el archivo. Si no existe, lo crea.
    // .append(true) hace que no borre lo anterior, sino que escriba al final.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tasks.txt")?;

    //Nota sobre el ?: Ese signo de interrogación al final de las funciones es pura magia de Rust. Significa: "Si esto da error, detente y devuelve el error; si sale bien, continúa".
    writeln!(file, "{}", description)?;

    Ok(())
}

struct Task {
    id: u32,
    description: String,
    done: bool,
}
