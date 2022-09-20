use futures::executor;

async fn ejemplo_tokio() -> String {
    "Prueba tokio".to_string()
}

async fn ejemplo_libreria_estandar() -> String {
    "Prueba libreria estandar".to_string()
}

#[tokio::main]
async fn main() {

    //Usando tokio
    println!("Inicio!");
    let resultado=ejemplo_tokio().await;
    println!("{:?}",resultado);
    println!("Fin!");

    //Usando libreria estándar
    println!("Inicio!");
    let resultado=executor::block_on(ejemplo_libreria_estandar());
    println!("{:?}",resultado);
    println!("Fin!");

}
