use rand::Rng;
use strum_macros::Display;
#[derive(Display, Debug, PartialEq, Copy, Clone)]
enum F1Error {
    AccountIsEmpty,
    InvestmentLost,
}
#[derive(Display, Debug, PartialEq, Copy, Clone)]
enum F2Error {
    F1Error(F1Error),
}
#[derive(Display, Debug, PartialEq, Copy, Clone)]
enum F3Error {
    F1Error(F1Error),
}
#[derive(Display, Debug, PartialEq, Copy, Clone)]
enum F4Error {
    F2Error(F2Error),
    F3Error(F3Error),
}

fn f1() -> F1Error {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..10);
    if n % 2 == 0 {
        return F1Error::AccountIsEmpty;
    }
    return F1Error::InvestmentLost;
}

fn f2() -> F2Error {
    let err = f1();
    return F2Error::F1Error(err);
}

fn f3() -> F3Error {
    let err = f1();
    return F3Error::F1Error(err);
}

fn f4() -> F4Error {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..10);
    if n % 2 == 0 {
        return F4Error::F2Error(f2());
    }
    return F4Error::F3Error(f3());
}

fn main() {
    let err = f4();

    match err {
        F4Error::F2Error(f2err) => match f2err {
            F2Error::F1Error(f1err) => match f1err {
                F1Error::AccountIsEmpty => println!("Aand it's gone"),
                _ => println!("This line is for bank members only"),
            },
        },
        F4Error::F3Error(f3err) => match f3err {
            F3Error::F1Error(f1err) => match f1err {
                F1Error::AccountIsEmpty => println!("The money in your account didn't do well"),
                _ => println!("This line is for bank members only"),
            },
        },
    }
    println!("Trace {:?}", err);
}
