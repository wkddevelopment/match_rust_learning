// Link: https://rust-lang-de.github.io/rustbook-de/ch06-02-match.html


fn main(){

    fn plus_eins(x: Option<i32>) -> Option<i32>{
        match x {
            None => None, // Prüfung auf Null
            Some(i) => Some(i + 1),
        }
    }

    let eins = Some(1);
    let zwei = plus_eins(eins);
    println!("Zwei ist: {:?}", zwei);

}




/*
enum Münze {
    ZweiEuro,
    EinEuro(KopfBilder),
    FünzigCent,
    ZwanzigCent,
    ZehnCent,
    FünfCent,
    EinCent,
}

#[derive(Debug)]
enum KopfBilder {
    MariaTheresia,
    Schönbrunn,
    Riesenrad,
}


fn main() {
    
    fn münzsortiermaschine(münze: Münze) -> f32 {
        match münze {
            Münze::ZweiEuro => 2.0,
            Münze::EinEuro(bild) => {
                println!("1€ Münze mit einem Bild von {:?} darauf", bild);
                1.0
            }
            Münze::FünzigCent => {
                println!("50 Cent ist (k- ?)ein guter Musiker.");
                0.5
            },
            Münze::ZwanzigCent => 0.2,
            Münze::ZehnCent => 0.1,
            Münze::FünfCent => 0.05,
            Münze::EinCent => 0.01
        }
    }

    let erste_münze = münzsortiermaschine(Münze::FünzigCent);
    println!("Der Wert der ersten eingeworfenen Münze beträgt: {}", erste_münze);
    let zweite_münze = münzsortiermaschine(Münze::EinEuro(KopfBilder::Schönbrunn));
    println!("Der Wert der zweiten eingeworfenen Münze beträgt: {}", zweite_münze);

}

*/