mod box_m {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct CloseBox<T> {
        pub contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents: contents }
        }
    }
}

fn main() {
    let open_box = box_m::OpenBox{ contents: "pub information"};
    let _closed_box = box_m::CloseBox::new(12);
}
