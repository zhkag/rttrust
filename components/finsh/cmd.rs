use kernel::println;
use kernel::object::ObjectClassType;
use kernel::macros::init_export;

#[init_export("6")]
fn list_thread() {
    let system = kernel::system!();
    if let Some(information) = system.get_object_information(ObjectClassType::Thread){
        for node in information.object_list.iter_mut() {
            let object = system.list_to_object(node);
            println!("object {}",object);
        }
    }
    if let Some(object) =  system.object_find("main",ObjectClassType::Thread){
        println!("name {}",object);
    }
}
