use std::env; // Importar la biblioteca para manejar argumentos de línea de comandos
use reqwest; // Importar la biblioteca para realizar solicitudes HTTP
use regex::Regex; // Importar la biblioteca para trabajar con expresiones regulares
use std::collections::HashSet; // Importar la biblioteca para almacenar elementos únicos
use std::fs; // Importar la biblioteca para operaciones de lectura de archivos

use reqwest::StatusCode; // Importar el tipo StatusCode de reqwest
use std::thread; // Importar la biblioteca para manejar hilos
use std::sync::mpsc; // Importar la biblioteca para comunicación entre hilos

// Función para obtener el código de estado HTTP de una URL
fn get_http_status(url: &str) -> Result<StatusCode, reqwest::Error> {
    let response = reqwest::blocking::get(url)?; // Realizar solicitud HTTP bloqueante
    Ok(response.status()) // Devolver el código de estado de la respuesta
}

// Función principal del programa
fn main() {
    let args: Vec<String> = env::args().collect(); // Obtener argumentos de línea de comandos
    if args.len() < 3 {
        eprintln!("Uso: {} <dominio_objetivo> <opcion> [diccionario]", args[0]); // Mostrar mensaje de uso si faltan argumentos
        return;
    }
    let objetivo = &args[1]; // Obtener el dominio objetivo de los argumentos

    match args.get(2).map(|s| s.as_str()) { // Realizar un match en base a la opción proporcionada
        Some("gtm") => gtm(objetivo), // Llamar a la función gtm si la opción es "gtm"
        Some("dic") => { // Llamar a la función dic si la opción es "dic"
            if let Some(diccionario) = args.get(3) { // Verificar si se proporcionó un diccionario como argumento
                dic(objetivo, diccionario); // Llamar a la función dic con el dominio objetivo y el diccionario
            } else {
                eprintln!("Debe proporcionar el nombre del diccionario como argumento"); // Mostrar mensaje de error si falta el diccionario
            }
        },
        _ => eprintln!("Opción no válida"), // Mostrar mensaje de error si la opción no es válida
    }
}

// Función para realizar acciones relacionadas con GTM (Google Tag Manager)
fn gtm(objetivo: &str) {
    if let Ok(etiqueta) = obtener_etiqueta_gtm(objetivo) { // Obtener la etiqueta GTM del dominio objetivo
        if let Ok(subdominios) = obtener_dominios(objetivo, &etiqueta) { // Obtener los subdominios relacionados con la etiqueta GTM
            let subdominios = eliminar_duplicados(subdominios); // Eliminar duplicados de los subdominios encontrados

            for s in subdominios { // Iterar sobre los subdominios y mostrarlos en pantalla
                println!("{}", s);
            }
        }
    }
}

// Función para realizar acciones basadas en un diccionario de palabras
fn dic(objetivo: &str, diccionario: &str) {
    let content = fs::read_to_string(diccionario).expect("Error al leer el archivo"); // Leer el contenido del diccionario desde un archivo

    let re = Regex::new(r"\b\w+\b").unwrap(); // Crear una expresión regular para encontrar palabras

    let words: Vec<String> = re.find_iter(&content) // Encontrar palabras en el contenido del diccionario y convertirlas a minúsculas
        .map(|m| m.as_str().to_lowercase())
        .collect();

    let dominio_raiz = objetivo.to_owned(); // Obtener el dominio raíz del objetivo

    println!("Dominio Raíz: {}", dominio_raiz); // Mostrar el dominio raíz en pantalla

    let (tx, rx) = mpsc::channel(); // Crear un canal para comunicarse con los hilos secundarios

    let words_clone = words.clone(); 
    let dominio_raiz_clone = dominio_raiz.clone();

    thread::spawn(move || { 
        for word in &words_clone { 
            let subdominio = format!("{}.{}", word, dominio_raiz_clone); 
            let url = format!("https://{}", subdominio);

            match get_http_status(&url) { 
                Ok(status) => println!("Código de respuesta para {}: {}", subdominio, status), 
                Err(_e) => {/*NO IMPRIME ERROR*/}, 
            }
        }
        tx.send(()).unwrap(); 
    });

    rx.recv().unwrap(); 
}
// Función para obtener la etiqueta GTM (Google Tag Manager) de un dominio
fn obtener_etiqueta_gtm(objetivo: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://{}", objetivo); // Construir la URL para la solicitud HTTP
    let res = reqwest::blocking::get(&url)?.text()?; // Realizar solicitud HTTP y obtener el cuerpo de la respuesta

    let re = Regex::new(r"GTM-[A-Z0-9]{7}").unwrap(); // Crear una expresión regular para encontrar la etiqueta GTM
    let etiqueta = re.find(&res).map_or("", |m| m.as_str()).to_string(); // Encontrar y extraer la etiqueta GTM del cuerpo de la respuesta

    Ok(etiqueta) // Devolver la etiqueta GTM encontrada
}

// Función para obtener los dominios relacionados con una etiqueta GTM
fn obtener_dominios(objetivo: &str, etiqueta: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = format!("https://googletagmanager.com/gtm.js?id={}", etiqueta); // Construir la URL para obtener los dominios relacionados
    let res = reqwest::blocking::get(&url)?.text()?; // Realizar solicitud HTTP y obtener el cuerpo de la respuesta

    let dominio_raiz = analizar_dominio_raiz(objetivo); // Obtener el dominio raíz del objetivo

    let re = Regex::new(&format!(r"(([a-zA-Z0-9-\.]+)?\.)?{}\.[a-zA-Z]{{0,3}}(\.[a-zA-Z]{{0,3}})?", dominio_raiz)).unwrap(); // Crear una expresión regular para encontrar dominios relacionados
    let dominios: Vec<String> = re.find_iter(&res).map(|m| m.as_str().to_string()).collect(); // Encontrar y recopilar los dominios encontrados

    Ok(dominios) // Devolver los dominios relacionados encontrados
}

// Función para analizar y extraer el dominio raíz de un objetivo dado
fn analizar_dominio_raiz(objetivo: &str) -> String {
    let partes_dominio: Vec<&str> = objetivo.split('.').collect(); // Dividir el dominio en partes separadas por puntos
    let num_partes = partes_dominio.len(); // Obtener el número de partes del dominio
    
    let mut dominio_raiz = String::new(); // Inicializar una cadena para almacenar el dominio raíz

    if num_partes >= 5 && (partes_dominio[num_partes - 1].len() == 2 || partes_dominio[num_partes - 1].len() == 3) {
        dominio_raiz.push_str(partes_dominio[num_partes - 6]); // Si se cumplen ciertas condiciones, seleccionar una parte específica como dominio raíz
    } else if num_partes >= 2 {
        dominio_raiz.push_str(partes_dominio[num_partes - 2]); // De lo contrario, seleccionar otra parte como dominio raíz
    }

    dominio_raiz // Devolver el dominio raíz obtenido
}

// Función para eliminar duplicados de una lista de dominios
fn eliminar_duplicados(dominios: Vec<String>) -> Vec<String> {
    let mut encontrados = HashSet::new(); // Crear un HashSet para almacenar elementos únicos
    
    dominios.into_iter().filter(|v| encontrados.insert(v.clone())).collect() // Filtrar y recopilar elementos únicos en una nueva lista
}