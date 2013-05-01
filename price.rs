
use core::clone;
use core::to_str;

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
        return false;
    }

    fn is_min(&self, prices: &[~Price]) -> bool{
        return false;
    }
}
