struct LargeArray {
    a: Box<[i128; 10000]>,
}

impl LargeArray {
    #[inline(always)]
    fn transfer(mut self) -> Self {
        println!("{:?}", &mut self.a[1] as *mut i128);

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;

        //return the same object
        self
    }
}

fn main() {
    let mut f = LargeArray { a: Box::new([10i128; 10000] )};

    println!("{:?}", &mut f.a[1] as *mut i128);

    let mut f2 = f.transfer();

    println!("{:?}", &mut f2.a[1] as *mut i128);
}