
use core::result;
use core::io;
use core::path;

mod db;
mod price;

fn load(filename: &str) -> ~[~str] {

    let fres = io::file_reader(~path::Path(filename));
    let f = result::unwrap(fres);
    return f.read_lines();
}

fn parse(symbol: &str, lines: &[~str]) -> ~[price::Price] {

    let mut date = ~"";
    let mut close = ~"";
    let mut i = 0;
    let mut prices: ~[price::Price] = ~[];

    for lines.each |&line| {

        i = 0;
        for line.each_split_char(',') |piece| {
            if i == 0 {
                date = piece.to_str();
            } else if i == 4 {
                close = piece.to_str();
            }
            i += 1;
        }
        prices += [price::Price{
            symbol: symbol.to_str(),
            date: date.clone(),
            price: close.clone()}
        ]
    }

    return prices;
}

fn save(prices: ~[price::Price]) {

    let dbname = "test.db";

    let is_created: bool = os::path_exists(~path::Path(dbname));

    let res = db::DB::open(dbname);
    if ! res.is_ok() {
        return;
    }

    let db = res.unwrap();

    if !is_created {
        match db.create() {
            Err(code) => { println(fmt!("Error creating tables: %s",
                                        code.to_str())); return; }
            _ => {}
        };
    }

    let mut val: float = 0.0;
    for prices.each |&price| {
        println(price.to_str());

        val = float::from_str(price.price).unwrap();
        db.write_price(price.symbol.to_str(), price.date.to_str(), val);
    }

}

fn cmd_load(symbol: &str, filename: &str) {
    let all_lines: ~[~str] = load(filename);

    // 'tail' because the first line is the column headers, which we
    // don't want.
    let lines: &[~str] = all_lines.tail();

    let prices: ~[price::Price] = parse(symbol, lines);

    save(prices);
}

fn cmd_max(symbol: &str) {

    let res = db::DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.max_price(symbol).to_str());
}

fn cmd_min(symbol: &str) {
    let res = db::DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.min_price(symbol).to_str());
}

fn cmd_trade(symbol: &str) {

    let res = db::DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let d = res.unwrap();

    let year = d.load_year(symbol);

    let unit = 1000.0;
    let mut position = 0.0;
    let mut money = 10000.0;

    for d.prices_after(year[year.len()-1]).each |next_price| {

        if next_price.is_max(year) {
            if position <= 0.0 {
                loop;
            }
            println(fmt!("SELL: %?", next_price));

            let p = float::from_str(next_price.price).unwrap();
            let num_sold = unit / p;
            position -= num_sold;
            money += num_sold * p;

        } else if next_price.is_min(year) {
            if money <= 0.0 {
                loop;
            }
            print(fmt!("%s: BUY at %s - ", next_price.date, next_price.price));

            let p = float::from_str(next_price.price).unwrap();
            let num_bought = unit / p;
            position += num_bought;
            money -= num_bought * p;
            println(fmt!("$%f, %f shares", money, position));
        }
    }
}

fn main() {

    static USAGE: &'static str = "Usage: highlow load <symbol> <filename>|max <symbol>|min <symbol>|trade <symbol>";

    let mut args = os::args();
    args.shift();

    match args {
        [~"load", symbol, filename] => cmd_load(symbol, filename),
        [~"max", symbol] => cmd_max(symbol),
        [~"min", symbol] => cmd_min(symbol),
        [~"trade", symbol] => cmd_trade(symbol),
        [other] => { println(fmt!("Invalid cmd: %s", other)); println(USAGE); }
        _ => println(USAGE)
    }
}
