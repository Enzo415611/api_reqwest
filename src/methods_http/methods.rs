use reqwest::{get, Client};
use serde_json::{from_str, Value};
use crate::{MyApp, Types};

impl MyApp {
    pub fn verifier_method(&mut self) {
        match self.method_is_active {
            Types::Get => self.method_get(),
            Types::Post => self.method_post(),
            Types::Delete => self.method_delete(),
            Types::Put => self.method_put(),
        }
    }

    pub fn convert_json(&self) -> Value{
        let convert_json: Value =
            match from_str(&*self.input_body_str) {
                Ok(resp) => {resp},
                Err(e) => {
                    match from_str(format!("erro ao converter: {}",e ).as_str()) {
                        Ok(r) => {
                            r
                        },
                        Err(e) => {
                            Value::Null
                        }
                    }

                },
            };
        convert_json
    }

    pub fn method_get(&mut self) {
        // clone da variavel tx usada para enviar mensagem entre threads
        let tx = self.tx.clone();
        // clone da URL para ser usada dentro da thread
        let url_clone = self.url.clone();
        // inicia nova thread async
        tokio::spawn(async move {
            // resultado da url
            // a variavel result recebe o resultado do match
            let result =
                // valida se foi possivel fazer a requisição a Api
                match get(&url_clone).await {
                    // se ok entramos dentro de outro match
                    Ok(response) =>
                    // valida a resposta da Api
                        match response.text().await {
                            Ok(text) => Ok(text),
                            // Mensagem de erro de resposta da Api
                            Err(e) => { Err(format!("Erro ao ler a resposta da Api: {}", e)) }
                        },
                    // Mensagem de erro de requisição
                    Err(e) => Err(format!("Erro na requisição: {}", e))
                };
            // envia o valor de result
            tx.send(result.expect("Erro ao enviar a resposta")).unwrap()
        });
    }

    pub fn method_post(&mut self) {
        self.body = self.convert_json();
        let client = Client::new();

        // clone
        let url_clone = self.url.clone();
        let clone_body = self.body.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let resp =
                match client.post(&url_clone)
                    .json(&clone_body)
                    .send()
                    .await
                    .unwrap()
                    .text().await {
                    Ok(respo) => Ok(respo),
                    Err(e) => Err(format!("Erro ao enviar o json a api: {}", e))
                };

            tx.send(resp.expect("Erro ao enviar a resposta entre threads")).unwrap();
        });
    }

    pub fn method_delete(&mut self){
        let client = Client::new();
        self.body = self.convert_json();
        // clone
        let url_clone = self.url.clone();
        let clone_body = self.body.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let resp =
                match client.delete(&url_clone)
                    .json(&clone_body)
                    .send()
                    .await
                    .unwrap()
                    .text().await {
                    Ok(respo) => Ok(respo),
                    Err(e) => Err(format!("Erro ao enviar o json a api: {}", e))
                };

            tx.send(resp.expect("Erro ao enviar a resposta entre threads")).unwrap();
        });
    }

     fn method_put(&mut self){
        let client = Client::new();
        self.body = self.convert_json();

        // clone
        let url_clone = self.url.clone();
        let tx = self.tx.clone();
        let clone_body = self.body.clone();

        tokio::spawn(async move {
            let resp =
                match client.put(&url_clone)
                    .json(&clone_body)
                    .send()
                    .await
                    .unwrap()
                    .text().await {
                    Ok(respo) => Ok(respo),
                    Err(e) => Err(format!("Erro ao enviar o json a api: {}",e))
                };
            tx.send(resp.expect("Erro ao enviar a resposta entre threads")).unwrap();
        });
    }
}



