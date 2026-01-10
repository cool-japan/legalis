//! Economic Simulation Extensions Module
//!
//! This module provides advanced economic modeling capabilities:
//! - DSGE (Dynamic Stochastic General Equilibrium) models
//! - Input-output economic modeling
//! - Financial contagion simulation
//! - Market microstructure modeling
//! - Behavioral economics integration

use crate::error::{SimResult, SimulationError};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// DSGE (Dynamic Stochastic General Equilibrium) Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSGEModel {
    /// Discount factor (beta)
    discount_factor: f64,
    /// Risk aversion coefficient
    risk_aversion: f64,
    /// Technology shock persistence
    shock_persistence: f64,
    /// Technology shock standard deviation
    shock_std_dev: f64,
    /// Current technology level
    technology: f64,
    /// Capital stock
    capital: f64,
    /// Labor supply
    labor: f64,
    /// Total factor productivity
    tfp: f64,
}

impl DSGEModel {
    /// Create a new DSGE model
    pub fn new(discount_factor: f64, risk_aversion: f64) -> SimResult<Self> {
        if discount_factor <= 0.0 || discount_factor >= 1.0 {
            return Err(SimulationError::InvalidParameter(
                "Discount factor must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            discount_factor,
            risk_aversion,
            shock_persistence: 0.95,
            shock_std_dev: 0.007,
            technology: 1.0,
            capital: 10.0,
            labor: 1.0,
            tfp: 1.0,
        })
    }

    /// Set shock parameters
    pub fn with_shock_params(mut self, persistence: f64, std_dev: f64) -> Self {
        self.shock_persistence = persistence;
        self.shock_std_dev = std_dev;
        self
    }

    /// Simulate one period
    pub fn step(&mut self) -> DSGEState {
        let mut rng = rand::rng();

        // Technology shock (AR(1) process)
        let epsilon: f64 = rng.random_range(-1.0..1.0) * self.shock_std_dev;
        self.technology = self.shock_persistence * self.technology.ln() + epsilon;
        self.technology = self.technology.exp();

        // Production function: Y = A * K^alpha * L^(1-alpha)
        let alpha = 0.36;
        let output =
            self.tfp * self.technology * self.capital.powf(alpha) * self.labor.powf(1.0 - alpha);

        // Consumption and investment
        let depreciation = 0.025;
        let investment_rate = 0.2;
        let investment = investment_rate * output;
        let consumption = output - investment;

        // Update capital stock
        self.capital = (1.0 - depreciation) * self.capital + investment;

        DSGEState {
            output,
            consumption,
            investment,
            capital: self.capital,
            labor: self.labor,
            technology: self.technology,
            interest_rate: self.discount_factor.recip() - 1.0,
        }
    }

    /// Simulate multiple periods
    pub fn simulate(&mut self, periods: usize) -> Vec<DSGEState> {
        (0..periods).map(|_| self.step()).collect()
    }

    /// Get steady state values
    pub fn steady_state(&self) -> DSGEState {
        let alpha = 0.36;
        let depreciation = 0.025;
        let steady_k =
            (alpha / (self.discount_factor.recip() - 1.0 + depreciation)).powf(1.0 / (1.0 - alpha));
        let steady_y = steady_k.powf(alpha);
        let steady_c = steady_y - depreciation * steady_k;
        let steady_i = depreciation * steady_k;

        DSGEState {
            output: steady_y,
            consumption: steady_c,
            investment: steady_i,
            capital: steady_k,
            labor: 1.0,
            technology: 1.0,
            interest_rate: self.discount_factor.recip() - 1.0,
        }
    }
}

/// DSGE model state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSGEState {
    pub output: f64,
    pub consumption: f64,
    pub investment: f64,
    pub capital: f64,
    pub labor: f64,
    pub technology: f64,
    pub interest_rate: f64,
}

/// Input-Output economic model (Leontief)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOutputModel {
    /// Number of sectors
    sectors: usize,
    /// Technical coefficient matrix (A)
    tech_coefficients: Vec<Vec<f64>>,
    /// Sector names
    sector_names: Vec<String>,
}

impl InputOutputModel {
    /// Create a new input-output model
    pub fn new(sectors: usize, sector_names: Vec<String>) -> SimResult<Self> {
        if sectors == 0 {
            return Err(SimulationError::InvalidParameter(
                "Number of sectors must be positive".to_string(),
            ));
        }

        if sector_names.len() != sectors {
            return Err(SimulationError::InvalidParameter(
                "Sector names length must match number of sectors".to_string(),
            ));
        }

        let tech_coefficients = vec![vec![0.0; sectors]; sectors];

        Ok(Self {
            sectors,
            tech_coefficients,
            sector_names,
        })
    }

    /// Set technical coefficient (how much sector j needs from sector i per unit output)
    pub fn set_coefficient(
        &mut self,
        from_sector: usize,
        to_sector: usize,
        value: f64,
    ) -> SimResult<()> {
        if from_sector >= self.sectors || to_sector >= self.sectors {
            return Err(SimulationError::InvalidParameter(
                "Sector index out of bounds".to_string(),
            ));
        }

        self.tech_coefficients[from_sector][to_sector] = value;
        Ok(())
    }

    /// Calculate total output required for given final demand
    /// Using Leontief inverse: X = (I - A)^(-1) * D
    pub fn calculate_total_output(&self, final_demand: &[f64]) -> SimResult<Vec<f64>> {
        if final_demand.len() != self.sectors {
            return Err(SimulationError::InvalidParameter(
                "Final demand vector length must match number of sectors".to_string(),
            ));
        }

        // Calculate (I - A)
        let mut i_minus_a = vec![vec![0.0; self.sectors]; self.sectors];
        #[allow(clippy::needless_range_loop)]
        for i in 0..self.sectors {
            for j in 0..self.sectors {
                i_minus_a[i][j] = if i == j { 1.0 } else { 0.0 } - self.tech_coefficients[i][j];
            }
        }

        // Solve (I - A) * X = D using Gaussian elimination
        let total_output = self.solve_linear_system(&i_minus_a, final_demand)?;
        Ok(total_output)
    }

    /// Simple Gaussian elimination for solving linear systems
    #[allow(clippy::needless_range_loop)]
    fn solve_linear_system(&self, matrix: &[Vec<f64>], rhs: &[f64]) -> SimResult<Vec<f64>> {
        let n = matrix.len();
        let mut aug = vec![vec![0.0; n + 1]; n];

        // Create augmented matrix
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = matrix[i][j];
            }
            aug[i][n] = rhs[i];
        }

        // Forward elimination
        for k in 0..n {
            // Find pivot
            let mut max_row = k;
            for i in (k + 1)..n {
                if aug[i][k].abs() > aug[max_row][k].abs() {
                    max_row = i;
                }
            }

            // Swap rows
            aug.swap(k, max_row);

            // Check for singular matrix
            if aug[k][k].abs() < 1e-10 {
                return Err(SimulationError::InvalidParameter(
                    "Matrix is singular or nearly singular".to_string(),
                ));
            }

            // Eliminate column
            for i in (k + 1)..n {
                let factor = aug[i][k] / aug[k][k];
                for j in k..=n {
                    aug[i][j] -= factor * aug[k][j];
                }
            }
        }

        // Back substitution
        let mut solution = vec![0.0; n];
        for i in (0..n).rev() {
            solution[i] = aug[i][n];
            for j in (i + 1)..n {
                solution[i] -= aug[i][j] * solution[j];
            }
            solution[i] /= aug[i][i];
        }

        Ok(solution)
    }

    /// Calculate multipliers (how much total output changes for 1 unit change in final demand)
    pub fn multipliers(&self) -> SimResult<Vec<f64>> {
        let unit_demand = vec![1.0; self.sectors];
        let total_output = self.calculate_total_output(&unit_demand)?;
        Ok(total_output)
    }

    /// Get sector names
    pub fn sector_names(&self) -> &[String] {
        &self.sector_names
    }
}

/// Financial institution node for contagion modeling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInstitution {
    pub id: usize,
    pub name: String,
    pub capital: f64,
    pub assets: f64,
    pub liabilities: f64,
    pub failed: bool,
}

impl FinancialInstitution {
    /// Create a new financial institution
    pub fn new(id: usize, name: String, capital: f64, assets: f64) -> Self {
        Self {
            id,
            name,
            capital,
            assets,
            liabilities: assets - capital,
            failed: false,
        }
    }

    /// Check if institution is solvent
    pub fn is_solvent(&self) -> bool {
        self.capital > 0.0 && !self.failed
    }

    /// Apply shock to assets
    pub fn apply_shock(&mut self, shock: f64) {
        self.assets *= 1.0 - shock;
        self.capital = self.assets - self.liabilities;
        if self.capital <= 0.0 {
            self.failed = true;
        }
    }
}

/// Financial contagion simulator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialContagionModel {
    /// Financial institutions
    institutions: Vec<FinancialInstitution>,
    /// Exposure matrix (how much i owes to j)
    exposures: Vec<Vec<f64>>,
    /// Contagion threshold
    threshold: f64,
}

impl FinancialContagionModel {
    /// Create a new contagion model
    pub fn new(institutions: Vec<FinancialInstitution>, threshold: f64) -> Self {
        let n = institutions.len();
        let exposures = vec![vec![0.0; n]; n];

        Self {
            institutions,
            exposures,
            threshold,
        }
    }

    /// Set exposure between institutions
    pub fn set_exposure(&mut self, from_id: usize, to_id: usize, amount: f64) -> SimResult<()> {
        if from_id >= self.institutions.len() || to_id >= self.institutions.len() {
            return Err(SimulationError::InvalidParameter(
                "Institution ID out of bounds".to_string(),
            ));
        }

        self.exposures[from_id][to_id] = amount;
        Ok(())
    }

    /// Simulate contagion from initial shock
    pub fn simulate_contagion(&mut self, initial_shock: &[(usize, f64)]) -> ContagionResult {
        let mut rounds = 0;
        let mut newly_failed;

        // Apply initial shocks
        for &(id, shock) in initial_shock {
            if id < self.institutions.len() {
                self.institutions[id].apply_shock(shock);
            }
        }

        // Propagate contagion
        loop {
            newly_failed = false;
            rounds += 1;

            for i in 0..self.institutions.len() {
                if !self.institutions[i].is_solvent() {
                    // This institution failed, spread losses to creditors
                    for j in 0..self.institutions.len() {
                        if i != j && self.institutions[j].is_solvent() {
                            let exposure = self.exposures[i][j];
                            if exposure > 0.0 {
                                // Calculate loss for institution j
                                let loss = exposure
                                    * (1.0
                                        - self.institutions[i].capital.max(0.0)
                                            / self.institutions[i].assets);
                                let loss_rate = loss / self.institutions[j].assets;

                                if loss_rate > self.threshold {
                                    self.institutions[j].apply_shock(loss_rate);
                                    if !self.institutions[j].is_solvent() {
                                        newly_failed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !newly_failed || rounds > 100 {
                break;
            }
        }

        let failed_count = self.institutions.iter().filter(|i| !i.is_solvent()).count();
        let total_capital_loss: f64 = self
            .institutions
            .iter()
            .map(|i| if i.failed { i.capital.abs() } else { 0.0 })
            .sum();

        ContagionResult {
            rounds,
            failed_institutions: failed_count,
            total_institutions: self.institutions.len(),
            total_capital_loss,
            systemic_crisis: failed_count as f64 / self.institutions.len() as f64 > 0.3,
        }
    }

    /// Get institutions
    pub fn institutions(&self) -> &[FinancialInstitution] {
        &self.institutions
    }
}

/// Result of contagion simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContagionResult {
    pub rounds: usize,
    pub failed_institutions: usize,
    pub total_institutions: usize,
    pub total_capital_loss: f64,
    pub systemic_crisis: bool,
}

/// Order type in market
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Bid,
    Ask,
}

/// Limit order in order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    pub id: usize,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: f64,
    pub trader_id: usize,
}

/// Market microstructure model (order book)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Bids (buy orders), sorted by price descending
    bids: Vec<LimitOrder>,
    /// Asks (sell orders), sorted by price ascending
    asks: Vec<LimitOrder>,
    /// Next order ID
    next_id: usize,
    /// Transaction history
    transactions: Vec<Transaction>,
}

impl OrderBook {
    /// Create a new order book
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            next_id: 0,
            transactions: Vec::new(),
        }
    }

    /// Submit a limit order
    pub fn submit_order(
        &mut self,
        order_type: OrderType,
        price: f64,
        quantity: f64,
        trader_id: usize,
    ) -> usize {
        let order_id = self.next_id;
        self.next_id += 1;

        let order = LimitOrder {
            id: order_id,
            order_type,
            price,
            quantity,
            trader_id,
        };

        match order_type {
            OrderType::Bid => {
                self.bids.push(order);
                self.bids
                    .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
            OrderType::Ask => {
                self.asks.push(order);
                self.asks
                    .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }

        // Try to match orders
        self.match_orders();

        order_id
    }

    /// Match crossing orders
    fn match_orders(&mut self) {
        while !self.bids.is_empty() && !self.asks.is_empty() {
            let best_bid = &self.bids[0];
            let best_ask = &self.asks[0];

            if best_bid.price >= best_ask.price {
                // Match found
                let match_price = (best_bid.price + best_ask.price) / 2.0;
                let match_quantity = best_bid.quantity.min(best_ask.quantity);

                let transaction = Transaction {
                    price: match_price,
                    quantity: match_quantity,
                    buyer_id: best_bid.trader_id,
                    seller_id: best_ask.trader_id,
                };
                self.transactions.push(transaction);

                // Update or remove orders
                let bid_remaining = self.bids[0].quantity - match_quantity;
                let ask_remaining = self.asks[0].quantity - match_quantity;

                if bid_remaining <= 0.0 {
                    self.bids.remove(0);
                } else {
                    self.bids[0].quantity = bid_remaining;
                }

                if ask_remaining <= 0.0 {
                    self.asks.remove(0);
                } else {
                    self.asks[0].quantity = ask_remaining;
                }
            } else {
                break;
            }
        }
    }

    /// Get best bid price
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|o| o.price)
    }

    /// Get best ask price
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|o| o.price)
    }

    /// Get bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Get recent transactions
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    /// Get market depth at level
    pub fn market_depth(&self, levels: usize) -> MarketDepth {
        let bid_depth: f64 = self.bids.iter().take(levels).map(|o| o.quantity).sum();
        let ask_depth: f64 = self.asks.iter().take(levels).map(|o| o.quantity).sum();

        MarketDepth {
            bid_depth,
            ask_depth,
            levels,
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub price: f64,
    pub quantity: f64,
    pub buyer_id: usize,
    pub seller_id: usize,
}

/// Market depth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepth {
    pub bid_depth: f64,
    pub ask_depth: f64,
    pub levels: usize,
}

/// Behavioral economics prospect theory value function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProspectTheory {
    /// Loss aversion coefficient (lambda)
    loss_aversion: f64,
    /// Risk aversion for gains (alpha)
    risk_aversion_gains: f64,
    /// Risk aversion for losses (beta)
    risk_aversion_losses: f64,
    /// Reference point
    reference_point: f64,
}

impl ProspectTheory {
    /// Create a new prospect theory model
    pub fn new(loss_aversion: f64) -> Self {
        Self {
            loss_aversion,
            risk_aversion_gains: 0.88,
            risk_aversion_losses: 0.88,
            reference_point: 0.0,
        }
    }

    /// Set reference point
    pub fn with_reference_point(mut self, reference_point: f64) -> Self {
        self.reference_point = reference_point;
        self
    }

    /// Calculate value function: v(x)
    pub fn value(&self, outcome: f64) -> f64 {
        let x = outcome - self.reference_point;

        if x >= 0.0 {
            // Gains
            x.powf(self.risk_aversion_gains)
        } else {
            // Losses
            -self.loss_aversion * (-x).powf(self.risk_aversion_losses)
        }
    }

    /// Evaluate a prospect (lottery)
    pub fn evaluate_prospect(&self, outcomes: &[(f64, f64)]) -> f64 {
        outcomes
            .iter()
            .map(|(outcome, prob)| prob * self.value(*outcome))
            .sum()
    }

    /// Compare two prospects
    pub fn prefer_prospect(&self, prospect_a: &[(f64, f64)], prospect_b: &[(f64, f64)]) -> bool {
        self.evaluate_prospect(prospect_a) > self.evaluate_prospect(prospect_b)
    }
}

/// Hyperbolic discounting model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperbolicDiscounting {
    /// Present bias parameter (beta)
    present_bias: f64,
    /// Long-term discount factor (delta)
    discount_factor: f64,
}

impl HyperbolicDiscounting {
    /// Create a new hyperbolic discounting model
    pub fn new(present_bias: f64, discount_factor: f64) -> SimResult<Self> {
        if present_bias <= 0.0 || present_bias > 1.0 {
            return Err(SimulationError::InvalidParameter(
                "Present bias must be between 0 and 1".to_string(),
            ));
        }

        if discount_factor <= 0.0 || discount_factor >= 1.0 {
            return Err(SimulationError::InvalidParameter(
                "Discount factor must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            present_bias,
            discount_factor,
        })
    }

    /// Calculate present value of future payoff
    pub fn present_value(&self, payoff: f64, period: usize) -> f64 {
        if period == 0 {
            payoff
        } else {
            self.present_bias * self.discount_factor.powi(period as i32) * payoff
        }
    }

    /// Compare immediate vs delayed reward
    pub fn prefer_immediate(&self, immediate: f64, delayed: f64, delay_periods: usize) -> bool {
        immediate > self.present_value(delayed, delay_periods)
    }
}

/// Anchoring and adjustment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchoringModel {
    /// Anchor value
    anchor: f64,
    /// Adjustment factor (0 to 1, higher = more adjustment away from anchor)
    adjustment_factor: f64,
}

impl AnchoringModel {
    /// Create a new anchoring model
    pub fn new(anchor: f64, adjustment_factor: f64) -> SimResult<Self> {
        if !(0.0..=1.0).contains(&adjustment_factor) {
            return Err(SimulationError::InvalidParameter(
                "Adjustment factor must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            anchor,
            adjustment_factor,
        })
    }

    /// Estimate value given true value and anchor
    pub fn estimate(&self, true_value: f64) -> f64 {
        self.anchor + self.adjustment_factor * (true_value - self.anchor)
    }

    /// Simulate decision making under anchoring
    pub fn decision(&self, options: &[f64]) -> usize {
        options
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = (self.estimate(**a) - self.anchor).abs();
                let dist_b = (self.estimate(**b) - self.anchor).abs();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsge_model_creation() {
        let model = DSGEModel::new(0.96, 2.0);
        assert!(model.is_ok());
        let model = model.unwrap();
        assert_eq!(model.discount_factor, 0.96);
        assert_eq!(model.risk_aversion, 2.0);
    }

    #[test]
    fn test_dsge_invalid_discount() {
        let model = DSGEModel::new(1.5, 2.0);
        assert!(model.is_err());
    }

    #[test]
    fn test_dsge_simulation() {
        let mut model = DSGEModel::new(0.96, 2.0).unwrap();
        let states = model.simulate(10);
        assert_eq!(states.len(), 10);
        for state in &states {
            assert!(state.output > 0.0);
            assert!(state.consumption > 0.0);
            assert!(state.capital > 0.0);
        }
    }

    #[test]
    fn test_dsge_steady_state() {
        let model = DSGEModel::new(0.96, 2.0).unwrap();
        let ss = model.steady_state();
        assert!(ss.output > 0.0);
        assert!(ss.consumption > 0.0);
        assert!(ss.investment > 0.0);
        assert!(ss.capital > 0.0);
    }

    #[test]
    fn test_input_output_model() {
        let sectors = vec!["Agriculture".to_string(), "Manufacturing".to_string()];
        let model = InputOutputModel::new(2, sectors);
        assert!(model.is_ok());
        let model = model.unwrap();
        assert_eq!(model.sectors, 2);
    }

    #[test]
    fn test_input_output_coefficients() {
        let sectors = vec!["A".to_string(), "B".to_string()];
        let mut model = InputOutputModel::new(2, sectors).unwrap();

        assert!(model.set_coefficient(0, 0, 0.1).is_ok());
        assert!(model.set_coefficient(0, 1, 0.2).is_ok());
        assert!(model.set_coefficient(1, 0, 0.3).is_ok());
        assert!(model.set_coefficient(1, 1, 0.15).is_ok());
    }

    #[test]
    fn test_input_output_total_output() {
        let sectors = vec!["A".to_string(), "B".to_string()];
        let mut model = InputOutputModel::new(2, sectors).unwrap();

        model.set_coefficient(0, 0, 0.1).unwrap();
        model.set_coefficient(0, 1, 0.2).unwrap();
        model.set_coefficient(1, 0, 0.3).unwrap();
        model.set_coefficient(1, 1, 0.15).unwrap();

        let final_demand = vec![100.0, 50.0];
        let result = model.calculate_total_output(&final_demand);
        assert!(result.is_ok());
        let total_output = result.unwrap();
        assert_eq!(total_output.len(), 2);
        assert!(total_output[0] > 100.0);
        assert!(total_output[1] > 50.0);
    }

    #[test]
    fn test_input_output_multipliers() {
        let sectors = vec!["A".to_string(), "B".to_string()];
        let mut model = InputOutputModel::new(2, sectors).unwrap();

        model.set_coefficient(0, 0, 0.1).unwrap();
        model.set_coefficient(1, 1, 0.1).unwrap();

        let result = model.multipliers();
        assert!(result.is_ok());
        let multipliers = result.unwrap();
        assert!(multipliers[0] > 1.0);
        assert!(multipliers[1] > 1.0);
    }

    #[test]
    fn test_financial_institution() {
        let inst = FinancialInstitution::new(0, "Bank A".to_string(), 100.0, 1000.0);
        assert_eq!(inst.capital, 100.0);
        assert_eq!(inst.assets, 1000.0);
        assert_eq!(inst.liabilities, 900.0);
        assert!(inst.is_solvent());
    }

    #[test]
    fn test_financial_contagion() {
        let inst1 = FinancialInstitution::new(0, "Bank A".to_string(), 100.0, 1000.0);
        let inst2 = FinancialInstitution::new(1, "Bank B".to_string(), 80.0, 800.0);

        let mut model = FinancialContagionModel::new(vec![inst1, inst2], 0.05);
        model.set_exposure(0, 1, 200.0).unwrap();

        let result = model.simulate_contagion(&[(0, 0.15)]);
        assert!(result.failed_institutions > 0);
        assert_eq!(result.total_institutions, 2);
    }

    #[test]
    fn test_order_book_creation() {
        let book = OrderBook::new();
        assert_eq!(book.bids.len(), 0);
        assert_eq!(book.asks.len(), 0);
    }

    #[test]
    fn test_order_submission() {
        let mut book = OrderBook::new();
        let order_id = book.submit_order(OrderType::Bid, 100.0, 10.0, 1);
        assert_eq!(order_id, 0);
        assert_eq!(book.best_bid(), Some(100.0));
    }

    #[test]
    fn test_order_matching() {
        let mut book = OrderBook::new();
        book.submit_order(OrderType::Bid, 100.0, 10.0, 1);
        book.submit_order(OrderType::Ask, 99.0, 5.0, 2);

        assert_eq!(book.transactions().len(), 1);
        assert_eq!(book.transactions()[0].quantity, 5.0);
    }

    #[test]
    fn test_bid_ask_spread() {
        let mut book = OrderBook::new();
        book.submit_order(OrderType::Bid, 99.0, 10.0, 1);
        book.submit_order(OrderType::Ask, 101.0, 10.0, 2);

        assert_eq!(book.spread(), Some(2.0));
        assert_eq!(book.mid_price(), Some(100.0));
    }

    #[test]
    fn test_market_depth() {
        let mut book = OrderBook::new();
        book.submit_order(OrderType::Bid, 100.0, 10.0, 1);
        book.submit_order(OrderType::Bid, 99.0, 20.0, 1);
        book.submit_order(OrderType::Ask, 101.0, 15.0, 2);

        let depth = book.market_depth(2);
        assert_eq!(depth.bid_depth, 30.0);
        assert_eq!(depth.ask_depth, 15.0);
    }

    #[test]
    fn test_prospect_theory() {
        let pt = ProspectTheory::new(2.25);
        let gain_value = pt.value(100.0);
        let loss_value = pt.value(-100.0);

        assert!(gain_value > 0.0);
        assert!(loss_value < 0.0);
        assert!(loss_value.abs() > gain_value);
    }

    #[test]
    fn test_prospect_evaluation() {
        let pt = ProspectTheory::new(2.25);
        let prospect = vec![(100.0, 0.5), (-50.0, 0.5)];
        let value = pt.evaluate_prospect(&prospect);
        assert!(value.is_finite());
    }

    #[test]
    fn test_hyperbolic_discounting() {
        let hd = HyperbolicDiscounting::new(0.7, 0.95);
        assert!(hd.is_ok());
        let hd = hd.unwrap();

        let pv = hd.present_value(100.0, 1);
        assert!(pv < 100.0);
        assert!(pv > 0.0);
    }

    #[test]
    fn test_hyperbolic_preference() {
        let hd = HyperbolicDiscounting::new(0.7, 0.95).unwrap();
        assert!(hd.prefer_immediate(50.0, 55.0, 1));
        assert!(!hd.prefer_immediate(50.0, 100.0, 1));
    }

    #[test]
    fn test_anchoring_model() {
        let anchor = AnchoringModel::new(100.0, 0.5);
        assert!(anchor.is_ok());
        let anchor = anchor.unwrap();

        let estimate = anchor.estimate(120.0);
        assert_eq!(estimate, 110.0);
    }

    #[test]
    fn test_anchoring_decision() {
        let anchor = AnchoringModel::new(100.0, 0.3).unwrap();
        let options = vec![80.0, 100.0, 120.0];
        let choice = anchor.decision(&options);
        assert_eq!(choice, 1); // Closest to anchor
    }
}
