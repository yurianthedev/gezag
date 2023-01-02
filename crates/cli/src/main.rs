fn main() {}

mod config {
    use std::path::Path;

    struct CoupledConfig<'a> {
        config_location: &'a Path,
    }
}

mod local {
    use std::path::Path;

    struct Config<'a> {
        index_location: &'a Path,
    }
}
