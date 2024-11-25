# Resolución de Ecuación Diferencial por Métodos Numéricos

Este repositorio contiene un programa desarrollado como parte del Examen Ordinario del curso de Métodos Numéricos. El objetivo es resolver una ecuación diferencial ordinaria utilizando varios métodos numéricos implementados en Python. El programa realiza cálculos iterativos y muestra los resultados paso a paso en consola, proporcionando aproximaciones numéricas con diversos métodos.

## Métodos Implementados

El programa incluye las siguientes aproximaciones numéricas:

1. **Método de Euler**
2. **Método de Taylor de Orden 2**
3. **Método del Punto Medio**
4. **Método Modificado de Euler**
5. **Método de Heun**
6. **Método de Runge-Kutta de Orden 4**

Cada método permite resolver ecuaciones diferenciales ordinarias utilizando sus respectivas fórmulas iterativas.

## Ecuación Diferencial a Resolver

La ecuación diferencial planteada es:

\[
\frac{dw}{dx} = 1 + \frac{w}{x}
\]

sujeta a la condición inicial:

\[
w(1) = 0
\]

El programa calcula soluciones aproximadas en el intervalo \([1, 5]\) con un número definido de iteraciones (\(N\)) proporcionado por el usuario.

## Estructura del Código

El programa está organizado de la siguiente manera:

1. **Definición de Parámetros**: Incluye valores iniciales (\(x_0\), \(w_0\)), el intervalo \([a, b]\), y el número de pasos (\(N\)).
2. **Implementación de Métodos**: Cada método se encuentra implementado como una función independiente que realiza las aproximaciones numéricas.
3. **Ejecución Simultánea**: Un módulo principal ejecuta todos los métodos y compara los resultados.
4. **Impresión de Resultados**: Se muestran los valores calculados en cada iteración y los resultados finales de cada método.

## Ejecución

### Requisitos

- Python 3.x

### Cómo Ejecutar

1. Clona este repositorio:

   ```bash
   git clone https://github.com/FernandoV17/Examen-ORD-Metodos-Numericos.git
   cd Examen-ORD-Metodos-Numericos
   ```

2. Ejecuta el script principal:

   ```bash
   python3 main.py
   ```

3. Proporciona el valor de \(N\) (número de pasos) cuando se solicite en consola.

### Ejemplo de Salida

El programa imprimirá resultados en consola como el siguiente:

```
=== Ejecución Simultánea de Métodos Numéricos ===
Parámetros: a = 1, b = 5, N = 10, w0 = 0

Método: Euler
x = 1.0000000000, w = 0.0000000000
x = 1.4000000000, w = 0.4000000000
...

Método: Runge-Kutta Orden 4
x = 1.0000000000, w = 0.0000000000
x = 1.4000000000, w = 0.4074074074
...

=== Resultados Finales ===
Euler: w_final = 4.5678901234
Taylor Orden 2: w_final = 4.5679012345
Punto Medio: w_final = 4.5679123456
...
```

## Explicación de Métodos

### Método de Euler

El Método de Euler es un procedimiento iterativo basado en la fórmula:

\[
w\_{i+1} = w_i + h \cdot f(x_i, w_i)
\]

donde \(h\) es el tamaño del paso.

### Método de Runge-Kutta de Orden 4

Este método utiliza una combinación de cuatro cálculos intermedios (\(k_1, k_2, k_3, k_4\)) para proporcionar una aproximación precisa:

\[
w\_{i+1} = w_i + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)
\]

con las pendientes definidas como:

\[
\begin{aligned}
k_1 & = f(x_i, w_i), \\
k_2 & = f\left(x_i + \frac{h}{2}, w_i + \frac{h}{2}k_1\right), \\
k_3 & = f\left(x_i + \frac{h}{2}, w_i + \frac{h}{2}k_2\right), \\
k_4 & = f(x_i + h, w_i + hk_3).
\end{aligned}
\]

### Otros Métodos

Cada método implementado utiliza variantes específicas de aproximación, incluyendo promedios de pendientes, correcciones iterativas o derivadas adicionales.

## Resultados y Análisis

El programa genera valores aproximados de \(x\) y \(w(x)\) para cada método, los cuales pueden ser utilizados para análisis adicional o comparación entre métodos.

## Autores

Este código fue desarrollado por:

- **Fernando Villarreal Castillo** (2049219)
- **Julio Alejandro Galindo Estrada** (1945686)
- **Oscar Eduardo Reyes Pereyra** (2109292)

### Universidad Autónoma de Nuevo León

Curso: Métodos Numéricos para IB  
Catedrático: Dr. Miguel Mata Pérez  
Fecha de entrega: 25 de noviembre de 2024
