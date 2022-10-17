mod fundamental_step1;
mod fundamental_step2;
use std::collections::HashMap;

struct Cache<T: Sized> {
    data: HashMap<String, HashMap<String, T>>,
}
