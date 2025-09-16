mod tasks;
use tasks::task_1::task_1_1;
use tasks::task_2::task_2_1;

fn main() {
    let name_to_search = "Magokoro"; // &str
    task_1_1::execute_task(&name_to_search.to_string());

    task_2_1::execute_task();
}