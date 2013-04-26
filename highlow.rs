
//extern mod sqlite;
use core::result;
use core::io;
use core::path;

struct Price {
    date: ~str,
    close: ~str
}

fn load(filename: &str) -> ~[~str] {

    let fres = io::file_reader(~path::Path(filename));
    let f = result::unwrap(fres);

    return f.read_lines();
}

fn parse(lines: ~[~str]) -> ~[Price] {

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

            prices += [Price{date: date.to_str(), close: close.to_str()}]
        }
    }

    return prices;
}

fn main() {

    let lines = load("vt.csv");
    //println(fmt!("%?", lines));

    let prices = parse(lines);
    println(fmt!("%?", prices));

    /*
    let database =
        match sqlite::open("test.db") {
            Ok(db) => db,
            Err(e) => {
                println(fmt!("Error opening test.db: %?", e));
                return;
            }
        };
    let mut r = database.exec("CREATE TABLE gk_test (name text, age int)");
    println(fmt!("Create OK? %?", r.is_ok()));

    r = database.exec("INSERT INTO gk_test VALUES ('Graham', 36)");
    println(fmt!("Insert OK? %?", r.is_ok()));
    */
}
