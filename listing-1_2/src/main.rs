fn main() {
    let penguen_data = "\
        common name, length (cm)
        Little penguen, 33
        Yellow eyed penguen, 65
        Fiordland penguen, 60
        Invalid, data
    ";

    let records = penguen_data.lines();

    for (i, record) in records.enumerate(){
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            println!("{:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(lenght) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, lenght );
        }
        else {
            eprintln!("Error while parsing {:?} as f32, possible not numeric", fields[1]);
        }
    }
}
