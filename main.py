# Variable global para controlar la precisión decimal
DECIMALES = 10

def metodo_euler(a, b, N, w0):
    """Método de Euler."""
    h = (b - a) / N
    X = a
    w = w0
    resultados = [("Euler", X, w)]
    for _ in range(1, N + 1):
        w = w + h * (1 + (w / X))
        X = X + h
        resultados.append(("Euler", X, w))
    return resultados

def metodo_taylor_orden2(a, b, N, w0):
    """Método de Taylor de Orden 2."""
    h = (b - a) / N
    X = a
    w = w0
    resultados = [("Taylor Orden 2", X, w)]
    for _ in range(N):
        Ti = (1 + (w / X)) + ((h / 2) * (1 / X))
        w = w + h * Ti
        X = X + h
        resultados.append(("Taylor Orden 2", X, w))
    return resultados

def metodo_modificado_euler(a, b, N, w0):
    """Método Modificado de Euler."""
    h = (b - a) / N
    x = a
    y = w0
    resultados = [("Modificado Euler", x, y)]
    for _ in range(1, N + 1):
        xt = x + h / 2
        wt = y + (h / 2) * (1 + y / x)
        f = 1 + wt / xt
        y = y + h * f
        x = x + h
        resultados.append(("Modificado Euler", x, y))
    return resultados

def metodo_heun(a, b, N, w0):
    """Método de Heun."""
    h = (b - a) / N
    x = a
    y = w0
    resultados = [("Heun", x, y)]
    for _ in range(1, N + 1):
        k1 = 1 + (y / x)  # Pendiente inicial
        k2 = 1 + ((y + h * k1) / (x + h))  # Pendiente final
        y = y + (h / 2) * (k1 + k2)  # Promedio de las pendientes
        x = x + h
        resultados.append(("Heun", x, y))
    return resultados

def metodo_punto_medio(a, b, N, w0):
    """Método del Punto Medio."""
    h = (b - a) / N
    x = a
    y = w0
    resultados = [("Punto Medio", x, y)]
    for _ in range(N):
        k1 = 1 + (y / x)  # Pendiente inicial
        y_mid = y + (h / 2) * k1  # Estimación en el punto medio
        x_mid = x + h / 2  # Coordenada x en el punto medio
        k2 = 1 + (y_mid / x_mid)  # Pendiente en el punto medio
        y = y + h * k2  # Actualización usando la pendiente del punto medio
        x = x + h
        resultados.append(("Punto Medio", x, y))
    return resultados

def metodo_runge_kutta(a, b, N, w0):
    """Método de Runge-Kutta de Orden 4."""
    h = (b - a) / N
    x = a
    y = w0
    resultados = [("Runge-Kutta Orden 4", x, y)]
    for _ in range(1, N + 1):
        k1 = 1 + y / x
        k2 = 1 + (y + h * k1 / 2) / (x + h / 2)
        k3 = 1 + (y + h * k2 / 2) / (x + h / 2)
        k4 = 1 + (y + h * k3) / (x + h)
        y = y + (h / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
        x = x + h
        resultados.append(("Runge-Kutta Orden 4", x, y))
    return resultados

def ejecutar_metodos(a, b, N, w0):
    try:
        metodos = [
            metodo_euler,
            metodo_taylor_orden2,
            metodo_punto_medio,
            metodo_modificado_euler,
            metodo_heun,
            metodo_runge_kutta
        ]
        resultados_finales = []
        print(f"\nParámetros: a = {a}, b = {b}, N = {N}, w0 = {w0}\n")
        for metodo in metodos:
            resultados = metodo(a, b, N, w0)
            nombre = resultados[0][0]
            for _, X, W in resultados:
                print(f"x = {X:.{DECIMALES}f}, w = {W:.{DECIMALES}f}")
            resultados_finales.append((nombre, resultados[-1][2]))
        
        print("\n=== Resultados Finales ===")
        for nombre, w_final in resultados_finales:
            print(f"{nombre}: w_final = {w_final:.{DECIMALES}f}")
    except Exception as e:
        print(f"Ha ocurrido un error: {e}")

def header():
    print("###############################################################################")
    print("# Examen Ordinario: Resolución de Ecuación Diferencial por Diferentes Métodos #")
    print("###############################################################################")
    print("\n")
    print("Métodos Numéricos: AD2024")
    print("\n")
    print("Integrantes:")
    print("Fernando Villarreal Castillo 2049219")
    print("Julio Alejandro Galindo Estrada 1945686")
    print("Oscar Eduardo Reyes Pereyra 2109292")
    print("\n")

def main():
    try:
        header()  
        a = 1  
        b = 5  
        w0 = 0  
        N = int(input("Inserte un valor de N (número de pasos): "))
        if N <= 0:
            raise ValueError("N debe ser un número positivo.")
        ejecutar_metodos(a, b, N, w0)
    except ValueError as ve:
        print(f"Entrada inválida: {ve}")
    except Exception as e:
        print(f"Ha ocurrido un error inesperado: {e}")

if __name__ == "__main__":
    main()
