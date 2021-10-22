use super::data;

pub fn search(value: &str) {
    let d = data::Data::load_data().expect("Unable to load data");
    for i in d {
        let slice = &i.name[0..3];
        if &value[0..3] == slice {
            let s = format!(
                "Name: {}\nID: {}\nVersion: {}\nApp-Type: {}\n",
                i.name,
                i.id,
                i.version,
                i.app_type.is_what().unwrap()
            );
            println!("{}", s)
        }
    }
}
