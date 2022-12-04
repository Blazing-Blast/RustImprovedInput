use std::io::{stdin,stdout,Write};


/// Reads a line from stdin() and returns up to max characters in a String.
/// 
/// This function will always remove trailing \n and the \r on Windows only.
/// 
/// It also does not break if a EOF is reached and instead acts as if a newline was reached prints one itself.
pub fn readline<T>(max: usize, query: T) -> String where T: std::fmt::Display{
    let mut out: String = String::new();
    print!("{}", query);
    let _ = stdout().flush();
    match stdin().read_line(&mut out){
        Ok(n) => {
            if n >= max{
                if !out.ends_with('\n') {println!();}
                out.truncate(max);
                return out
            }
            else{
                if out.ends_with('\n') {
                    out.pop();
                }
                else {println!();}
                if cfg!(windows) && out.ends_with('\r') {
                    out.pop();
                }
                return out
            }
        }
        Err(error) => {println!("We got an error while getting input \nThe error is: {}", error); return out}
    };
}


/// Reads lines from stdin() until the maximum of characters is met.
/// Do keep in mind that \n's and on Windows \r's too also count towards the charcter count.
/// 
/// This function will always remove trailing \n and the \r on Windows only.
/// 
/// It also does not break if a EOF is reached and instead acts as if a newline was reached prints one itself.
pub fn readlines<T>(max: usize, query: T) -> String where T: std::fmt::Display{
    let mut out: String = String::new();
    print!("{}", query);
    let _ = stdout().flush();
    let mut charcters: usize = 0;
    loop{
        match stdin().read_line(&mut out){
            Ok(n) => {charcters += n;
                if charcters >= max{
                if !out.ends_with('\n') {println!();}
                out.truncate(max);
                return out
                }
                else if charcters == max + 1 {
                    if out.ends_with('\n') {
                        out.pop();
                    }
                    else {println!();}
                    if cfg!(windows) && out.ends_with('\r') {
                        out.pop();
                    }
                    return out
                }
            }
            Err(error) => {println!("We got an error while getting input \nThe error is: {}", error);}
        };
    }
}