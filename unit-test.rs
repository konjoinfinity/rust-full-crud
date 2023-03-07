#[async_std::test]
async fn list_dinos() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};

    let dino = Dino {
        name: String::from("test"),
        weight: 50,
        diet: String::from("carnivorous"),
    };

    let mut dinos_store = HashMap::new();
    dinos_store.insert(dino.name.clone(), dino);
    let dinos: Vec<Dino> = dinos_store.values().cloned().collect();
    let dinos_as_json_string = serde_json::to_string(&dinos)?;

    let state = Arc::new(RwLock::new(dinos_store));
    let app = server(state).await;

    let url = Url::parse("https://example.com/dinos").unwrap();
    let req = Request::new(Method::Get, url);
    let mut res: Response = app.respond(req).await?;
    let v = res.body_string().await?;
    assert_eq!(dinos_as_json_string, v);
    Ok(())
}

#[async_std::test]
async fn create_dino() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};

    let dino = Dino {
        name: String::from("test"),
        weight: 50,
        diet: String::from("carnivorous"),
    };

    let dinos_store = HashMap::new();

    let state = Arc::new(RwLock::new(dinos_store));
    let app = server(state).await;

    let url = Url::parse("https://example.com/dinos").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&dino)?);
    let res: Response = app.respond(req).await?;
    assert_eq!(201, res.status());
    Ok(())
}
