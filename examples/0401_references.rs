use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;
fn main() {
    let mut table = Table::new();
    table.insert("key1".to_string(), vec!["value1_1".to_string(), "value1_2".to_string()]);
    table.insert("key2".to_string(), vec!["value2_1".to_string()]);

    //display(table)
    //assert_eq!(table.len(), 2); -> FAIL, because table is moved into display() and is no longer assigned
    
    display_ref(&table);
    assert_eq!(table.len(), 2);
}

fn display(table: Table) {
    for (key, values) in table.iter() {
        println!("{}: {:?}", key, values);
    }
}

fn display_ref(table: &Table) {
    for (key, values) in table.iter() {
        println!("{}: {:?}", key, values);
    }
}