use std::time::{Duration, SystemTime};
use bc_runtime_components::state::State;
use bc_utils_lg::prelude::*;

#[derive(Debug, PartialEq, Default)]
pub struct DataBase {
    pub info: InfoDataBase,
    pub base: MAP<String, Vec<f64>>,
}

#[derive(Debug, PartialEq, Default)]
pub struct InfoDataBase {
    pub symbol: String,
    pub time_create_unix: Duration,
    pub window: usize,
}

impl DataBase {
    pub fn init(&mut self, symbol: String, window: usize) {
        self.info.symbol = symbol;
        self.info.window = window;
    }

    pub fn init_time(&mut self) {
        self.info.time_create_unix = SystemTime::now().elapsed().unwrap();
    }

    pub fn init_src(&mut self, src: &[f64]) {
        for i in 0..src.len() {
            self.base.insert(format!("src_{i}"), vec![]);
        }
    }

    pub fn push_src(&mut self, src: &[f64]) {
        for (i, v) in src.iter().enumerate() {
            self.base.get_mut(&format!("src_{i}")).unwrap().push(*v);
        }
    }

    pub fn init_state(&mut self, settings_pipeline: &SETTINGS_PIPELINE) {
        for k in settings_pipeline.indications.keys() {
            self.base.insert(format!("ind_{}", k), vec![]);
        }
        for k in settings_pipeline.signals.keys() {
            self.base.insert(format!("sign&sign_{}", k), vec![]);
            self.base.insert(format!("sign&prob_{}", k), vec![]);
        }
        for k in settings_pipeline.signals_train.keys() {
            self.base.insert(format!("signtr_{}", k), vec![]);
        }
        for k in settings_pipeline.utils_state.keys() {
            self.base.insert(format!("utilsstate_{}", k), vec![]);
        }
        for k in settings_pipeline.order_creators.keys() {
            self.base.insert(format!("ordercreate&qty_{}", k), vec![]);
            self.base
                .insert(format!("ordercreate&commission_{}", k), vec![]);
            self.base
                .insert(format!("ordercreate&leverage_{}", k), vec![]);
            self.base
                .insert(format!("ordercreate&istrigger_{}", k), vec![]);
            self.base
                .insert(format!("ordercreate&triggerdirection_{}", k), vec![]);
            self.base
                .insert(format!("ordercreate&pricetrigger_{}", k), vec![]);
        }
        for k in settings_pipeline.order_filters.keys() {
            self.base
                .insert(format!("orderfilter&ispassed_{}", k), vec![]);
            self.base
                .insert(format!("orderfilter&istrade_{}", k), vec![]);
        }
    }

    pub fn push_state(&mut self, state: &State) {
        for (k, v) in state.indications.iter() {
            self.base.get_mut(&format!("ind_{k}")).unwrap().push(*v);
        }
        for (k, v) in state.signals_train.iter() {
            self.base.get_mut(&format!("signtr_{k}")).unwrap().push(*v);
        }
        for (k, v) in state.signals.iter() {
            self.base
                .get_mut(&format!("sign&sign_{k}"))
                .unwrap()
                .push(v.signal);
            self.base
                .get_mut(&format!("sign&prob_{k}"))
                .unwrap()
                .push(v.probability);
        }
        for (k, v) in state.utils_state.iter() {
            self.base
                .get_mut(&format!("utilsstate_{k}"))
                .unwrap()
                .push(*v);
        }
        for (k, v) in state.orders.iter() {
            self.base
                .get_mut(&format!("ordercreate&qty_{}", k))
                .unwrap()
                .push(v.order.qty);
            self.base
                .get_mut(&format!("ordercreate&commission_{}", k))
                .unwrap()
                .push(v.order.commission);
            self.base
                .get_mut(&format!("ordercreate&leverage_{}", k))
                .unwrap()
                .push(v.order.leverage);
            self.base
                .get_mut(&format!("ordercreate&istrigger_{}", k))
                .unwrap()
                .push(v.is_trigger as i8 as f64);
            let trigger = v.trigger.clone().unwrap_or_default();
            self.base
                .get_mut(&format!("ordercreate&triggerdirection_{}", k))
                .unwrap()
                .push(trigger.direction as f64);
            self.base
                .get_mut(&format!("ordercreate&pricetrigger_{}", k))
                .unwrap()
                .push(trigger.price);
        }
        for (k, v) in state.orders_filtered.iter() {
            self.base
                .get_mut(&format!("orderfilter&ispassed_{}", k))
                .unwrap()
                .push(v.0.is_some() as i8 as f64);
            self.base
                .get_mut(&format!("orderfilter&istrade_{}", k))
                .unwrap()
                .push(v.1 as i8 as f64);
        }
    }

    pub fn init_trade_state(&mut self) {
        for i in 0..2 {
            self.base
                .insert(format!("tradestate&position&qty_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&leverage_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&avgopenprice_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&positionidx_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&pnlpercent_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&pnlqty_{i}"), vec![]);
            self.base
                .insert(format!("tradestate&position&isactive_{i}"), vec![]);
        }
        self.base.insert("tradestate&capital".to_string(), vec![]);
        self.base
            .insert("tradestate&ordertriggerslen".to_string(), vec![]);
        self.base.insert("tradestate&ordernsum".to_string(), vec![]);
        self.base.insert("tradestate&ordernsum".to_string(), vec![]);
    }

    pub fn push_trade_state(&mut self, trade_state: &TradeState) {
        for i in 0..2usize {
            let position = trade_state
                .positions
                .borrow()
                .get(&i)
                .cloned()
                .unwrap_or_default();
            self.base
                .get_mut(&format!("tradestate&position&qty_{i}"))
                .unwrap()
                .push(position.qty);
            self.base
                .get_mut(&format!("tradestate&position&leverage_{i}"))
                .unwrap()
                .push(position.leverage);
            self.base
                .get_mut(&format!("tradestate&position&avgopenprice_{i}"))
                .unwrap()
                .push(position.avg_open_price);
            self.base
                .get_mut(&format!("tradestate&position&positionidx_{i}"))
                .unwrap()
                .push(position.position_idx as f64);
            self.base
                .get_mut(&format!("tradestate&position&pnlpercent_{i}"))
                .unwrap()
                .push(position.pnl_percent);
            self.base
                .get_mut(&format!("tradestate&position&pnlqty_{i}"))
                .unwrap()
                .push(position.pnl_qty);
            self.base
                .get_mut(&format!("tradestate&position&isactive_{i}"))
                .unwrap()
                .push(position.is_active as i8 as f64);
        }
        self.base
            .get_mut("tradestate&capital")
            .unwrap()
            .push(trade_state.capital.0);
        self.base
            .get_mut("tradestate&ordertriggerslen")
            .unwrap()
            .push(
                trade_state
                    .orders_trigger
                    .borrow()
                    .values()
                    .map(|bind| bind.len())
                    .sum::<usize>() as f64,
            );
        self.base.get_mut("tradestate&ordernsum").unwrap().push(
            trade_state
                .orders
                .borrow()
                .values()
                .map(|bind| bind.len())
                .sum::<usize>() as f64,
        );
        self.base
            .get_mut("tradestate&ordernsum")
            .unwrap()
            .push(trade_state.positions.borrow().values().count() as f64);
    }

    pub fn limit_window_exc(&mut self) {
        if self.base.len() >= self.info.window {
            for v in self.base.values_mut() {
                v.remove(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_runtime_components::test_state::state::STATE;
    use bc_test_kit::prelude::*;
    use bc_trade_state::test_state::trade_state::TRADE_STATE;
use bc_utils_lg::test_state::prelude::*;

    #[test]
    fn init_res_1() {
        let mut db = DataBase::default();
        db.init("symbol".to_string(), 10);
        assert_eq_pr!(
            db,
            DataBase {
                info: InfoDataBase {
                    symbol: "symbol".to_string(),
                    window: 10,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    }

    #[test]
    fn init_src_res_1() {
        let mut db = DataBase::default();
        db.init_src(&SRC_EL);
        assert_eq_pr!(
            db,
            DataBase {
                base: (0..SRC_EL.len())
                    .map(|i| (format!("src_{i}"), vec![]))
                    .collect::<MAP<String, Vec<f64>>>(),
                ..Default::default()
            }
        )
    }

    #[test]
    fn init_state_res_1() {
        let mut db = DataBase::default();
        db.init_state(&PIPELINE);
        assert_eq_pr!(
            db,
            DataBase {
                base: MAP::from_iter([
                    ("ordercreate&pricetrigger_open_order".to_string(), vec![],),
                    ("ordercreate&pricetrigger_order".to_string(), vec![],),
                    ("ordercreate&qty_sl".to_string(), vec![],),
                    ("ordercreate&commission_sl".to_string(), vec![],),
                    ("sign&sign_invert_1".to_string(), vec![],),
                    ("ordercreate&commission_open_order".to_string(), vec![],),
                    ("orderfilter&ispassed_side_1".to_string(), vec![],),
                    ("ind_rma_1".to_string(), vec![],),
                    ("sign&sign_signal".to_string(), vec![],),
                    ("orderfilter&ispassed_count_1".to_string(), vec![],),
                    ("orderfilter&istrade_wrap".to_string(), vec![],),
                    ("ordercreate&commission_order".to_string(), vec![],),
                    ("ordercreate&istrigger_sl".to_string(), vec![],),
                    ("ordercreate&commission_avg_order".to_string(), vec![],),
                    ("utilsstate_direction_1".to_string(), vec![],),
                    ("sign&prob_invert_1".to_string(), vec![],),
                    ("orderfilter&ispassed_count_3".to_string(), vec![],),
                    ("ordercreate&triggerdirection_sl".to_string(), vec![],),
                    ("orderfilter&istrade_count_3".to_string(), vec![],),
                    ("ordercreate&leverage_avg_order".to_string(), vec![],),
                    ("ordercreate&istrigger_open_order".to_string(), vec![],),
                    ("sign&sign_th_1".to_string(), vec![],),
                    ("ind_sma_1".to_string(), vec![],),
                    ("ordercreate&pricetrigger_avg_order".to_string(), vec![],),
                    ("ordercreate&leverage_order".to_string(), vec![],),
                    ("ordercreate&istrigger_order".to_string(), vec![],),
                    ("orderfilter&ispassed_count_2".to_string(), vec![],),
                    ("utilsstate_qty_1".to_string(), vec![],),
                    ("ordercreate&qty_avg_order".to_string(), vec![],),
                    ("sign&prob_signal".to_string(), vec![],),
                    ("orderfilter&istrade_count_2".to_string(), vec![],),
                    ("ordercreate&triggerdirection_order".to_string(), vec![],),
                    ("orderfilter&ispassed_wrap".to_string(), vec![],),
                    ("signtr_mm_1".to_string(), vec![],),
                    ("ordercreate&leverage_open_order".to_string(), vec![],),
                    ("ordercreate&qty_order".to_string(), vec![],),
                    ("orderfilter&istrade_count_1".to_string(), vec![],),
                    ("ordercreate&istrigger_avg_order".to_string(), vec![],),
                    ("ordercreate&pricetrigger_sl".to_string(), vec![],),
                    ("ordercreate&leverage_sl".to_string(), vec![],),
                    (
                        "ordercreate&triggerdirection_open_order".to_string(),
                        vec![],
                    ),
                    ("orderfilter&istrade_side_1".to_string(), vec![],),
                    ("sign&prob_th_1".to_string(), vec![],),
                    ("ordercreate&triggerdirection_avg_order".to_string(), vec![],),
                    ("ordercreate&qty_open_order".to_string(), vec![],),
                ]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn init_trade_state_res_1() {
        let mut db = DataBase::default();
        db.init_trade_state();
        assert_eq_pr!(
            db,
            DataBase {
                base: MAP::from_iter([
                    (format!("tradestate&position&qty_0"), vec![]),
                    (format!("tradestate&position&leverage_0"), vec![]),
                    (format!("tradestate&position&avgopenprice_0"), vec![]),
                    (format!("tradestate&position&positionidx_0"), vec![]),
                    (format!("tradestate&position&pnlpercent_0"), vec![]),
                    (format!("tradestate&position&pnlqty_0"), vec![]),
                    (format!("tradestate&position&isactive_0"), vec![]),
                    (format!("tradestate&position&qty_1"), vec![]),
                    (format!("tradestate&position&leverage_1"), vec![]),
                    (format!("tradestate&position&avgopenprice_1"), vec![]),
                    (format!("tradestate&position&positionidx_1"), vec![]),
                    (format!("tradestate&position&pnlpercent_1"), vec![]),
                    (format!("tradestate&position&pnlqty_1"), vec![]),
                    (format!("tradestate&position&isactive_1"), vec![]),
                    ("tradestate&capital".to_string(), vec![]),
                    ("tradestate&ordertriggerslen".to_string(), vec![]),
                    ("tradestate&ordernsum".to_string(), vec![]),
                    ("tradestate&ordernsum".to_string(), vec![]),
                ]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn push_src_res_1() {
        let mut db = DataBase::default();
        db.init_src(&SRC_EL);
        db.push_src(&SRC_EL);
        assert_eq_pr!(
            db.base.values().map(|v| v.len()).sum::<usize>(),
            SRC_EL.len()
        );
        assert_eq_pr!(db.base["src_0"][0], SRC_EL[0]);
        assert_eq_pr!(db.base["src_1"][0], SRC_EL[1]);
        db.push_src(&SRC_EL1);
        assert_eq_pr!(
            db.base.values().map(|v| v.len()).sum::<usize>(),
            SRC_EL.len() * 2
        );
        assert_eq_pr!(db.base["src_0"][0], SRC_EL[0]);
        assert_eq_pr!(db.base["src_1"][0], SRC_EL[1]);
        assert_eq_pr!(db.base["src_0"][1], SRC_EL1[0]);
        assert_eq_pr!(db.base["src_1"][1], SRC_EL1[1]);
    }

    #[test]
    fn push_state_res_1() {
        let mut db = DataBase::default();
        let state = STATE();
        db.init_state(&PIPELINE);
        db.push_state(&state);
        assert_eq_pr!(db.base["ind_rma_1"][0], state.indications["rma_1"]);
        assert_eq_pr!(
            db.base["sign&sign_signal"][0],
            state.signals["signal"].signal
        );
        assert_eq_pr!(
            db.base["sign&prob_signal"][0],
            state.signals["signal"].probability
        );
        db.push_state(&state);
        assert_eq_pr!(db.base["ind_rma_1"][1], state.indications["rma_1"]);
        assert_eq_pr!(
            db.base["sign&sign_signal"][1],
            state.signals["signal"].signal
        );
        assert_eq_pr!(
            db.base["sign&prob_signal"][1],
            state.signals["signal"].probability
        );
    }

    #[test]
    fn push_trade_state_res_1() {
        let mut db = DataBase::default();
        let trade_state = TRADE_STATE();
        db.init_trade_state();
        db.push_trade_state(&trade_state);
        assert_eq_pr!(db.base["tradestate&capital"][0], trade_state.capital.0);
        db.push_trade_state(&trade_state);
        assert_eq_pr!(db.base["tradestate&capital"][1], trade_state.capital.0);
    }
}
