#![feature(test)]

extern crate test;

type Price = i32;
type Size = u32;
type Meta = u128;

extern crate rand;

type Inner = Vec<(Price, Vec<(Size, Meta)>)>;

struct PriceList {
    inner: Inner
}

struct Split<'a> {
    left: Vec<&'a (Price, Vec<(Size, Meta)>)>,
    right: Vec<&'a (Price, Vec<(Size, Meta)>)>,
}

impl PriceList {
    pub fn new() -> PriceList {
        PriceList {
            inner: vec![]
        }
    }

    pub fn add(&mut self, price: Price, size_meta: (Size, Meta)) {

        match self.inner.binary_search_by_key(&price, |&(ref a, _)| *a) {
            Ok(index) => {
                self.inner.get_mut(index).map(|x| x.1.push(size_meta));
                ()
            },
            Err(index) => self.inner.insert(index, (price, vec![size_meta]))
        };
    }

    fn get_prices(&self) -> Vec<Price> {

        self.inner.iter().cloned().map(|x| x.0).collect()

    }

//    fn split(&self, price: Price, size: Size) -> impl Iterator<Item=&[(Price, Vec<(Size, Meta)>)]>{
//
//        self.inner.split(
//            move |x|
//                x.0 <= price &&
//                x.1.iter().fold(0, |sum, val| sum + val.0) <= size)
//
//    }

    pub fn split(&self, price: Price, size: Size) -> Split {

        let mut left = vec![];
        let mut right = vec![];

        for x in &self.inner {
            if x.0 <= price &&
                x.1.iter().fold(0, |sum, val| sum + val.0) <= size {
                left.push(x);
            } else {
                right.push(x)
            }
        }

        return Split {
            left: left,
            right: right,
        }

    }

}


#[cfg(test)]
mod tests {

    use test::Bencher;
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

    #[bench]
    fn bench_price_list_add(b: &mut Bencher) {

        let mut inner: Inner = vec![];
        for _i in 0..1000 {
            inner.push((rand::random(), gen_size_meta(100)));
        }


        b.iter(|| {

            let mut price_list = PriceList::new();

            for i in &inner {
                for j in &i.1 {
                    price_list.add(i.0, *j);
                }
            }
        });
    }

    #[test]
    fn check_split() {

        let mut lst = PriceList::new();

        lst.add(100, (100, 0));
        lst.add(100, (50, 0));
        lst.add(200, (200, 0));

        let splitted = lst.split(200, 150);

        assert_eq!(vec![&(100, vec![(100, 0), (50, 0)])], splitted.left);
        assert_eq!(vec![&(200, vec![(200, 0)])], splitted.right);

    }

    #[bench]
    fn bench_split_random_values(b: &mut Bencher) {
        let mut inner: Inner = vec![];
        for _i in 0..1000 {
            inner.push((rand::random(), gen_size_meta(100)));
        }

        let mut price_list = PriceList::new();

        for i in &inner {
            for j in &i.1 {
                price_list.add(i.0, *j);
            }
        }

        b.iter(|| {
            price_list.split(rand::random(), rand::random());
        });
    }
}

