## Find SubDomain in Rust

🔍 Este script en Rust te permite buscar subdominios de dos maneras: a través de Google Tag Manager o mediante una búsqueda tradicional con un diccionario. Utiliza múltiples hilos para realizar consultas HTTP y mostrar el código de respuesta.

### Uso del Script

Para utilizar el script, sigue las siguientes instrucciones:

- Para buscar subdominios utilizando Google Tag Manager:
```bash
cargo run dominio gtm
```

- Para realizar una búsqueda de fuerza bruta con un diccionario:
```bash
cargo run dominio dic archivo.txt
```

### Detalles del Funcionamiento

El script emplea múltiples hilos para llevar a cabo las consultas HTTP de manera eficiente. Proporciona el código de respuesta de cada subdominio encontrado, ya sea a través de Google Tag Manager o mediante la búsqueda con diccionario. Esta herramienta es útil para descubrir subdominios asociados a un dominio principal de forma automatizada y estructurada.

¡Explora y descubre los subdominios de manera rápida y efectiva con este script en Rust! 🚀

### Recuerda que lo puedes compilar de esta forma:
```bash
cargo build
```
- En cargo.toml ya se encuentran las dependencias para un compilado optimizado.

## Find SubDomain in Rust

🔍 This Rust script allows you to search for subdomains in two ways: through Google Tag Manager or using a traditional dictionary search. It utilizes multiple threads to perform HTTP queries and display the response code.

### Script Usage

To use the script, follow these instructions:

- To search for subdomains using Google Tag Manager:
```bash
cargo run domain gtm
```

- For brute force dictionary search:
```bash
cargo run domain dic dictionary.txt
```

### Operation Details

The script leverages multiple threads to efficiently conduct HTTP queries. It provides the response code for each discovered subdomain, whether through Google Tag Manager or dictionary search. This tool is valuable for automated and structured exploration of subdomains associated with a primary domain.

Explore and discover subdomains quickly and effectively with this Rust script! 🚀
