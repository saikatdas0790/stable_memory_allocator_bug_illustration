use candid::{candid_method, export_service};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_memory::{
    collections::{hash_map::SHashMap, vec::SVec},
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
    utils::ic_types::SPrincipal,
};

#[cfg(test)]
mod test;

type MyPrincipalVec = SVec<SPrincipal>;
type MyPrincipalMap = SHashMap<SPrincipal, SPrincipal>;
type MyOwnStringMap = SHashMap<String, String>;

#[init]
fn init() {
    stable_memory_init(true, 0);

    // create the stable variable
    s! { MyPrincipalVec = MyPrincipalVec::new() };
    s! { MyPrincipalMap = MyPrincipalMap::new_with_capacity(200_000) };
    s! { MyOwnStringMap = MyOwnStringMap::new_with_capacity(200_000) };
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
fn get_my_principal_vec() -> Vec<SPrincipal> {
    let entire_vector = s!(MyPrincipalVec);

    let mut result: Vec<SPrincipal> = Vec::new();

    for index in 0..entire_vector.len() {
        result.push(entire_vector.get_cloned(index).unwrap());
    }

    result
}

#[query]
#[candid_method(query)]
fn get_my_principal_map() -> Option<SPrincipal> {
    let entire_map = s!(MyPrincipalMap);

    entire_map.get_cloned(&SPrincipal(ic_cdk::caller()))
}

#[update]
#[candid_method(update)]
fn add_my_principal() {
    let my_principal = SPrincipal(ic_cdk::caller());

    let mut my_principal_vector = s!(MyPrincipalVec);
    my_principal_vector.push(&my_principal);
    s! { MyPrincipalVec = my_principal_vector };

    let mut my_principal_map = s!(MyPrincipalMap);
    my_principal_map.insert(my_principal, &my_principal);
    s! {MyPrincipalMap = my_principal_map};
}

#[query]
#[candid_method(query)]
fn get_my_string_map(some_text: String) -> Option<String> {
    let entire_map = s!(MyOwnStringMap);

    entire_map.get_cloned(&some_text)
}

#[update]
#[candid_method(update)]
fn add_my_string(my_string: String) {
    let mut my_string_map = s!(MyOwnStringMap);
    my_string_map.insert(my_string.clone(), &my_string);
    s! {MyOwnStringMap = my_string_map};
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
