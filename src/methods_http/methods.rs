use reqwest::{get, Client, RequestBuilder};
use serde_json::json;
use crate::MyApp;

impl MyApp{
    pub fn method_get(&mut self){
        // clone da variavel tx usada para enviar mensagem entre threads
        let tx = self.tx.clone();
        // clone da URL para ser usada dentro da thread
        let url_to_fetch = self.url.clone();
        // inicia nova thread async
        tokio::spawn(async move {
            // resultado da url
            // a variavel result recebe o resultado do match
            let result =

                // valida se foi possivel fazer a requisição a Api
                match get(&url_to_fetch).await {
                    // se ok entramos dentro de outro match
                    Ok(response) =>
                        // valida a resposta da Api
                        match response.text().await {
                            Ok(text) => Ok(text),
                            // Mensagem de erro de resposta da Api
                            Err(e) =>  {Err(format!("Erro ao ler a resposta da Api: {}",e ))}
                        },
                    // Mensagem de erro de requisição
                    Err(e) => Err(format!("erro na requisição: {}", e))
            };
            // envia o valor de result
            tx.send(result.expect("err")).unwrap()
        });
    }

    pub fn method_post(&mut self){
        let client = Client::new();
        tokio::spawn(async move {
            let resp = client.post("")
                .json(&json!("nome: efef, age: 3"))
                .send()
                .await
                .unwrap()
                .text().await;
        });
    }
}


