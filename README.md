# ¡Crea tu test autocorregible a partir de un PDF!
Esta aplicación web te permite generar un test a partir de los contenidos de un PDF en los principales formatos autocorregibles (GIFT y Moodle XML) que podrás utilizar en los LMS más comunes

Además de estos formatos también se generará un archivo HTML que se puede abrir directamente en el navegador, sin necesidad de ningún requisito ni infraestructura. Para quienes necesiten que este archivo esté online se ha implementado un desarrollo para subirlo a <a href="https://neocities.org/" target="_blank">Neocities</a>, una plataforma que permite el alojamiento gratutito de archivos estáticos

<img src="https://i.imgur.com/4GufYkn.png" alt="Captura de pantalla de la aplicación web">

## Ejecución y uso
Descarga la versión correspondiente a tu sistema operativo y arquitectura y ejecuta el binario. Buscará un puerto libre y levantará ahí la aplicación web que podrás abrir desde cualquier navegador.

## Configuración
La lectura del PDF y la preparación de las preguntas se realiza utilizando los servicios de <a href="https://www.chatpdf.com" target="_blank">ChatPDF</a>. La elección de esta plataforma es que ofrece un plan gratuito (sin necesidad de introducir tarjeta) con unos límites bastante generos para un uso no profesional (500 mensajes y 5000 páginas PDF por mes).

Para su uso tendrás que introducir la <code>x-api-key</code> que encontrarás en tu perfil.

En el caso de que quieras subir la página a Neocities deberás crear una <code>api-key</code> e incorporarla en la configuración. Puedes hacerlo desde **Sites > Manage site settings > Api**

Ambos ajustes se realizan en la aplicación web desde el desplegable **Instrucciones y configuración**

### Consejos y uso
Al incorporar un nombre utiliza solo caracteres alfanuméricos y guiones y se recomienda el uso de tildes o acentuaciones especiales. Ten en cuenta que con esa información se creará ka carpeta que contendrá los archivos generados en el directorio <strong>mis_tests</strong> junto al ejecutable o se subirá a la web de Neocities, por lo debe ser amigable.

Aunque ChatPDF admite archivos hasta 32 megas, esta aplicación web solo procesará el archivo si es menor de 8 megas. Si tienes un PDF que ocupa más espacio es mejor que lo comprimas, no tengas miedo en perder la calidad de las imágenes puesto que no se van a usar.

### Ejemplos
El material utilizado procede del curso <a href="https://ocw.unican.es/course/view.php?id=11" target="_blank">Introducción a la Antropología Social y Cultural (2010)</a> dentro de la iniciativa OpenCourseWare de la Universidad de Cantabria publicados con licencia Creative Commons 4.0 BY-NC-SA.

De este curso, se han seleccionado 3 temas:
- Tema 2. El concepto de cultura. <a href="https://antikorps13.neocities.org/concepto_cultura" target="_blank">Test generado</a>
- Tema 6. Antropología de las creencias. <a href="https://antikorps13.neocities.org/creencias" target="_blank">Test generado</a>
- Tema 7. Género y Cultura. <a href="https://antikorps13.neocities.org/genero_cultura">Test generado</a>	
  

<img src="https://i.imgur.com/AtkKH1p.png" alt="Captura de pantalla">

## Nota del autor
Esta aplicación no es más que una aproximación a la idea, no tiene ninguna finalidad de uso real. Si te parece un proyecto interesante para desarrollar busco colaboradores para realizar una primera versión realmente operativa y funcional con mejor interfaz, una gestión más humana de errores, nuevos formatos de tests, posibilidades de penalización por error, diferentes tipos de preguntas, etc. 

Como tecnología de desarrollado he elegido Rust por la facilidad de proveer un archivo ejecutable, pero no tendría problema en adaptarme a otro stack tecnológico. Escríbeme y seguro que nos podemos poner de acuerdo 😄 
