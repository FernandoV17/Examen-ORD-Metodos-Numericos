use std::fmt;

const DECIMALES: usize = 10;

#[derive(Debug)]
struct Resultado {
    metodo: &'static str,
    x: f64,
    w: f64,
}

impl fmt::Display for Resultado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<20} x = {:.precision$}, w = {:.precision$}",
            self.metodo,
            self.x,
            self.w,
            precision = DECIMALES
        )
    }
}

fn imprimir_header() {
    println!("###############################################################################");
    println!("# Examen Ordinario: Resolución de Ecuación Diferencial por Diferentes Métodos #");
    println!("###############################################################################\n");
    println!("Métodos Numéricos: AD2024\n");
    println!("Integrantes:");
    println!("Fernando Villarreal Castillo 2049219");
    println!("Julio Alejandro Galindo Estrada 1945686");
    println!("Oscar Eduardo Reyes Pereyra 2109292\n");
}

fn metodo_euler(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Euler",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        w += h * (1.0 + w / x);
        x += h;
        resultados.push(Resultado {
            metodo: "Euler",
            x,
            w,
        });
    }

    resultados
}

fn metodo_taylor_orden2(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Taylor Orden 2",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        let ti = (1.0 + w / x) + (h / 2.0) * (1.0 / x);
        w += h * ti;
        x += h;
        resultados.push(Resultado {
            metodo: "Taylor Orden 2",
            x,
            w,
        });
    }

    resultados
}

fn metodo_modificado_euler(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Modificado Euler",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        let xt = x + h / 2.0;
        let wt = w + (h / 2.0) * (1.0 + w / x);
        let f = 1.0 + wt / xt;
        w += h * f;
        x += h;
        resultados.push(Resultado {
            metodo: "Modificado Euler",
            x,
            w,
        });
    }

    resultados
}

fn metodo_heun(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Heun",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        let k1 = 1.0 + w / x;
        let k2 = 1.0 + (w + h * k1) / (x + h);
        w += (h / 2.0) * (k1 + k2);
        x += h;
        resultados.push(Resultado {
            metodo: "Heun",
            x,
            w,
        });
    }

    resultados
}

fn metodo_punto_medio(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Punto Medio",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        let k1 = 1.0 + w / x;
        let y_mid = w + (h / 2.0) * k1;
        let x_mid = x + h / 2.0;
        let k2 = 1.0 + y_mid / x_mid;
        w += h * k2;
        x += h;
        resultados.push(Resultado {
            metodo: "Punto Medio",
            x,
            w,
        });
    }

    resultados
}

fn metodo_runge_kutta(a: f64, b: f64, n: usize, w0: f64) -> Vec<Resultado> {
    let h = (b - a) / n as f64;
    let mut resultados = vec![Resultado {
        metodo: "Runge-Kutta Orden 4",
        x: a,
        w: w0,
    }];
    let mut x = a;
    let mut w = w0;

    for _ in 0..n {
        let k1 = 1.0 + w / x;
        let k2 = 1.0 + (w + h * k1 / 2.0) / (x + h / 2.0);
        let k3 = 1.0 + (w + h * k2 / 2.0) / (x + h / 2.0);
        let k4 = 1.0 + (w + h * k3) / (x + h);
        w += (h / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        x += h;
        resultados.push(Resultado {
            metodo: "Runge-Kutta Orden 4",
            x,
            w,
        });
    }

    resultados
}

fn ejecutar_metodos(a: f64, b: f64, n: usize, w0: f64) {
    let metodos = vec![
        metodo_euler,
        metodo_taylor_orden2,
        metodo_modificado_euler,
        metodo_heun,
        metodo_punto_medio,
        metodo_runge_kutta,
    ];

    let mut resultados_finales = vec![];

    for metodo in metodos {
        let resultados = metodo(a, b, n, w0);
        for resultado in &resultados {
            println!("{}", resultado);
        }
        println!();

        if let Some(ultimo) = resultados.last() {
            resultados_finales.push((ultimo.metodo, ultimo.w));
        }
    }

    println!("=== Resultados Finales ===");
    for (metodo, w_final) in resultados_finales {
        println!(
            "{:<20} w_final = {:.precision$}",
            metodo,
            w_final,
            precision = DECIMALES
        );
    }
}

fn main() {
    imprimir_header();

    let a = 1.0;
    let b = 5.0;
    let w0 = 0.0;

    println!("Inserte un valor de N (número de pasos): ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    let n: usize = input
        .trim()
        .parse()
        .expect("N debe ser un número entero positivo");

    if n > 0 {
        ejecutar_metodos(a, b, n, w0);
    } else {
        eprintln!("N debe ser mayor que 0.");
    }
}
