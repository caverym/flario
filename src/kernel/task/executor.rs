use super::{Task, TaskId};
use alloc::task::Wake;
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::Waker;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::FutureExt;
/*
Asynchronous execute for kernel tasks. Currently used for welcome message and shell.
 */

/// Executor itself. Contains a map of tasks, queue and Wakers/
pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Creates an empty Executor
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Runs the executor's tasks. Never returns.
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    /// Spawn a task, adds a task to the queue.
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    /// Looks for Ready tasks, removes them from queue when complete, pushes them back when executing.
    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        // Loop for every TaskId in queue.
        while let Some(task_id) = task_queue.pop() {
            // Get a reference to a task from its ID.
            let task = match tasks.get_mut(&task_id) {
                Some(t) => t,
                None => continue, // go to next ID in queue.
            };

            // get waker reference based on the task's ID
            let waker = waker_cache.entry(task_id).or_insert_with(|| {
                Waker::from(Arc::new(TaskWaker::new(task_id, task_queue.clone())))
            });

            // Get the Task's context.
            let mut context = Context::from_waker(waker);

            // Poll the task's state with context.
            match task.future.poll_unpin(&mut context) {
                // Task is complete, remove from queue and its waker from cache
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                // Task is not complete, do nothing
                Poll::Pending => {}
            }
        }
    }

    /// halt the thread when all tasks complete.
    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};

        // disable CPU interrupts
        interrupts::disable();
        if self.task_queue.is_empty() {
            // queue is empty, enable interrupts and halt the thread.
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

/// Task waker wakes a task.
struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    /// Create a new TaskWaker with a task's ID and queue.
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> TaskWaker {
        TaskWaker {
            task_id,
            task_queue,
        }
    }

    // wake the task
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
