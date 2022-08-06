use candid::{candid_method, export_service};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};

#[cfg(test)]
mod test;

type MyStrings = Vec<String>;

#[init]
fn init() {
    stable_memory_init(true, 0);

    // create the stable variable
    s! { MyStrings = MyStrings::new() };
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_memory_pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    stable_memory_post_upgrade(0);
}

#[query]
#[candid_method(query)]
fn get_my_strings() -> MyStrings {
    s!(MyStrings)
}

#[update]
#[candid_method(update)]
fn add_my_string(entry: String) {
    let mut my_strings = s!(MyStrings);
    my_strings.push(entry);

    s! { MyStrings = my_strings };
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
