// A very simple cooperative scheduler.
// Tasks are functions that take no arguments and return ().
pub type TaskFn = fn();

static mut TASKS: [Option<TaskFn>; 8] = [None; 8];
static mut TASK_COUNT: usize = 0;

/// Adds a task to the scheduler.
pub unsafe fn add_task(task: TaskFn) {
    if TASK_COUNT < TASKS.len() {
        TASKS[TASK_COUNT] = Some(task);
        TASK_COUNT += 1;
    }
}

/// Runs all registered tasks in round-robin fashion forever.
pub unsafe fn run_tasks(){
        for i in 0..TASK_COUNT {
            if let Some(task) = TASKS[i] {
                task();
            }
        }
}
