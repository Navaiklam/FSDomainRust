## Find SubDomain in Rust

🔍 Este script en Rust te permite buscar subdominios de dos maneras: a través de Google Tag Manager o mediante una búsqueda tradicional con un diccionario. Utiliza múltiples hilos para realizar consultas HTTP y mostrar el código de respuesta.

### Uso del Script

Para utilizar el script, sigue las siguientes instrucciones:

- Para buscar subdominios utilizando Google Tag Manager:
cargo run dominio gtm


- Para realizar una búsqueda de fuerza bruta con un diccionario:
cargo run dominio dic archivo.txt


### Detalles del Funcionamiento

El script emplea múltiples hilos para llevar a cabo las consultas HTTP de manera eficiente. Proporciona el código de respuesta de cada subdominio encontrado, ya sea a través de Google Tag Manager o mediante la búsqueda con diccionario. Esta herramienta es útil para descubrir subdominios asociados a un dominio principal de forma automatizada y estructurada.

¡Explora y descubre los subdominios de manera rápida y efectiva con este script en Rust! 🚀