use super::utils::ChildStorageApi;
use super::ParsedInput;

fn str<'a>(input: &'a [u8]) -> &'a str {
    std::str::from_utf8(input).unwrap()
}
// Input: child1, child2, key, value
pub fn test_set_get_child_storage(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key = input.get(2);
    let value = input.get(3);

    // Get invalid key
    let res = api.rtm_ext_get_allocated_child_storage(child1, key);
    assert_eq!(res, [0u8; 0]);

    // Set key/value
    api.rtm_ext_set_child_storage(child1, key, value);

    // Get valid key
    let res = api.rtm_ext_get_allocated_child_storage(child1, key);
    assert_eq!(res, value);

    println!("{}", str(&res));

    // Get invalid key from invalid child
    let res = api.rtm_ext_get_allocated_child_storage(child2, key);
    assert_eq!(res, [0; 0]);
}

// Input: child1, child2, key, value, offset
pub fn test_get_child_storage_into(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key = input.get(2);
    let value = input.get(3);
    let offset = std::str::from_utf8(input.get(4)).unwrap().parse::<usize>().unwrap();

    // Prepare for comparison, set the min required length
    let empty = if offset > value.len() {
        vec![0u8;0]
    } else {
        vec![0; value.len() - offset]
    };

    // Get invalid key
    let res = api.rtm_ext_get_child_storage_into(child1, key, &empty, offset as u32);
    assert_eq!(res, empty);

    // Set key/value
    api.rtm_ext_set_child_storage(child1, key, value);

    // Get valid key
    let res = api.rtm_ext_get_child_storage_into(child1, key, &empty, offset as u32);
    if offset > value.len() {
        assert_eq!(*res.as_slice(), [0u8;0]);
    } else {
        assert_eq!(*res.as_slice(), value[(offset as usize)..]);
    };

    println!("{}", str(&res));

    // Get invalid key from invalid child
    let res = api.rtm_ext_get_child_storage_into(child2, key, &empty, offset as u32);
    assert_eq!(res, empty);
}

// Input: child1, child2, key, value
pub fn test_exists_child_storage(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key = input.get(2);
    let value = input.get(3);

    // Check invalid key
    let res = api.rtm_ext_exists_child_storage(child1, key);
    assert_eq!(res, 0);

    // Set key/value
    api.rtm_ext_set_child_storage(child1, key, value);

    // Check valid key
    let res = api.rtm_ext_exists_child_storage(child1, key);
    assert_eq!(res, 1);

    println!("true");

    // Check invalid key from invalid child
    let res = api.rtm_ext_exists_child_storage(child2, key);
    assert_eq!(res, 0);
}

// Input: child1, child2, key, value
pub fn test_clear_child_storage(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key = input.get(2);
    let value = input.get(3);

    // Set key/value
    api.rtm_ext_set_child_storage(child1, key, value);
    api.rtm_ext_set_child_storage(child2, key, value);

    // Get valid keys
    let res = api.rtm_ext_get_allocated_child_storage(child1, key);
    assert_eq!(res, value);

    let res = api.rtm_ext_get_allocated_child_storage(child2, key);
    assert_eq!(res, value);

    // Clear key
    api.rtm_ext_clear_child_storage(child1, key);

    // Get invalid key
    let res = api.rtm_ext_get_allocated_child_storage(child1, key);
    assert_eq!(res, [0; 0]);

    // Get valid key from other child
    let res = api.rtm_ext_get_allocated_child_storage(child2, key);
    assert_eq!(res, value);
}

// Input: prefix, child1, child2, key1, value1, key2, value2
pub fn test_clear_child_prefix(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let prefix = input.get(0);
    let child1 = input.get(1);
    let child2 = input.get(2);
    let key1 = input.get(3);
    let value1 = input.get(4);
    let key2 = input.get(5);
    let value2 = input.get(6);

    // Set keys/values for each child
    api.rtm_ext_set_child_storage(child1, key1, value1);
    api.rtm_ext_set_child_storage(child1, key2, value2);
    api.rtm_ext_set_child_storage(child2, key1, value1);
    api.rtm_ext_set_child_storage(child2, key2, value2);

    // Clear keys with specified prefix
    api.rtm_ext_clear_child_prefix(child2, prefix);

    // Check deletions (only keys from `child2` are got deleted)
    let res = api.rtm_ext_get_allocated_child_storage(child1, key1);
    assert_eq!(res, value1);

    let res = api.rtm_ext_get_allocated_child_storage(child1, key2);
    assert_eq!(res, value2);

    let res = api.rtm_ext_get_allocated_child_storage(child2, key1);
    if key1.starts_with(prefix) {
        assert_eq!(res, [0; 0]);
        println!("Key `{}` was deleted", str(key1));
    } else {
        assert_eq!(res, value1);
        println!("Key `{}` remains", str(key1));
    }

    let res = api.rtm_ext_get_allocated_child_storage(child2, key2);
    if key2.starts_with(prefix) {
        assert_eq!(res, [0; 0]);
        println!("Key `{}` was deleted", str(key2));
    } else {
        assert_eq!(res, value2);
        println!("Key `{}` remains", str(key2));
    }
}

// Input: child1, child2, key, value
pub fn test_kill_child_storage(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key = input.get(2);
    let value = input.get(3);

    // Set key/values
    api.rtm_ext_set_child_storage(child1, key, value);
    api.rtm_ext_set_child_storage(child2, key, value);

    // Kill the child
    api.rtm_ext_kill_child_storage(child1);

    // Get invalid key
    let res = api.rtm_ext_get_allocated_child_storage(child1, key);
    assert_eq!(res, [0; 0]);

    // Get valid key from other child
    let res = api.rtm_ext_get_allocated_child_storage(child2, key);
    assert_eq!(res, value);
}

// Input: child1, child2, key1, value1, key2, value2
pub fn test_child_storage_root(input: ParsedInput) {
    let mut api = ChildStorageApi::new();

    let child1 = input.get(0);
    let child2 = input.get(1);
    let key1 = input.get(2);
    let value1 = input.get(3);
    let key2 = input.get(4);
    let value2 = input.get(5);

    api.rtm_ext_set_child_storage(child1, key1, value1);

    // Test multiple key/value pairs
    api.rtm_ext_set_child_storage(child2, key1, value1);
    api.rtm_ext_set_child_storage(child2, key2, value2);

    let child_root1 = api.rtm_ext_child_storage_root(child1);
    let child_root2 = api.rtm_ext_child_storage_root(child2);
    println!("{},{}", hex::encode(child_root1), hex::encode(child_root2));
}