use super::Data;
pub fn list(max: Option<u8>) {
    let v = Data::load_data().expect("Unable to load data");
    match max {
        Some(m) => {
            if v.len() < m as usize {
                for i in v {
                    let s = format!(
                        "Name: {}\nID: {}\nVersion: {}\nApp-Type: {}\n",
                        i.name,
                        i.id,
                        i.version,
                        i.app_type.is_what().unwrap()
                    );
                    println!("{}", s);
                }
            } else {
                let m = m as usize;
                for j in 0..m {
                    let s = format!(
                        "Name: {}\nID: {}\nVersion: {}\nApp-Type: {}\n",
                        v[j].name,
                        v[j].id,
                        v[j].version,
                        v[j].app_type.is_what().unwrap()
                    );
                    println!("{}", s);
                }
            }
        }
        None => {
            for i in v {
                let s = format!(
                    "Name: {}\nID: {}\nVersion: {}\nApp-Type: {}\n",
                    i.name,
                    i.id,
                    i.version,
                    i.app_type.is_what().unwrap()
                );
                println!("{}", s);
            }
        }
    }
}
