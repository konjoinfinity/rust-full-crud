let state = State {
        dinos: Default::default()
    };

    let mut app = tide::with_state(state);
    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.at("/dinos")
       .post(|mut req: Request<State>| async move {
           let  dino: Dino = req.body_json().await?;
           // let get a mut ref of our store ( hashMap )
           let mut dinos = req.state().dinos.write().await;
           dinos.insert(String::from(&dino.name), dino.clone());
           let mut res = Response::new(201);
           res.set_body(Body::from_json(&dino)?);
          Ok(res)
     })
