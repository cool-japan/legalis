//! Deterministic 3-D layout algorithms that position the nodes of a
//! [`Scene3d`].
//!
//! All algorithms are seeded deterministically (via [`super::seed_position`])
//! so a given scene always lays out identically — no `rand` dependency, in line
//! with the workspace SciRS2 policy.

use super::{Scene3d, Vec3, seed_position};
use crate::DiffResult;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};

/// Which layout algorithm to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    /// Fruchterman–Reingold force-directed layout in 3-D.
    #[default]
    ForceDirected,
    /// Even distribution over a sphere via the Fibonacci-lattice / golden-angle
    /// method.
    FibonacciSphere,
    /// BFS layers from the highest-degree root, stacked along `+Y`, each layer
    /// fanned out in a circle.
    Layered,
    /// A space-filling cubic lattice.
    Grid,
}

/// Tunable parameters shared by the layout algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayoutParams {
    /// Number of relaxation iterations (force-directed only).
    pub iterations: u32,
    /// Overall spread / characteristic edge length.
    pub spread: f64,
    /// Strength of the centring gravity that keeps components together.
    pub gravity: f64,
    /// Initial temperature (max per-step displacement); cools linearly to 0.
    pub initial_temperature: f64,
}

impl Default for LayoutParams {
    fn default() -> Self {
        Self {
            iterations: 120,
            spread: 3.0,
            gravity: 0.05,
            initial_temperature: 4.0,
        }
    }
}

impl LayoutParams {
    /// Builder: sets the iteration count.
    #[must_use]
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    /// Builder: sets the spread.
    #[must_use]
    pub fn with_spread(mut self, spread: f64) -> Self {
        self.spread = spread.max(0.001);
        self
    }

    /// Builder: sets the gravity strength.
    #[must_use]
    pub fn with_gravity(mut self, gravity: f64) -> Self {
        self.gravity = gravity;
        self
    }
}

/// Applies `algorithm` to `scene`, mutating node positions in place.
///
/// An empty scene is a successful no-op.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::diff;
/// use legalis_diff::immersive::{scene_from_diff, apply_layout, LayoutAlgorithm, LayoutParams};
///
/// let old = Statute::new("law", "T", Effect::new(EffectType::Grant, "B"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "R");
/// let d = diff(&old, &new).unwrap();
///
/// let mut scene = scene_from_diff(&d);
/// apply_layout(&mut scene, LayoutAlgorithm::FibonacciSphere, &LayoutParams::default()).unwrap();
/// // Every position is finite after layout.
/// assert!(scene.nodes().iter().all(|n| n.position.is_finite()));
/// ```
///
/// # Errors
///
/// Currently infallible for the built-in algorithms, but returns a
/// [`crate::DiffResult`] so future layouts may surface failures.
pub fn apply_layout(
    scene: &mut Scene3d,
    algorithm: LayoutAlgorithm,
    params: &LayoutParams,
) -> DiffResult<()> {
    if scene.is_empty() {
        return Ok(());
    }
    match algorithm {
        LayoutAlgorithm::ForceDirected => force_directed(scene, params),
        LayoutAlgorithm::FibonacciSphere => fibonacci_sphere(scene, params),
        LayoutAlgorithm::Layered => layered(scene, params),
        LayoutAlgorithm::Grid => grid(scene, params),
    }
    Ok(())
}

/// Fruchterman–Reingold in 3-D with a cooling schedule and centring gravity.
fn force_directed(scene: &mut Scene3d, params: &LayoutParams) {
    let n = scene.node_count();
    let ids: Vec<String> = scene.nodes().iter().map(|node| node.id.clone()).collect();

    // Seed positions deterministically inside a cube sized to the node count.
    let half = params.spread * (n as f64).cbrt().max(1.0);
    let mut pos: Vec<Vec3> = ids.iter().map(|id| seed_position(id, half)).collect();

    // Optimal distance k = spread * (volume / n)^(1/3).
    let volume = (2.0 * half).powi(3).max(1.0);
    let k = params.spread * (volume / n as f64).cbrt();
    let k = k.max(0.25);

    // Adjacency as index pairs with weights.
    let idx: HashMap<&str, usize> = ids
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();
    let mut springs: Vec<(usize, usize, f64)> = Vec::new();
    for edge in scene.edges() {
        if let (Some(&a), Some(&b)) = (idx.get(edge.source.as_str()), idx.get(edge.target.as_str()))
        {
            springs.push((a, b, edge.weight.max(0.1)));
        }
    }

    let iterations = params.iterations.max(1);
    for step in 0..iterations {
        let cooling = 1.0 - (step as f64) / (iterations as f64);
        let temperature = (params.initial_temperature * cooling).max(0.0);
        let mut disp = vec![Vec3::zero(); n];

        // Repulsion between every pair (O(n^2); scenes here are modest).
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = pos[i] - pos[j];
                let dist = delta.length().max(1e-4);
                let force = (k * k) / dist;
                let dir = delta.scale(1.0 / dist);
                disp[i] += dir.scale(force);
                disp[j] -= dir.scale(force);
            }
        }

        // Attraction along edges.
        for &(a, b, weight) in &springs {
            let delta = pos[a] - pos[b];
            let dist = delta.length().max(1e-4);
            let force = (dist * dist) / k * weight;
            let dir = delta.scale(1.0 / dist);
            disp[a] -= dir.scale(force);
            disp[b] += dir.scale(force);
        }

        // Centring gravity (pulls everything toward the origin).
        for i in 0..n {
            disp[i] -= pos[i].scale(params.gravity * k);
        }

        // Integrate, capped by the current temperature.
        for i in 0..n {
            let mag = disp[i].length();
            if mag > 1e-9 {
                let capped = disp[i].scale(mag.min(temperature) / mag);
                pos[i] += capped;
            }
        }
    }

    write_back(scene, &ids, &pos);
}

/// Distributes nodes evenly over a sphere using the golden-angle spiral.
fn fibonacci_sphere(scene: &mut Scene3d, params: &LayoutParams) {
    let ids: Vec<String> = scene.nodes().iter().map(|node| node.id.clone()).collect();
    let n = ids.len();
    let radius = params.spread * (n as f64).cbrt().max(1.0);
    // Golden angle in radians.
    let golden = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());
    let mut pos = Vec::with_capacity(n);
    for i in 0..n {
        let denom = if n > 1 { (n - 1) as f64 } else { 1.0 };
        // y from +1 down to -1.
        let y = 1.0 - (i as f64 / denom) * 2.0;
        let ring = (1.0 - y * y).max(0.0).sqrt();
        let theta = golden * i as f64;
        pos.push(Vec3::new(
            theta.cos() * ring * radius,
            y * radius,
            theta.sin() * ring * radius,
        ));
    }
    write_back(scene, &ids, &pos);
}

/// Stacks BFS layers from the highest-degree root along `+Y`, fanning each layer
/// out in a circle on its own XZ plane.
fn layered(scene: &mut Scene3d, params: &LayoutParams) {
    let ids: Vec<String> = scene.nodes().iter().map(|node| node.id.clone()).collect();
    if ids.is_empty() {
        return;
    }

    // Choose the root as the highest-degree node (ties broken by id order).
    let root = ids
        .iter()
        .max_by(|a, b| scene.degree(a).cmp(&scene.degree(b)).then_with(|| b.cmp(a)))
        .cloned()
        .unwrap_or_else(|| ids[0].clone());

    // BFS depth from the root.
    let mut depth: HashMap<String, u32> = HashMap::new();
    depth.insert(root.clone(), 0);
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(root.clone());
    while let Some(cur) = queue.pop_front() {
        let d = *depth.get(&cur).unwrap_or(&0);
        for nb in scene.neighbors(&cur) {
            if !depth.contains_key(&nb) {
                depth.insert(nb.clone(), d + 1);
                queue.push_back(nb);
            }
        }
    }

    // Disconnected nodes land on the deepest+1 layer.
    let max_depth = depth.values().copied().max().unwrap_or(0);
    for id in &ids {
        depth.entry(id.clone()).or_insert(max_depth + 1);
    }

    // Group ids per layer (BTreeMap → deterministic order).
    let mut layers: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for id in &ids {
        let d = *depth.get(id).unwrap_or(&0);
        layers.entry(d).or_default().push(id.clone());
    }
    for v in layers.values_mut() {
        v.sort();
    }

    let layer_gap = params.spread * 2.0;
    let mut pos_by_id: HashMap<String, Vec3> = HashMap::new();
    for (&d, members) in &layers {
        let count = members.len().max(1);
        let ring_radius = params.spread * (count as f64).sqrt().max(0.5);
        let y = d as f64 * layer_gap;
        for (i, id) in members.iter().enumerate() {
            if count == 1 {
                pos_by_id.insert(id.clone(), Vec3::new(0.0, y, 0.0));
            } else {
                let angle = (i as f64 / count as f64) * std::f64::consts::TAU;
                pos_by_id.insert(
                    id.clone(),
                    Vec3::new(angle.cos() * ring_radius, y, angle.sin() * ring_radius),
                );
            }
        }
    }

    let pos: Vec<Vec3> = ids
        .iter()
        .map(|id| pos_by_id.get(id).copied().unwrap_or_else(Vec3::zero))
        .collect();
    write_back(scene, &ids, &pos);
}

/// Fills a cubic lattice in id order.
fn grid(scene: &mut Scene3d, params: &LayoutParams) {
    let ids: Vec<String> = scene.nodes().iter().map(|node| node.id.clone()).collect();
    let n = ids.len();
    let side = (n as f64).cbrt().ceil().max(1.0) as usize;
    let gap = params.spread;
    let offset = (side.saturating_sub(1)) as f64 * gap * 0.5;
    let mut pos = Vec::with_capacity(n);
    for i in 0..n {
        let x = i % side;
        let y = (i / side) % side;
        let z = i / (side * side);
        pos.push(Vec3::new(
            x as f64 * gap - offset,
            y as f64 * gap - offset,
            z as f64 * gap - offset,
        ));
    }
    write_back(scene, &ids, &pos);
}

/// Writes computed positions back into the scene by id.
fn write_back(scene: &mut Scene3d, ids: &[String], pos: &[Vec3]) {
    for (id, p) in ids.iter().zip(pos.iter()) {
        let safe = if p.is_finite() { *p } else { Vec3::zero() };
        scene.set_position(id, safe);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::immersive::{scene_from_diff, scene_from_diffs};
    use legalis_core::{Effect, EffectType, Statute};

    fn many_diffs(count: usize) -> Vec<crate::StatuteDiff> {
        (0..count)
            .map(|i| {
                let old = Statute::new(
                    format!("law-{i}"),
                    "Old",
                    Effect::new(EffectType::Grant, "Benefit"),
                );
                let mut new = old.clone();
                new.title = format!("New {i}");
                new.effect = Effect::new(EffectType::Revoke, "Revoked");
                crate::diff(&old, &new).expect("diff")
            })
            .collect()
    }

    fn sample_scene() -> Scene3d {
        scene_from_diffs(&many_diffs(4))
    }

    #[test]
    fn test_force_directed_is_deterministic() {
        let mut a = sample_scene();
        let mut b = sample_scene();
        let params = LayoutParams::default().with_iterations(50);
        apply_layout(&mut a, LayoutAlgorithm::ForceDirected, &params).unwrap();
        apply_layout(&mut b, LayoutAlgorithm::ForceDirected, &params).unwrap();
        for (na, nb) in a.nodes().iter().zip(b.nodes().iter()) {
            assert_eq!(na.position, nb.position);
        }
    }

    #[test]
    fn test_force_directed_positions_finite() {
        let mut scene = sample_scene();
        apply_layout(
            &mut scene,
            LayoutAlgorithm::ForceDirected,
            &LayoutParams::default(),
        )
        .unwrap();
        assert!(scene.nodes().iter().all(|n| n.position.is_finite()));
    }

    #[test]
    fn test_force_directed_separates_nodes() {
        let mut scene = sample_scene();
        apply_layout(
            &mut scene,
            LayoutAlgorithm::ForceDirected,
            &LayoutParams::default(),
        )
        .unwrap();
        // No two distinct nodes occupy the exact same point.
        let nodes = scene.nodes();
        let mut collisions = 0;
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if nodes[i].position.distance(nodes[j].position) < 1e-6 {
                    collisions += 1;
                }
            }
        }
        assert_eq!(collisions, 0);
    }

    #[test]
    fn test_fibonacci_sphere_on_sphere() {
        let mut scene = sample_scene();
        let params = LayoutParams::default().with_spread(2.0);
        apply_layout(&mut scene, LayoutAlgorithm::FibonacciSphere, &params).unwrap();
        let radius = params.spread * (scene.node_count() as f64).cbrt().max(1.0);
        // Every node lies (approximately) on the sphere of that radius.
        assert!(
            scene
                .nodes()
                .iter()
                .all(|n| (n.position.length() - radius).abs() < 1e-6)
        );
    }

    #[test]
    fn test_layered_root_at_origin_plane() {
        let scene_src = scene_from_diff(&many_diffs(1)[0]);
        let mut scene = scene_src;
        apply_layout(
            &mut scene,
            LayoutAlgorithm::Layered,
            &LayoutParams::default(),
        )
        .unwrap();
        // Highest-degree node is the statute root; its layer (y) should be the
        // minimum among all nodes.
        let min_y = scene
            .nodes()
            .iter()
            .map(|n| n.position.y)
            .fold(f64::INFINITY, f64::min);
        let root = scene.node("law-0").expect("root present");
        assert!((root.position.y - min_y).abs() < 1e-9);
    }

    #[test]
    fn test_grid_layout_finite_and_unique() {
        let mut scene = sample_scene();
        apply_layout(&mut scene, LayoutAlgorithm::Grid, &LayoutParams::default()).unwrap();
        let nodes = scene.nodes();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                assert!(nodes[i].position.distance(nodes[j].position) > 1e-9);
            }
        }
    }

    #[test]
    fn test_empty_scene_layout_is_noop() {
        let mut scene = Scene3d::new();
        assert!(
            apply_layout(
                &mut scene,
                LayoutAlgorithm::ForceDirected,
                &LayoutParams::default()
            )
            .is_ok()
        );
        assert!(scene.is_empty());
    }

    #[test]
    fn test_layout_params_builders() {
        let p = LayoutParams::default()
            .with_iterations(10)
            .with_spread(5.0)
            .with_gravity(0.1);
        assert_eq!(p.iterations, 10);
        assert!((p.spread - 5.0).abs() < 1e-9);
        assert!((p.gravity - 0.1).abs() < 1e-9);
    }
}
