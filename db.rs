
extern mod sqlite;
use core::clone;
use core::to_str;

struct DB {
    filename: ~str,
    database: sqlite::Database
}

pub impl DB {

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

    fn load_year(&self, symbol: &str) -> @[~Price] {

        /*
          252 is about the number of trading days in a year,
          from http://usequities.nyx.com/sites/usequities.nyx.com/files/tradeday_13.pdf
          Accuracy isn't especially important here, it's just seed data
        */

        let SQL = fmt!("SELECT date, price FROM prices WHERE symbol = '%s' ORDER BY date LIMIT 252", symbol);

        let st: sqlite::Cursor = self.database.prepare(SQL, &None).unwrap();
        let mut date;
        let mut priceVal;
        let mut priceObj;

        let mut result = @[];

        while st.step() == sqlite::SQLITE_ROW {
            date = st.get_text(0);
            priceVal = st.get_text(1);
            priceObj = ~Price{
                symbol: symbol.to_str(),
                date: date,
                price: priceVal};

            result += [priceObj];
        }

        return result;
    }

    fn prices_after(&self, after_this: &Price) -> ~[~Price] {

        let SQL = fmt!("SELECT date, price FROM prices WHERE symbol = '%s' AND date > '%s' ORDER BY date", after_this.symbol, after_this.date);

        let st: sqlite::Cursor = self.database.prepare(SQL, &None).unwrap();
        let mut date;
        let mut priceVal;
        let mut priceObj;

        let mut result = ~[];

        while st.step() == sqlite::SQLITE_ROW {
            date = st.get_text(0);
            priceVal = st.get_text(1);
            priceObj = ~Price{
                symbol: after_this.symbol.to_str(),
                date: date,
                price: priceVal};

            result += [priceObj];
        }

        return result;
    }
}

pub struct Price {
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

pub impl Price {

    fn is_max(&self, prices: &[~Price]) -> bool {

        let p = float::from_str(self.price).unwrap();
        !vec::any(prices, |contender| { float::from_str(contender.price).unwrap() > p })
    }

    fn is_min(&self, prices: &[~Price]) -> bool{
        let p = float::from_str(self.price).unwrap();
        !vec::any(prices, |contender| { float::from_str(contender.price).unwrap() < p })
    }
}

