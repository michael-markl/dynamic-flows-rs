use std::{cmp::Reverse, collections::HashMap};

use priority_queue::PriorityQueue;

use crate::num::Num;

/// Describes a future change in the outflow of an edge.
#[derive(PartialEq, Debug)]
pub struct ChangeEvent<T: Num> {
    pub time: T,
    pub value: ChangeEventValue<T>,
}

#[derive(PartialEq, Debug)]
pub struct ChangeEventValue<T: Num> {
    pub new_outflow_map: HashMap<usize, T>,
    pub values_sum: T,
}

pub struct DepletionQueue<T: Num> {
    /// Contains a priority queue of all edges whose queues will depete in the future -- sorted by the time of depletion.
    depletions: PriorityQueue<usize, Reverse<T>>,
    /// If the depletion of  the queue of an edge, results in a change in the outflow of that edge, then the time of that change is stored here.
    change_times_after_a_depletion: PriorityQueue<usize, Reverse<T>>,
    new_outflow: HashMap<usize, ChangeEventValue<T>>,
}

impl<T: Num> DepletionQueue<T> {
    pub fn new() -> Self {
        Self {
            depletions: PriorityQueue::new(),
            change_times_after_a_depletion: PriorityQueue::new(),
            new_outflow: HashMap::new(),
        }
    }

    pub fn set(
        &mut self,
        edge: usize,
        depletion_time: T,
        outflow_change_event: Option<ChangeEvent<T>>,
    ) {
        debug_assert!(depletion_time > -T::INFINITY);
        self.depletions.push(edge, Reverse(depletion_time));

        if let Some(change_event) = outflow_change_event {
            self.new_outflow.insert(edge, change_event.value);
            self.change_times_after_a_depletion
                .push(edge, Reverse(change_event.time));
        } else if self.change_times_after_a_depletion.remove(&edge).is_some() {
            self.new_outflow.remove(&edge);
        }
    }

    pub fn remove(&mut self, edge: usize) {
        self.depletions.remove(&edge);
        self.change_times_after_a_depletion.remove(&edge);
        self.new_outflow.remove(&edge);
    }

    pub fn pop_by_depletion(&mut self) -> Option<(usize, T, Option<ChangeEvent<T>>)> {
        let (edge, Reverse(depletion_time)) = self.depletions.pop()?;

        let change_event =
            self.change_times_after_a_depletion
                .remove(&edge)
                .map(|(_, Reverse(change_time))| {
                    let change_event_val = self.new_outflow.remove(&edge).unwrap();
                    ChangeEvent {
                        time: change_time,
                        value: change_event_val,
                    }
                });
        Some((edge, depletion_time, change_event))
    }

    pub fn min_depletion_time(&self) -> Option<&T> {
        self.depletions.peek().map(|(_, Reverse(time))| time)
    }

    pub fn min_change_time(&self) -> Option<&T> {
        return self
            .change_times_after_a_depletion
            .peek()
            .map(|(_, Reverse(time))| time);
    }
}

#[cfg(test)]
mod tests {
    use crate::float::F64;

    use super::DepletionQueue;

    #[test]
    fn test_depletion_queue() {
        let mut q: DepletionQueue<F64> = DepletionQueue::new();
        q.set(1, 1.0.into(), None);
        assert_eq!(q.min_depletion_time(), Some(&1.0.into()));
        assert_eq!(q.min_change_time(), None);
        assert_eq!(q.pop_by_depletion(), Some((1, 1.0.into(), None)));
        assert_eq!(q.pop_by_depletion(), None);
    }
}
