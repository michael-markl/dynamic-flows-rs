use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::{
    dynamic_flow::DynamicFlow, num::Num, piecewise_constant::PiecewiseConstant, point::Point,
};

#[derive(Debug)]
pub struct NetworkLoader<T: Num> {
    // Describes the path by mapping (Commodity, Edge?) -> Edge?
    next_edge: HashMap<(usize, Option<usize>), usize>,

    // The changes of the inflow rate of all paths:
    // The key is the time of the change, the value is the path that changes and the new inflow rate
    path_inflow_rate_changes: PriorityQueue<(usize, T), Reverse<T>>,
}

pub struct PathInflow<'a, T: Num> {
    pub path: &'a [usize],
    pub inflow: &'a PiecewiseConstant<T>,
}

impl<T: Num> NetworkLoader<T> {
    pub fn new<'a>(path_inflows: &'a [PathInflow<'a, T>]) -> Self {
        let mut next_edge_map: HashMap<(usize, Option<usize>), usize> =
            HashMap::with_capacity(path_inflows.iter().map(|p| p.path.len() + 1).sum());
        let mut path_inflow_rate_changes = PriorityQueue::with_capacity(
            path_inflows.iter().map(|p| p.inflow.points().len()).sum(),
        );
        for (i, path_inflow) in path_inflows.iter().enumerate() {
            if let Some(&edge) = path_inflow.path.first() {
                next_edge_map.insert((i, None), edge);
            }
            for (&edge, &next_edge) in path_inflow.path.iter().tuple_windows() {
                next_edge_map.insert((i, Some(edge)), next_edge);
            }

            for &Point(time, value) in path_inflow.inflow.points().iter() {
                path_inflow_rate_changes.push((i, value), Reverse(time));
            }
        }

        Self {
            next_edge: next_edge_map,
            path_inflow_rate_changes,
        }
    }

    pub fn build_flow(
        mut self,
        num_edges: usize,
        capacity: &[T],
        inv_capacity: &[T],
        travel_time: &[T],
    ) -> DynamicFlow<T> {
        let mut flow: DynamicFlow<T> = DynamicFlow::new(num_edges);

        // By edge, by path
        let mut new_inflow: HashMap<usize, HashMap<usize, T>> = HashMap::new();
        while flow.built_until() < T::INFINITY {
            while self
                .path_inflow_rate_changes
                .peek()
                .is_some_and(|(_, Reverse(time))| *time <= flow.built_until())
            {
                let ((path, new_value), _) = self.path_inflow_rate_changes.pop().unwrap();
                new_inflow
                    .entry(self.next_edge[&(path, None)])
                    .or_insert(HashMap::new())
                    .entry(path)
                    .and_modify(|v| {
                        *v += new_value;
                    })
                    .or_insert(new_value);
            }

            let max_extension_time = self
                .path_inflow_rate_changes
                .peek()
                .map(|(_, Reverse(change_time))| *change_time);

            let changed_edges = flow.extend(
                new_inflow,
                max_extension_time,
                capacity,
                inv_capacity,
                travel_time,
            );
            new_inflow = HashMap::new();
            for edge in changed_edges {
                let values = flow.outflow_at_built_until(edge);
                match values {
                    None => {}
                    Some(outflow_map) => {
                        for (&path, &outflow) in outflow_map.iter() {
                            let next_edge = self.next_edge.get(&(path, Some(edge)));
                            if let Some(&next_edge) = next_edge {
                                new_inflow
                                    .entry(next_edge)
                                    .or_insert(HashMap::new())
                                    .entry(path)
                                    .and_modify(|v| {
                                        *v += outflow;
                                    })
                                    .or_insert(outflow);
                            }
                        }
                    }
                }
            }
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use crate::{float::F64, num::Num, piecewise_constant::PiecewiseConstant, points};

    use super::{NetworkLoader, PathInflow};

    #[test]
    fn it_should_do_a_correct_network_loading() {
        let network_loader: NetworkLoader<F64> = NetworkLoader::new(&[
            PathInflow {
                path: &[0, 1, 2],
                inflow: &PiecewiseConstant::new(
                    [-F64::INFINITY, F64::INFINITY],
                    points![(0.0, 1.0), (3.0, 0.0)],
                ),
            },
            PathInflow {
                path: &[2, 0, 1],
                inflow: &PiecewiseConstant::new(
                    [-F64::INFINITY, F64::INFINITY],
                    points![(0.0, 2.0), (3.0, 0.0)],
                ),
            },
        ]);
        let flow = network_loader.build_flow(
            3,
            &[1.0.into(), 2.0.into(), 3.0.into()],
            &[(1.0 / 1.0).into(), (1.0 / 2.0).into(), (1.0 / 3.0).into()],
            &[1.0.into(), 2.0.into(), 3.0.into()],
        );
        assert_eq!(flow.built_until(), F64::INFINITY);
    }
}
