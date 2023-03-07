  app.at("/dinos/:name")
        .get(|req: Request<State>| async move {
            let mut dinos = req.state().dinos.write().await;
            let key: String = req.param("name")?;
            let res = match dinos.entry(key) {
                Entry::Vacant(_entry) => Response::new(404),
                Entry::Occupied(entry) => {
                    let mut res = Response::new(200);
                    res.set_body(Body::from_json(&entry.get())?);
                    res
                }
            };
            Ok(res)
        })
