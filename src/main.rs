use futures::executor;

async fn ejemplo_tokio() -> String {
    "Prueba tokio".to_string()
}

async fn ejemplo_libreria_estandar() -> String {
    "Prueba libreria estandar".to_string()
}

extern crate reqwest;
async fn ejemplo_consulta_datos_api() -> Result<String, reqwest::Error> {
    let url = "https://query1.finance.yahoo.com/v8/finance/chart/TSLA";

    reqwest::get(url).await?.text().await
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

    // Consultando datos de una api
    println!("Inicio!");
    let resultado = ejemplo_consulta_datos_api().await;
    println!("{:?}",resultado);
    println!("Fin!");
}
