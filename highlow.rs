
extern mod sqlite;

use core::result;
use core::io;
use core::path;
use core::clone;
use core::to_str;

struct Price {
    date: ~str,
    close: ~str
}

impl clone::Clone for Price {
    fn clone(&self) -> Price {
        Price{date: self.date.clone(), close: self.close.clone()}
    }
}
impl to_str::ToStr for Price {
    fn to_str(&self) -> ~str {
        fmt!("%s: %s", self.date, self.close)
    }
}

fn load(filename: &str) -> ~[~str] {

    let fres = io::file_reader(~path::Path(filename));
    let f = result::unwrap(fres);
    return f.read_lines();
}

fn parse(lines: &[~str]) -> ~[Price] {

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
        prices += [Price{date: date.clone(), close: close.clone()}]
    }

    return prices;
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
        self.database.exec("CREATE TABLE prices (date text, price real)")
    }

    fn write_price(&self, date: ~str, price: float) {
        let res = self.database.exec(fmt!(
            "INSERT INTO prices VALUES ('%s', %f)", date, price));
        if ! res.is_ok() {
            println(fmt!("INSERT error: %s", res.unwrap_err().to_str()));
        }
    }

    fn max_price(&self) -> ~Price {
        return ~Price{date:~"TODO", close:~"TODO"};
        /*
        let st: sqlite::Cursor = self.database.prepare("SELECT date, price FROM prices ORDER BY price DESC LIMIT 1", &None).unwrap();
        st.step_row();
        let date = st.get_text(0);
        let price = st.get_num(1);
        return ~Price{date:date, close:fmt!("%f", price)}
        */
    }

    fn min_price(&self) -> ~Price {
        return ~Price{date:~"TODO", close:~"TODO"};
    }
}

fn save(prices: ~[Price]) {
    println("Start save");

    let res = DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println("Opened");

    match db.create() {
        Err(code) => { println(fmt!("Error creating tables: %s",
                                    code.to_str())); return; }
        _ => {}
    };

    println("Created");

    let mut val: float = 0.0;
    for prices.each |&price| {
        println(price.to_str());

        val = float::from_str(price.close).unwrap();
        db.write_price(price.date.to_str(), val);
    }

}

fn cmd_init() {
    let all_lines: ~[~str] = load("vt.csv");

    // 'tail' because the first line is the column headers, which we
    // don't want.
    let lines: &[~str] = all_lines.tail();

    let prices: ~[Price] = parse(lines);
    println(fmt!("%?", prices));

    save(prices);
}

fn cmd_max() {

    let res = DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.max_price().to_str());
}

fn cmd_min() {
    let res = DB::open("test.db");
    if ! res.is_ok() {
        return;
    }
    let db = res.unwrap();

    println(db.min_price().to_str());
}

fn main() {

    static USAGE: &'static str = "Usage: highlow init|max|min";

    let args: &[~str] = os::args();
    if args.len() != 2 {
        println(USAGE);
        return;
    }

    let cmd: ~str = args[1].clone();

    match cmd {
        ~"init" => cmd_init(),
        ~"max" => cmd_max(),
        ~"min" => cmd_min(),
        other => println(fmt!("Invalid cmd: %s", other))
    }
}

