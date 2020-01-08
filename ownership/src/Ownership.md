# Ownership. Propiedad.

Todos los programas tienen que administrar la forma en que usan la memoria de una computadora mientras se ejecutan. En Rust la memoria se administra a través de un sistema de propiedad con un conjunto de reglas que el compilador verifica en el momento de la compilación. Ninguna de las características de propiedad ralentiza su programa mientras se está ejecutando.

### Reglas de propiedad:
 * Cada valor en Rust tiene un **propietario**.
 * Solo puede haber un propietario a la vez.
 * Cuando el propietario sale del alcance, el valor se descartará.
 * Los valores devueltos (*return*) también devuelven la propiedad.
