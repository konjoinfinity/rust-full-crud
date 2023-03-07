 .put(|mut req: Request<State>| async move {
            let dino_update: Dino = req.body_json().await?;
            let mut dinos = req.state().dinos.write().await;
            let key: String = req.param("name")?;
            let res = match dinos.entry(key) {
                Entry::Vacant(_entry) => Response::new(404),
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = dino_update;
                    let mut res = Response::new(200);
                    res.set_body(Body::from_json(&entry.get())?);
                    res
                }
            };
            Ok(res)
        })
        .delete(|req: Request<State>| async move {
            let mut dinos = req.state().dinos.write().await;
            let key: String = req.param("name")?;
            let deleted = dinos.remove(&key);
            let res = match deleted {
                None => Response::new(404),
                Some(_) => Response::new(204),
            };
            Ok(res)
        });
