## Ownership. Propiedad.

Todos los programas tienen que administrar la forma en que usan la memoria de una computadora mientras se ejecutan. En Rust la memoria se administra a través de un sistema de propiedad con un conjunto de reglas que el compilador verifica en el momento de la compilación. Ninguna de las características de propiedad ralentiza su programa mientras se está ejecutando.

#### Reglas de propiedad:
 * Cada valor en Rust tiene un **propietario**.
 * Solo puede haber un propietario a la vez.
 * Cuando el propietario sale del alcance, el valor se descartará.
 * Los valores devueltos (*return*) también devuelven la propiedad.

## Referencias y préstamos.
El símbolo de `&` se llaman referencias y permiten referirse a algún valor sin tomar posesión de él.
Las referencias también son llamadas como *funciones de préstamo*.
Las referencias son inmutables por defecto, para volverlas mutables sólo se debe de especificar que la variable sera `mut` y a la referencia se le debe de colocar `&mut`, aunque sólo se podrá hacer una referencia mutable a un dato, a la vez que tampoco se puede tener una referencia mutable teniendo una inmutable.

#### Reglas de referencias.
 * En algún momento se puede tener ya sea una referencia mutable o algún número n de referencias inmutables.
 * Las referencias siempre deben de ser válidas.

## Slice. Segmentos.
Se le llamana *referencias por segmento* a aquellas a las que puedes tomar por referencia parcialmente. Por ejemplo, en un tipo String definido por "hello world" puedes crear dos variables, una que tome referencia para los primeros 5 bytes y otra para los bytes restantes. 
