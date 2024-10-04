use kinode_process_lib::{
    await_message, call_init, println,
    vfs::{self, File},
    Address, Message, Response,
};
use std::collections::HashMap;

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let drive = vfs::create_drive(our.package_id(), "hello", Some(1)).unwrap();
    let mut open_files: HashMap<String, File> = HashMap::new();

    let amount_files = 5_000;

    let instant = std::time::Instant::now();
    for i in 0..amount_files {
        let file_name = format!("{}/bench_{}.txt", &drive, i);
        match vfs::open_file(&file_name, true, None) {
            Ok(file) => {
                // match file.write_all("hellO world".as_bytes()) {
                //     Ok(_) => {}
                //     Err(e) => println!("got error while writing to file {}: {e:?}", file_name),
                // }
                open_files.insert(file_name.clone(), file);
            }
            Err(e) => println!("got error while opening file: {e:?}"),
        }
    }
    println!("wrote {} files", amount_files);
    println!("took {:?}ms", instant.elapsed().as_millis());

    // for (file_name, file) in open_files.iter_mut() {
    //     match file.read_to_string() {
    //         Ok(s) => println!("read: {s}, from {file_name}"),
    //         Err(e) => println!("got error while reading file {}: {e:?}", file_name),
    //     }
    //     match file.write_all("hellO world".as_bytes()) {
    //         Ok(_) => {}
    //         Err(e) => println!("got error while writing to file {}: {e:?}", file_name),
    //     }
    // }
}
