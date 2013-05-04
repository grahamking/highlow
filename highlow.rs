
use core::result;
use core::io;
use core::path;

mod db;

fn load(filename: &str) -> ~[~str] {

    let fres = io::file_reader(~path::Path(filename));
    let f = result::unwrap(fres);
    return f.read_lines();
}

fn parse(symbol: &str, lines: &[~str]) -> ~[db::Price] {

    let mut date = ~"";
    let mut close = ~"";
    let mut i = 0;
    let mut prices: ~[db::Price] = ~[];

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
        prices += [db::Price{
            symbol: symbol.to_str(),
            date: date.clone(),
            price: close.clone()}
        ]
    }

    return prices;
}

fn save(prices: ~[db::Price]) {

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
        print("."); // Show progress

        val = float::from_str(price.price).unwrap();
        db.write_price(price.symbol.to_str(), price.date.to_str(), val);
    }
    println("");

}

fn cmd_load(symbol: &str, filename: &str) {
    let all_lines: ~[~str] = load(filename);

    // 'tail' because the first line is the column headers, which we
    // don't want.
    let lines: &[~str] = all_lines.tail();

    let prices: ~[db::Price] = parse(symbol, lines);

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

fn cmd_trade(symbol: &str, amount: float) {

    let res = db::DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let d = res.unwrap();

    let unit = 1000.0;
    let mut position = 0.0;
    let mut money = amount;

    let mut year: @[~db::Price] = d.load_year(symbol);

    for d.prices_after(year[year.len()-1]).each |&next_price| {

        if next_price.is_max(year) {
            if position < 1.0 {
                loop;
            }
            print(fmt!("%s: SELL at %s - ", next_price.date, next_price.price));

            let p = float::from_str(next_price.price).unwrap();
            let num_sold = cmp::min(unit / p, position);
            position -= num_sold;
            money += num_sold * p;
            println(fmt!("$%f, %f shares", money, position));

        } else if next_price.is_min(year) {
            if money < unit {
                loop;
            }
            print(fmt!("%s: BUY at %s - ", next_price.date, next_price.price));

            let p = float::from_str(next_price.price).unwrap();
            let num_bought = unit / p;
            position += num_bought;
            money -= num_bought * p;
            println(fmt!("$%f, %f shares", money, position));
        }

        let mut owned: ~[~db::Price] = year.to_owned();
        owned.shift();
        owned = vec::append_one(owned, next_price);
        year = at_vec::from_owned(owned)
    }
}

fn main() {

    static USAGE: &'static str = "Usage: highlow load <symbol> <filename>|max <symbol>|min <symbol>|trade <symbol> <amount>";

    let mut args = os::args();
    args.shift();

    match args {
        [~"load", symbol, filename] => cmd_load(symbol, filename),
        [~"max", symbol] => cmd_max(symbol),
        [~"min", symbol] => cmd_min(symbol),
        [~"trade", symbol, amount] => cmd_trade(symbol, float::from_str(amount).unwrap()),
        [other] => { println(fmt!("Invalid cmd: %s", other)); println(USAGE); }
        _ => println(USAGE)
    }
}
