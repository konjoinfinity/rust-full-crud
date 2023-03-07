app.at("/dinos")
        .get(|req: Request<State>| async move {
            let dinos = req.state().dinos.read().await;
            // get all the dinos as a vector
            let dinos_vec: Vec<Dino> = dinos.values().cloned().collect();
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&dinos_vec)?);
            Ok(res)
        })
