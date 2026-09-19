use std::sync::{Arc, Mutex} ;
use std::thread ;

fn main() {
    let m = Arc::new(Mutex::new(false)) ;

    let m2 = m.clone() ;

    let h = thread::spawn(move || {
        let mut g = m2.lock().unwrap() ;

        if !*g {
            *g = !*g ;
        }
    }) ;

    h.join().unwrap() ;

    println!("{}", *m.lock().unwrap()) ;    // Out: true

    // Второй вариант:

    let h = thread::spawn({ // <- добавлена кавычка {
        let m2 = m.clone() ; // создание клона
        move || {
            let mut g = m2.lock().unwrap() ;
            if *g {
                *g = !*g ;
            }
        }
    }) ;

    h.join().unwrap() ;

    println!("{}", *m.lock().unwrap()) ;    // Out: false
}
