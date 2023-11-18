use std::time::SystemTime;

/// Represents a task with a title, description, and completion status.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed_at: Option<SystemTime>,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: "".to_string(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        match self.completed_at {
            None => self.completed_at = Some(SystemTime::now()),
            x => self.completed_at = x,
        }
    }
    pub fn uncomplete(&mut self) {
        self.completed_at = None
    }
}

/// A trait representing a collection of tasks.
///
/// The `Collection` trait provides methods for managing tasks in a collection.
/// It is a generic trait, allowing different implementations to work with different types of tasks.
///
/// # Examples
///
/// ```rust
/// use seybio_task_manager::{Collection, Task};
///
/// struct MyCollection {
///     tasks: Vec<Task>,
/// }
///
/// impl Collection for MyCollection {
///     type Task = Task;
///
///     fn new() -> Self {
///         MyCollection {
///             tasks: Vec::new(),
///         }
///     }
///
///     fn add_task(&mut self, task: Self::Task) {
///         self.tasks.push(task);
///     }
///
///     fn remove_task(&mut self, task: Self::Task) {
///         if let Some(index) = self.tasks.iter().position(|t| t == &task) {
///             self.tasks.remove(index);
///         }
///     }
/// }
/// ```
pub trait Collection {
    type Task;
    fn new() -> Self;
    fn add_task(&mut self, task: Self::Task);
    fn remove_task(&mut self, task: Self::Task);
}

/// TaskCollection struct represents a collection of tasks.
///
/// # Fields
/// - `tasks`: A vector of `Task` objects.
///
/// # Example
/// ```
/// use serde::{Serialize, Deserialize};
/// use seybio_task_manager::Task;
///
/// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// pub struct TaskCollection {
///     pub tasks: Vec<Task>,
/// }
/// ```
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct TaskCollection {
    pub tasks: Vec<Task>,
}

impl Collection for TaskCollection {
    type Task = Task;

    fn new() -> Self {
        Self { tasks: vec![] }
    }
    fn add_task(&mut self, task: Self::Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, task: Self::Task) {
        self.tasks.retain(|t| *t != task);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Collection, Task, TaskCollection};

    #[test]
    fn it_inits_task() {
        let expected_title = "new Task";
        let task = Task::new(expected_title);
        assert_eq!(task.title, expected_title.to_string());
        assert_eq!(task.description, "".to_string());
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_completes_a_task() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        assert_ne!(task.completed_at, None)
    }

    #[test]
    fn it_uncompletes_a_task() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        task.uncomplete();
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_does_not_set_completes_mulitple_times() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        let expected_completed = task.completed_at;
        task.complete();
        assert_eq!(task.completed_at, expected_completed)
    }

    #[test]
    fn inits_empty_task_collection() {
        let collection = TaskCollection::new();
        assert!(collection.tasks.is_empty())
    }

    #[test]
    fn test_get_all_tasks_from_collection() {
        let mut collection = TaskCollection::new();
        let task1 = Task::new("task 1");

        collection.add_task(task1.clone());
        assert_eq!(collection.tasks, vec![task1])
    }
}
