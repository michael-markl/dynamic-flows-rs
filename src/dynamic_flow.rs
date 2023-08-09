use std::{
    cmp::{max, min, Reverse},
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter,
};

use num_traits::abs;
use priority_queue::PriorityQueue;

use crate::{
    depletion_queue::{ChangeEvent, ChangeEventValue, DepletionQueue},
    num::{Num, Sum},
    piecewise_constant::PiecewiseConstant,
    piecewise_linear::PiecewiseLinear,
    points,
};

#[derive(Clone, Debug)]
struct FlowRatesCollectionItem<T: Num> {
    time: T,
    values: HashMap<usize, T>,
}

#[derive(Clone, Debug)]
pub struct FlowRatesCollection<T: Num> {
    function_by_comm: HashMap<usize, PiecewiseConstant<T>>,
    accumulative: PiecewiseLinear<T>,
    queue: VecDeque<FlowRatesCollectionItem<T>>,
}
impl<T: Num> FlowRatesCollection<T> {
    fn new() -> Self {
        FlowRatesCollection {
            function_by_comm: HashMap::new(),
            accumulative: PiecewiseLinear::new(
                (-T::INFINITY, T::INFINITY),
                T::ZERO,
                T::ZERO,
                points!((T::ZERO, T::ZERO)),
            ),
            queue: VecDeque::new(),
        }
    }

    pub fn get_values_at_time(&mut self, time: T) -> Option<&HashMap<usize, T>> {
        match self.queue.front() {
            None => None,
            Some(item) => {
                if item.time > time {
                    panic!("The desired time is not available anymore.")
                } else {
                    while self.queue.get(1).is_some_and(|next| next.time <= time) {
                        self.queue.pop_front();
                    }
                    return Some(&self.queue.front().unwrap().values);
                }
            }
        }
    }

    fn extend(&mut self, from_time: T, values_map: HashMap<usize, T>, values_sum: T) {
        match self.queue.back() {
            None => {
                for (i, value) in values_map.iter() {
                    let mut new_fn =
                        PiecewiseConstant::new((T::ZERO, T::INFINITY), points![(T::ZERO, T::ZERO)]);
                    new_fn.extend(&from_time, value);
                    let res = self.function_by_comm.insert(*i, new_fn);
                    assert!(res.is_none());
                }
            }
            Some(back) => {
                debug_assert!(back.time <= from_time + T::TOL);
                for (i, value) in values_map.iter() {
                    match self.function_by_comm.get_mut(i) {
                        None => {
                            let mut new_fn = PiecewiseConstant::new(
                                (T::ZERO, T::INFINITY),
                                points![(T::ZERO, T::ZERO)],
                            );
                            new_fn.extend(&from_time, value);
                            self.function_by_comm.insert(*i, new_fn);
                        }
                        Some(function) => {
                            function.extend(&from_time, value);
                        }
                    }
                }
            }
        }
        self.queue.push_back(FlowRatesCollectionItem {
            time: from_time,
            values: values_map,
        });
        self.accumulative.extend(&from_time, values_sum);
    }
}

/// A PreprocessedOutflowChange described the change of the outflow of an edge.
/// The time at which this change happens is at most T_e(built_until).
/// The outflow rate function of edge has already been extended by this change.
/// Hence, we only keep the time at which the change happens, in order to keep track
/// of edges that change in the future.
#[derive(Hash, PartialEq, Eq, Debug)]
struct PreprocessedOutflowChange<T: Num> {
    edge: usize,
    change_time: T,
}

#[derive(Debug)]
pub struct DynamicFlow<T: Num> {
    built_until: T,

    // inflow[e][i] is the function fᵢₑ⁺
    inflow: Vec<FlowRatesCollection<T>>,
    // outflow[e][i] is the function fᵢₑ⁻
    outflow: Vec<FlowRatesCollection<T>>,
    // queues[e] is the queue length at e
    queues: Vec<PiecewiseLinear<T>>,
    // A priority queue with times when some edge outflow changes up to (and including) the current arrivel time of the edge
    outflow_changes: PriorityQueue<PreprocessedOutflowChange<T>, Reverse<T>>,
    // A priority queue with events at which queues deplete
    depletions: DepletionQueue<T>,
}

impl<T: Num> DynamicFlow<T> {
    pub fn new(num_edges: usize) -> Self {
        DynamicFlow {
            built_until: T::ZERO,
            inflow: vec![FlowRatesCollection::new(); num_edges],
            outflow: vec![FlowRatesCollection::new(); num_edges],
            queues: vec![
                PiecewiseLinear::new(
                    (-T::INFINITY, T::INFINITY),
                    T::ZERO,
                    T::ZERO,
                    points!((T::ZERO, T::ZERO)),
                );
                num_edges
            ],
            outflow_changes: PriorityQueue::new(),
            depletions: DepletionQueue::new(),
        }
    }

    pub fn built_until(&self) -> T {
        self.built_until
    }

    pub fn inflow(&self) -> &Vec<FlowRatesCollection<T>> {
        &self.inflow
    }

    pub fn outflow(&self) -> &Vec<FlowRatesCollection<T>> {
        &self.outflow
    }

    pub fn outflow_at_built_until(&mut self, edge: usize) -> Option<&HashMap<usize, T>> {
        self.outflow[edge].get_values_at_time(self.built_until)
    }

    pub fn queues(&self) -> &Vec<PiecewiseLinear<T>> {
        &self.queues
    }

    /// Extends the flow with constant inflows new_inflow until some edge outflow changes.
    /// Edge inflows not in new_inflow are extended with their previous values.
    /// You can also specify a maximum extension length using max_extension_length.
    /// :returns set of edges where the outflow has changed at the new time `self.built_until`
    pub fn extend(
        &mut self,
        new_inflow: HashMap<usize, HashMap<usize, T>>,
        max_extension_time: Option<T>,
        capacity: &[T],
        inv_capacity: &[T],
        travel_time: &[T],
    ) -> HashSet<usize> {
        for (edge, new_inflow_e) in new_inflow.into_iter() {
            if *self.inflow[edge]
                .get_values_at_time(self.built_until)
                .unwrap_or(&HashMap::new())
                == new_inflow_e
            {
                continue;
            }
            let acc_in: T = new_inflow_e.values().sum_iter();
            let cur_queue: T = max(self.queues[edge].eval(self.built_until), T::ZERO);

            self.inflow[edge].extend(self.built_until, new_inflow_e.clone(), acc_in);

            let capacity_e = capacity[edge];
            let inv_capacity_e = inv_capacity[edge];
            let travel_time_e = travel_time[edge];
            if acc_in == T::ZERO {
                self._extend_case_i(edge, cur_queue, capacity_e, inv_capacity_e, travel_time_e);
            } else if cur_queue == T::ZERO || acc_in >= capacity_e - T::TOL {
                self._extend_case_ii(
                    edge,
                    new_inflow_e,
                    cur_queue,
                    acc_in,
                    capacity_e,
                    inv_capacity_e,
                    travel_time_e,
                );
            } else {
                self._extend_case_iii(
                    edge,
                    new_inflow_e,
                    cur_queue,
                    acc_in,
                    capacity_e,
                    inv_capacity_e,
                    travel_time_e,
                );
            }
        }

        self.built_until = {
            let mut new_built_until = T::INFINITY;
            if let Some(time) = self.depletions.min_change_time() {
                new_built_until = min(new_built_until, *time);
            }
            if let Some((_, Reverse(time))) = self.outflow_changes.peek() {
                new_built_until = min(new_built_until, *time);
            }
            if let Some(time) = max_extension_time {
                new_built_until = min(new_built_until, time);
            }
            new_built_until
        };

        self._process_depletions();

        let mut changed_edges: HashSet<usize> = HashSet::new();
        if self.built_until >= T::INFINITY {
            return changed_edges;
        }

        while self
            .outflow_changes
            .peek()
            .is_some_and(|(_, Reverse(time))| time <= &self.built_until)
        {
            changed_edges.insert(self.outflow_changes.pop().unwrap().0.edge);
        }

        changed_edges
    }

    fn _extend_case_i(
        &mut self,
        edge: usize,
        cur_queue: T,
        capacity: T,
        inv_capacity: T,
        travel_time: T,
    ) {
        let queue_fn = &mut self.queues[edge];
        let arrival = self.built_until + cur_queue * inv_capacity + travel_time;
        self.outflow[edge].extend(arrival, HashMap::new(), T::ZERO);

        self.outflow_changes.push(
            PreprocessedOutflowChange {
                edge,
                change_time: arrival,
            },
            Reverse(arrival),
        );

        if cur_queue == T::ZERO {
            let queue_slope = T::ZERO;
            queue_fn.extend(&self.built_until, queue_slope);
            self.depletions.remove(edge);
        } else {
            let queue_slope = -capacity;
            queue_fn.extend(&self.built_until, queue_slope);
            let depl_time = self.built_until + cur_queue * inv_capacity;
            let mille: T = iter::repeat(T::ONE).take(1000).sum();
            debug_assert!(
                queue_fn.eval(depl_time) <= mille * T::TOL,
                "depl_time: {}, queue_fn.eval(depl_time): {}",
                depl_time,
                queue_fn.eval(depl_time)
            );
            self.depletions.set(edge, depl_time, None)
        }
    }

    fn _extend_case_ii(
        &mut self,
        edge: usize,
        new_inflow_e: HashMap<usize, T>,
        cur_queue: T,
        acc_in: T,
        capacity: T,
        inv_capacity: T,
        travel_time: T,
    ) {
        let arrival = self.built_until + cur_queue * inv_capacity + travel_time;

        let acc_out = min(capacity, acc_in);
        let factor = acc_out / acc_in;
        let mut outflow_map: HashMap<usize, T> = new_inflow_e;
        for (_, v) in outflow_map.iter_mut() {
            *v *= factor;
        }

        self.outflow[edge].extend(arrival, outflow_map, acc_out);

        self.outflow_changes.push(
            PreprocessedOutflowChange {
                edge,
                change_time: arrival,
            },
            Reverse(arrival),
        );
        let queue_slope = max(acc_in - capacity, T::ZERO);
        self.queues[edge].extend(&self.built_until, queue_slope);
        self.depletions.remove(edge);
    }

    fn _extend_case_iii(
        &mut self,
        edge: usize,
        new_inflow_e: HashMap<usize, T>,
        cur_queue: T,
        acc_in: T,
        capacity: T,
        inv_capacity: T,
        travel_time: T,
    ) {
        let arrival = self.built_until + cur_queue * inv_capacity + travel_time;
        let factor = capacity / acc_in;

        let mut outflow_map: HashMap<usize, T> = new_inflow_e;
        for (_, v) in outflow_map.iter_mut() {
            *v *= factor;
        }

        self.outflow[edge].extend(arrival, outflow_map.clone(), capacity);

        self.outflow_changes.push(
            PreprocessedOutflowChange {
                edge,
                change_time: arrival,
            },
            Reverse(arrival),
        );

        let queue_slope = acc_in - capacity;
        self.queues[edge].extend(&self.built_until, queue_slope);

        let depl_time = self.built_until + cur_queue / queue_slope;
        let planned_change_time = depl_time + travel_time;
        let mille: T = iter::repeat(T::ONE).take(1000).sum();
        debug_assert!(self.queues[edge].eval(depl_time) < mille * T::TOL);

        self.depletions.set(
            edge,
            depl_time,
            Some(ChangeEvent {
                time: planned_change_time,
                value: ChangeEventValue {
                    new_outflow_map: outflow_map,
                    values_sum: acc_in,
                },
            }),
        );
    }

    fn _process_depletions(&mut self) {
        if self.built_until >= T::INFINITY {
            return;
        }
        while self
            .depletions
            .min_depletion_time()
            .is_some_and(|t| t <= &self.built_until)
        {
            let (edge, depl_time, change_event) = self.depletions.pop_by_depletion().unwrap();
            let queue_e = &mut self.queues[edge];
            queue_e.extend(&depl_time, T::ZERO);
            let queue_e_last = queue_e.points.last_mut().unwrap();
            let mille: T = iter::repeat(T::ONE).take(1000).sum();
            debug_assert!(abs(queue_e_last.1) < mille * T::TOL);
            queue_e_last.1 = T::ZERO;

            if let Some(change_event) = change_event {
                self.outflow[edge].extend(
                    change_event.time,
                    change_event.value.new_outflow_map,
                    change_event.value.values_sum,
                );
                self.outflow_changes.push(
                    PreprocessedOutflowChange {
                        edge,
                        change_time: change_event.time,
                    },
                    Reverse(change_event.time),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{float::F64, num::Num};

    use super::DynamicFlow;

    #[test]
    fn test_dynamic_flow() {
        let mut dynamic_flow: DynamicFlow<F64> = DynamicFlow::new(1);
        dynamic_flow.extend(
            HashMap::from([(0usize, HashMap::from([(0usize, 1.0.into())]))]),
            None,
            &[1.0.into()],
            &[1.0.into()],
            &[1.0.into()],
        );
        assert_eq!(dynamic_flow.built_until, 1.0);
        dynamic_flow.extend(
            HashMap::from([(0usize, HashMap::from([(0usize, 1.0.into())]))]),
            None,
            &[1.0.into()],
            &[1.0.into()],
            &[1.0.into()],
        );
        assert_eq!(dynamic_flow.built_until, F64::INFINITY);
    }
}
