pub fn print_error() {
    let x = grade(100);
    if !grade(100).is_err() {
        let y = x.unwrap();
        println!("!is_err:{}", y);
    }

    if let Ok(v) = grade(20) {
        println!("ok: {}", v);
    }

    let y = match grade(-10) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("Ok_Err: {}", y);
}

fn grade(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Invalid score".to_string());
    }

    Ok("A".to_string())
}