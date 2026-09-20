use std::time::{Duration, SystemTime};
use std::mem::take;

use bc_runtime_components::state::State;
use bc_utils_lg::prelude::*;

#[derive(Debug, PartialEq, Default)]
pub struct DatabaseRT {
    pub info: InfoDataBase,
    pub base: Vec<MAP<String, f64>>,
    pub current_map: MAP<String, f64>,
}

#[derive(Debug, PartialEq, Default)]
pub struct InfoDataBase {
    pub symbol: String,
    pub time_create_unix: Duration,
    pub window: usize,
}

impl DatabaseRT {
    pub fn init(&mut self, symbol: String, window: usize) {
        self.info.symbol = symbol;
        self.info.window = window;
    }

    pub fn init_time(&mut self) {
        self.info.time_create_unix = SystemTime::now().elapsed().unwrap();
    }

    pub fn push_src(&mut self, src: &[f64]) {
        for (i, v) in src.iter().enumerate() {
            self.current_map.insert(format!("src_{i}", ), *v);
        }
    }

    pub fn push_state(&mut self, state: &State) {
        for (k, v) in state.indications.iter() {
            self.current_map.insert(format!("ind_{k}", ), *v);
        }
        for (k, v) in state.signals_train.iter() {
            self.current_map.insert(format!("signtr_{k}", ), *v);
        }
        for (k, v) in state.signals.iter() {
            self.current_map
                .insert(format!("sign&sign_{k}", ), v.signal);
            self.current_map
                .insert(format!("sign&prob_{k}", ), v.probability);
        }
        for (k, v) in state.utils_state.iter() {
            self.current_map
                .insert(format!("utilsstate_{k}", ), *v);
        }
        for (k, v) in state.orders.iter() {
            self.current_map
                .insert(format!("ordercreate&qty_{}", k), v.order.qty);
            self.current_map
                .insert(format!("ordercreate&commission_{}", k), v.order.commission);
            self.current_map
                .insert(format!("ordercreate&leverage_{}", k), v.order.leverage);
            self.current_map
                .insert(format!("ordercreate&istrigger_{}", k), v.is_trigger as i8 as f64);
            let trigger = v.trigger.clone().unwrap_or_default();
            self.current_map
                .insert(format!("ordercreate&triggerdirection_{}", k), trigger.direction as f64);
            self.current_map
                .insert(format!("ordercreate&pricetrigger_{}", k), trigger.price);
        }
        for (k, v) in state.orders_filtered.iter() {
            self.current_map
                .insert(format!("orderfilter&ispassed_{}", k), v.0.is_some() as i8 as f64);
            self.current_map
                .insert(format!("orderfilter&istrade_{}", k),v.1 as i8 as f64);
        }
    }

    pub fn push_trade_state(&mut self, trade_state: &TradeState) {
        for i in 0..2usize {
            let position = trade_state
                .positions
                .borrow()
                .get(&i)
                .cloned()
                .unwrap_or_default();
            self.current_map
                .insert(format!("tradestate&position&qty_{i}", ), position.qty);
            self.current_map
                .insert(format!("tradestate&position&leverage_{i}", ), position.leverage);
            self.current_map
                .insert(format!("tradestate&position&avgopenprice_{i}", ), position.avg_open_price);
            self.current_map
                .insert(format!("tradestate&position&positionidx_{i}", ), position.position_idx as f64);
            self.current_map
                .insert(format!("tradestate&position&pnlpercent_{i}", ), position.pnl_percent);
            self.current_map
                .insert(format!("tradestate&position&pnlqty_{i}", ), position.pnl_qty);
            self.current_map
                .insert(format!("tradestate&position&isactive_{i}", ), position.is_active as i8 as f64);
        }
        self.current_map
            .insert("tradestate&capital".to_string(), trade_state.capital.0);
        self.current_map
            .insert("tradestate&ordertriggerslen".to_string(),                 trade_state
                    .orders_trigger
                    .borrow()
                    .values()
                    .map(|bind| bind.len())
                    .sum::<usize>() as f64,);
        self.current_map.insert("tradestate&ordernsum".to_string(), trade_state
                .orders
                .borrow()
                .values()
                .map(|bind| bind.len())
                .sum::<usize>() as f64,);
        self.current_map
            .insert("tradestate&ordernsum".to_string(), trade_state.positions.borrow().values().count() as f64);
    }

    pub fn push_map(&mut self) {
        self.base.push(take(&mut self.current_map));
    }

    pub fn limit_window_exc(&mut self) {
        if self.base.len() >= self.info.window {
            self.base.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_runtime_components::test_state::state::STATE;
    use bc_test_kit::prelude::*;
    use bc_trade_state::test_state::trade_state::TRADE_STATE;

    #[test]
    fn push_src_res_1() {
        let mut db = DatabaseRT::default();
        db.push_src(&SRC_EL);
        db.push_map();
        assert_eq_pr!(
            db.base[0].len(),
            SRC_EL.len()
        );
        assert_eq_pr!(db.base[0]["src_0"], SRC_EL[0]);
        assert_eq_pr!(db.base[0]["src_1"], SRC_EL[1]);
        db.push_src(&SRC_EL1);
        db.push_map();
        assert_eq_pr!(
            db.base[1].len(),
            SRC_EL.len()
        );
        assert_eq_pr!(db.base[0]["src_0"], SRC_EL[0]);
        assert_eq_pr!(db.base[0]["src_1"], SRC_EL[1]);
        assert_eq_pr!(db.base[1]["src_0"], SRC_EL1[0]);
        assert_eq_pr!(db.base[1]["src_1"], SRC_EL1[1]);
    }

    #[test]
    fn push_state_res_1() {
        let mut db = DatabaseRT::default();
        let state = STATE();
        db.push_state(&state);
        db.push_map();
        assert_eq_pr!(db.base[0]["ind_rma_1"], state.indications["rma_1"]);
        assert_eq_pr!(
            db.base[0]["sign&sign_signal"],
            state.signals["signal"].signal
        );
        assert_eq_pr!(
            db.base[0]["sign&prob_signal"],
            state.signals["signal"].probability
        );
        db.push_state(&state);
        db.push_map();
        assert_eq_pr!(db.base[1]["ind_rma_1"], state.indications["rma_1"]);
        assert_eq_pr!(
            db.base[1]["sign&sign_signal"],
            state.signals["signal"].signal
        );
        assert_eq_pr!(
            db.base[1]["sign&prob_signal"],
            state.signals["signal"].probability
        );
    }

    #[test]
    fn push_trade_state_res_1() {
        let mut db = DatabaseRT::default();
        let trade_state = TRADE_STATE();
        db.push_trade_state(&trade_state);
        db.push_map();
        assert_eq_pr!(db.base[0]["tradestate&capital"], trade_state.capital.0);
        db.push_trade_state(&trade_state);
        db.push_map();
        assert_eq_pr!(db.base[1]["tradestate&capital"], trade_state.capital.0);
    }
}
