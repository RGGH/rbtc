
            panic!("Failed to parse json; error is {}", e);
        });
    
    let price = json["coin"]["price"].to_string();
    let price = price.split_once('.').unwrap();

    println!("Bitcoin Price {:?}", price.0);
  

    Ok(())
}