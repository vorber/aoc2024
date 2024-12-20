use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, hash::Hash, ops::Add};

use itertools::Itertools;

#[derive(Debug)]
pub enum GraphError {
    NoPath
} 

pub struct Graph<V,C> {
    pub vertices: Vec<V>,
    pub edges: HashMap<V, Vec<Edge<V>>>,
    pub costs: HashMap<Edge<V>, C> 
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Edge<V> {
    pub from: V,
    pub to: V,
}

impl<V> Edge<V> {
    pub fn new(from: V, to: V) ->  Self { Edge { from, to} }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState<V,C> {
    position:V,
    cost:C
}

impl<V:Eq+Ord, C:Eq+Ord> Ord for DijkstraState<V,C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl<V:PartialEq+Ord, C:PartialEq+Ord> PartialOrd for DijkstraState<V,C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<V:Eq+Ord+Copy+Hash, C:Default+Copy+Ord+Add<Output = C>> Graph<V,C> {
    pub fn dijkstra(&self, start: V) -> HashMap<&V,Option<(C,Vec<V>)>> {
        let mut distances = self.vertices.iter().map(|v| (v, None)).collect::<HashMap<&V,Option<(C,Vec<V>)>>>();
        let mut q = BinaryHeap::new();
        q.push(DijkstraState { position: start, cost: C::default()});
        while let Some(DijkstraState {position,cost}) = q.pop() {
            if distances[&position].as_ref().is_some_and(|d| cost > d.0) { continue; }
            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let c = cost + self.costs[&edge];
                    let entry = distances.entry(&edge.to).or_default();
                    if entry.as_ref().is_none_or(|d| d.0 > c) {
                        *entry = Some((c, Vec::from([position])));
                        q.push(DijkstraState { position: edge.to, cost: c });
                    } else if entry.as_ref().is_some_and(|d| d.0 == c) {
                        entry.as_mut().unwrap().1.push(position);
                    }
                }
            }
        }
        distances
    } 

    pub fn find_path(&self, start:V, end:V) -> Result<Vec<V>, GraphError> {
        let visits = self.dijkstra(start);
        let mut path = Vec::new();
        let mut v = &end;
        while v != &start {
            let visit = visits.get(v).ok_or(GraphError::NoPath)?;
            path.push(v.clone());
            v = visit.as_ref().and_then(|x| x.1.first()).ok_or(GraphError::NoPath)?;
        }
        path.push(start.clone());
        Ok(path.into_iter().rev().collect_vec())
    }
}

