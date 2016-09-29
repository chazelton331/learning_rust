enum Signal {
    Stop,
    Go,
    Yield,
    Slow,
    Wackadoo,
}

fn process(signal: Signal) {
    match signal {
        Signal::Go                      => println!("GO"   ),
        Signal::Stop                    => println!("STAHP"),
        Signal::Yield | Signal::Slow    => println!("SLOWW"),
        _                               => println!("....."),
    }
}

fn main() {
    let s: Signal = Signal::Stop;
    let g: Signal = Signal::Go;
    let y: Signal = Signal::Yield;
    let o: Signal = Signal::Slow;
    let w: Signal = Signal::Wackadoo;

    process(s);
    process(g);
    process(y);
    process(o);
    process(w);
}
