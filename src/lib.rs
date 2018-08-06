#![feature(test)]
extern crate test;
extern crate rand;

pub type Price = i32;
pub type Size = u32;
pub type Meta = u128;

pub type Inner = Vec<(Price, Vec<(Size, Meta)>)>;

pub struct PriceList {
    inner: Inner
}

impl PriceList {
    pub fn new() -> PriceList {
        PriceList {
            inner: vec![]
        }
    }

    pub fn with_capacity(n: usize) -> PriceList {
        PriceList {
            inner: Vec::with_capacity(n)
        }
    }

    /// # Examples
    ///
    /// ```
    /// # extern crate test_work_price_list;
    /// # use test_work_price_list::{PriceList};
    /// let mut lst = PriceList::new();
    /// lst.add(1, (10, 2));
    /// ```
    pub fn add(&mut self, price: Price, size_meta: (Size, Meta)) {

        match self.inner.binary_search_by_key(&price, move |&(ref a, _)| *a) {
            Ok(index) => {
                self.inner.get_mut(index).map(|x| x.1.push(size_meta));
                ()
            },
            Err(index) => self.inner.insert(index, (price, vec![size_meta]))
        };
    }

    fn add_size_meta_list(&mut self, price: Price, size_meta: Vec<(Size, Meta)>) {
        match self.inner.binary_search_by_key(&price, |&(ref a, _)| *a) {
            Ok(index) => {
                self.inner.get_mut(index).map(|x| x.1 = size_meta);
                ()
            },
            Err(index) => self.inner.insert(index, (price, size_meta))
        };
    }

    fn get_prices(&self) -> Vec<Price> {
        self.inner.iter().map(|x| x.0).collect()
    }

    /// splits struct into 2, by price and size
    pub fn split(&mut self, price: Price, mut size: u128) -> PriceList {

        if self.inner.len() == 0 {
            return PriceList::new();
        }

        let index = match self.inner.binary_search_by_key(&price, move |&(ref a, _)| *a) {
            Ok(index) => index,
            Err(index) => index
        };

        let mut cur = PriceList::with_capacity(index + 1);
        let mut out = PriceList::with_capacity(1 + self.inner.len() - index);

        for i in &self.inner {

            if i.0 <= price {
                for j in &i.1 {
                    if j.0 as u128 <= size {
                        cur.add(i.0, *j);
                        size -= j.0 as u128
                    } else {
                        out.add(i.0, *j)
                    }
                }
            } else {
                out.add_size_meta_list(i.0, i.1.clone())
            }
        }

        cur.inner.shrink_to_fit();
        self.inner = cur.inner;
        return out;

    }

    fn get_size_sum(&self) -> u128 {
        self.inner.iter().map(|x| x.1.iter().fold(0u128, |a, b| a + b.0 as u128)).sum()
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::{thread_rng, Rng};

    fn gen_size_meta(times: u32) -> Vec<(Size, Meta)> {
        (0..times).map(|_| (rand::random(), rand::random::<u64>() as u128)).collect()
    }

    #[test]
    fn price_list_add() {
        let mut lst = PriceList::new();
        let size_meta  = (0, 0);

        lst.add(10, size_meta);
        lst.add(1, size_meta);
        lst.add(5, size_meta);
        lst.add(5, size_meta);

        assert_eq!(vec![1,5,10], lst.get_prices());

    }

    #[test]
    fn price_list_add_with_random_values() {

        let mut lst = PriceList::new();

        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(100, 1000);

        let random_values = (0..n).map(|_| {
            rand::random()
        }).collect::<Vec<i32>>();

//        println!("random_values {:?}", &random_values);
        for i in random_values {
            let size_meta_n: u32 = rng.gen_range(10, 100);

            for _j in 0..size_meta_n {
                lst.add(i, (rand::random(), rand::random::<u32>() as Meta));
            }
        }

        let mut cloned = lst.get_prices();
        cloned.sort();
        cloned.dedup();
        // все должно быть отсортировано и не должно быть дублей
        assert_eq!(lst.get_prices(), cloned)
    }



    #[test]
    fn check_split() {

        let mut lst = PriceList::new();

        lst.add(100, (100, 0));
        lst.add(100, (50, 0));
        lst.add(200, (200, 0));

        let splitted = lst.split(100, 100);

        assert_eq!(vec![(100, vec![(100, 0)])], lst.inner);
        assert_eq!(vec![(100, vec![(50, 0)]), (200, vec![(200, 0)])], splitted.inner);

    }

    //  разделение на равные по size доли;
    #[test]
    fn check_split_half_size() {

        let mut lst = PriceList::new();
        let mut rng = thread_rng();

        let mut size: u128 = 0;
        for _ in 0..10 {
            for _ in 0..10 {
                let _size = rand::random::<u32>();
                size += _size as u128;
                lst.add(rand::random(), (10, 0))
            }
        }

        // 10 элементов, по 10 в каждом, при size=10
        let half_size = 10 * 10 * 10 / 2;
        let right = lst.split(std::i32::MAX, half_size);

        assert_eq!(lst.get_size_sum(), right.get_size_sum());

    }


}