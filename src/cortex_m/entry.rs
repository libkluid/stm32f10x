#[macro_export]
macro_rules! entry {
    ($path: path) => {
        #[export_name = "_start"]
        pub unsafe extern "C" fn start() -> ! {
            let entry: unsafe fn() -> ! = $path;
            entry()
        }
    };
}
