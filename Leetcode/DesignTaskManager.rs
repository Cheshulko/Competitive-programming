// https://leetcode.com/problems/design-task-manager

use std::collections::{BTreeSet, HashMap};

type TaskId = i32;
type UserId = i32;
type Priority = i32;

struct TaskManager {
    tasks: BTreeSet<(Priority, TaskId, UserId)>,
    user_by_task: HashMap<TaskId, UserId>,
    task_priority: HashMap<TaskId, Priority>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let (tasks, user_by_task, task_priority) = tasks.into_iter().fold(
            (
                BTreeSet::<(Priority, TaskId, UserId)>::new(),
                HashMap::<TaskId, UserId>::new(),
                HashMap::<TaskId, Priority>::new(),
            ),
            |(mut ts, mut u_by_t, mut tp), v| {
                let &[user_id, task_id, priority] = v.as_slice() else {
                    panic!("Format is correct!")
                };

                u_by_t.insert(task_id, user_id);
                ts.insert((priority, task_id, user_id));
                tp.insert(task_id, priority);

                (ts, u_by_t, tp)
            },
        );

        Self {
            tasks,
            user_by_task,
            task_priority,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.user_by_task.insert(task_id, user_id);
        self.task_priority.insert(task_id, priority);
        self.tasks.insert((priority, task_id, user_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let user_id = self.user_by_task.get(&task_id).copied().unwrap();

        self.rmv(task_id);
        self.add(user_id, task_id, new_priority);
    }

    fn rmv(&mut self, task_id: i32) {
        let user_id = self.user_by_task.get(&task_id).copied().unwrap();
        let priority = self.task_priority.get(&task_id).copied().unwrap();

        self.user_by_task.remove(&task_id);
        self.task_priority.remove(&task_id);
        self.tasks.remove(&(priority, task_id, user_id));
    }

    fn exec_top(&mut self) -> i32 {
        if let Some((_, task_id, user_id)) = self.tasks.pop_last() {
            self.rmv(task_id);

            user_id
        } else {
            -1
        }
    }
}
