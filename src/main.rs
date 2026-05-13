#[derive(Debug, Clone)]
struct Libro {
    isbn: u32,
    titulo: String,
}

struct Nodo {
    libro: Libro,
    //Primero usamos un option por que esto hace que si o si haya un hijo ahi, por que como no manejamos punteros nulos.
    //Lo que hace Box que es como un puntero, es que mueve el nodo al Heap (la memoria dinamica). 
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(libro: Libro) -> Self {
        Nodo {
            libro,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    //lo que esta haciendo el as.ref es crear una referencia del Option<Box<Nodo>>, asi no se genera ownership.
    //y lo que hace map_or es casi como un if, por que si es None, retorna 0, pero si tiene algo (Some(n)), entonces devolvera la altura.
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    //el take se pone, por que agarra el valor y se lo saca al Option, quedandose como None, y esto hace mover el ownership.
    let mut x = y.izquierdo.take().expect("Hijo izquierdo ausente");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Hijo derecho ausente");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

fn insertar(nodo_opt: Option<Box<Nodo>>, libro: Libro) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(libro)),
        Some(n) => n,
    };

    let isbn_nuevo = libro.isbn;

    if isbn_nuevo < nodo.libro.isbn {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), libro));
    } else if isbn_nuevo > nodo.libro.isbn {
        nodo.derecho = Some(insertar(nodo.derecho.take(), libro));
    } else {
        return nodo; 
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && isbn_nuevo < nodo.izquierdo.as_ref().unwrap().libro.isbn {
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo > nodo.derecho.as_ref().unwrap().libro.isbn {
        return rotar_izquierda(nodo);
    }
    if balance > 1 && isbn_nuevo > nodo.izquierdo.as_ref().unwrap().libro.isbn {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo < nodo.derecho.as_ref().unwrap().libro.isbn {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

fn imprimir(nodo: &Option<Box<Nodo>>, nivel: usize) {
    if let Some(n) = nodo {
        imprimir(&n.derecho, nivel + 1);
        println!("{:indent$}[ISBN: {}] {}", "", n.libro.isbn, n.libro.titulo, indent = nivel * 4);
        imprimir(&n.izquierdo, nivel + 1);
    }
}

fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro> {
//al usar el &Option<Box<Nodo>> hacemos referencia al arbol y no tomamos el ownership.

    match nodo {
        None => None,
            Some(n) => {
                if isbn == n.libro.isbn {
                    Some(&n.libro)
                } else if isbn < n.libro.isbn {
                    buscar(&n.izquierdo, isbn)
                } else {
                    buscar(&n.derecho, isbn)
                }
            }
    }   
}

fn encontrar_minimo(nodo: &Box<Nodo>) -> Libro {
    let mut actual = nodo;

    while let Some(ref izquierdo) = actual.izquierdo {
        actual = izquierdo;
    }
    actual.libro.clone()
}

fn main() {
    let mut raiz: Option<Box<Nodo>> = None;
    let datos = vec![
        (10, "El Quijote"), (20, "1984"), (30, "Hamlet"),
        (5, "Fahrenheit 451"), (2, "La Odisea"), (25, "El Principito"),
    ];

    println!("--- Sistema de Inventario de Librería (AVL) ---");
    for (isbn, titulo) in datos {
        let libro = Libro { isbn, titulo: titulo.to_string() };
        raiz = Some(insertar(raiz.take(), libro));
    }

    imprimir(&raiz, 0);
    
    /*
    Insertaremos [10,20,30,5,2,25]

    Insertamos 10
    10

    Insertamos 20
    10
      \
       20

    Insertamos 30
    10
      \
       20
         \
          30
    
    aqui hay desbalance RR en el nodo 10, asi que se aplica rotacion izquierda en 10,

        20
       /  \ 
      10  30

      ahora insertamos 5

      20
    /   \ 
   10     30
 / 
5
    
ahora insertamos el 2,

        20
       /  \ 
      10   30
     / 
    5
   /
  2

aqui hay desbalance LL en el nodo 10, se aplica rotacion izquierda en 5

         20
        /   \ 
       5     30
      / \ 
     2   10


ahora insertamos el 25 el ultimo numero,

    20
   /  \ 
  5    30
 / \ 
2   10
     \ 
      25

    */

    /*
    En Rust utilizar el .take() en las funciones de rotacion en lugar de asignacion directa , se utiliza para mover 
    temporalmente el contenido que tiene un Option, asi no viola las reglas de ownership del lenguaje. y durante las rotaciones
    del AVL, los nodos tienen que estar organizandose, y cambiando de propietario varias veces y de forma segura, el .take()
    extrae el valor y deja None en su lugar, asi esto evita que haya referencias mutables simultaneas, y movimientos 
    invalidos de memoria, una asignicacion directa haria que se produzcan muchos errores.
    */

    println!("\n------Busqueda ----------");

    match buscar(&raiz, 10){
        Some (libro) => {
            println!("Libro encontrado: {} - {}", libro.isbn , libro.titulo);
        }
        None => {
            println!("Libro no encontrado");
        }
    }


    match buscar(&raiz, 80){
        Some (libro) => {
            println!("Libro encontrado: {} - {}", libro.isbn , libro.titulo);
        }
        None => {
            println!("Libro no encontrado");
        }
    }
}