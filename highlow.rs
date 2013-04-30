
extern mod sqlite;

use core::result;
use core::io;
use core::path;
use core::clone;
use core::to_str;

struct Price {
    symbol: ~str,
    date: ~str,
    price: ~str
}

impl clone::Clone for Price {
    fn clone(&self) -> Price {
        Price{
            symbol: self.symbol.clone(),
            date: self.date.clone(),
            price: self.price.clone()}
    }
}
impl to_str::ToStr for Price {
    fn to_str(&self) -> ~str {
        fmt!("%s %s: %s", self.symbol, self.date, self.price)
    }
}

struct DB {
    filename: ~str,
    database: sqlite::Database
}

impl DB {

    fn open(name: &str) -> Result<DB, ~str> {

        match sqlite::open(name) {
            // Here 'name' is borrowed. Calling .to_str() makes a new
            // owned string, which the object will hold.
            Ok(db) => Ok(DB{filename: name.to_str(), database: db}),
            Err(e) => {
                println(fmt!("Error opening test.db: %?", e));
                Err(e.to_str())
            }
        }
    }

    fn create(&self) -> Result<sqlite::ResultCode, sqlite::ResultCode>  {
        self.database.exec(
            "CREATE TABLE prices (symbol text, date text, price real)")
    }

    fn write_price(&self, symbol: &str, date: &str, price: float) {
        let res = self.database.exec(fmt!(
            "INSERT INTO prices VALUES ('%s', '%s', %f)", symbol, date, price));
        if ! res.is_ok() {
            println(fmt!("INSERT error: %s", res.unwrap_err().to_str()));
        }
    }

    fn _min_max(&self, min_max: &str, symbol: &str) -> ~Price {

        let extra =
            match min_max {
                "MAX" => "DESC",
                _ => ""
            };
        let sql = fmt!(
            "SELECT symbol, date, price FROM prices WHERE symbol = '%s' ORDER BY price %s LIMIT 1",
            symbol,
            extra);

        let st: sqlite::Cursor = self.database.prepare(sql, &None).unwrap();
        st.step();
        let symbol = st.get_text(0);
        let date = st.get_text(1);
        let price = st.get_num(2);
        return ~Price{symbol: symbol, date:date, price:fmt!("%f", price)}
    }

    fn max_price(&self, symbol: &str) -> ~Price {
        self._min_max("MAX", symbol)
    }

    fn min_price(&self, symbol: &str) -> ~Price {
        self._min_max("MIN", symbol)
    }
}

fn load(filename: &str) -> ~[~str] {

    let fres = io::file_reader(~path::Path(filename));
    let f = result::unwrap(fres);
    return f.read_lines();
}

fn parse(symbol: &str, lines: &[~str]) -> ~[Price] {

    let mut date = ~"";
    let mut close = ~"";
    let mut i = 0;
    let mut prices: ~[Price] = ~[];

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
        prices += [Price{
            symbol: symbol.to_str(),
            date: date.clone(),
            price: close.clone()}
        ]
    }

    return prices;
}

fn save(prices: ~[Price]) {

    let dbname = "test.db";

    let is_created: bool = os::path_exists(~path::Path(dbname));

    let res = DB::open(dbname);
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

    let prices: ~[Price] = parse(symbol, lines);

    save(prices);
}

fn cmd_max(symbol: &str) {

    let res = DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.max_price(symbol).to_str());
}

fn cmd_min(symbol: &str) {
    let res = DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.min_price(symbol).to_str());
}

fn main() {

    static USAGE: &'static str = "Usage: highlow load <symbol> <filename>|max <symbol>|min <symbol>";

    let mut args = os::args();
    args.shift();

    match args {
        [~"load", symbol, filename] => cmd_load(symbol, filename),
        [~"max", symbol] => cmd_max(symbol),
        [~"min", symbol] => cmd_min(symbol),
        [other] => { println(fmt!("Invalid cmd: %s", other)); println(USAGE); }
        _ => println(USAGE)
    }
}
