use bc_utils_lg::prelude::*;
use std::mem::take;
use std::time::{Duration, SystemTime};

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
    use crate::prelude::*;
    use bc_runtime_components::test_state::state::STATE;
    use bc_test_kit::prelude::*;
    use bc_trade_state::test_state::trade_state::TRADE_STATE;

    #[test]
    fn push_src_res_1() {
        let mut db = DatabaseRT::default();
        SRC_EL.as_slice().transform(&mut db.current_map, "");
        db.push_map();
        assert_eq_pr!(db.base[0].len(), SRC_EL.len());
        assert_eq_pr!(db.base[0]["src_0"], SRC_EL[0]);
        assert_eq_pr!(db.base[0]["src_1"], SRC_EL[1]);
        SRC_EL1.as_slice().transform(&mut db.current_map, "");
        db.push_map();
        assert_eq_pr!(db.base[1].len(), SRC_EL.len());
        assert_eq_pr!(db.base[0]["src_0"], SRC_EL[0]);
        assert_eq_pr!(db.base[0]["src_1"], SRC_EL[1]);
        assert_eq_pr!(db.base[1]["src_0"], SRC_EL1[0]);
        assert_eq_pr!(db.base[1]["src_1"], SRC_EL1[1]);
    }

    #[test]
    fn push_state_res_1() {
        let mut db = DatabaseRT::default();
        let state = STATE();
        StateDeserialize::from(&state).transform(&mut db.current_map, "");
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
        StateDeserialize::from(&state).transform(&mut db.current_map, "");
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
        trade_state.transform(&mut db.current_map, "");
        db.push_map();
        assert_eq_pr!(db.base[0]["tradestate&capital"], trade_state.capital.0);
        trade_state.transform(&mut db.current_map, "");
        db.push_map();
        assert_eq_pr!(db.base[1]["tradestate&capital"], trade_state.capital.0);
    }
}
