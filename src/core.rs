use bc_runtime_components::state::State;
use bc_utils_lg::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct StateDeserialize {
    pub indications: MAP<String, f64>,
    pub signals_train: MAP<String, f64>,
    pub signals: MAP<String, Signal>,
    pub utils_state: MAP<String, f64>,
    pub orders: MAP<String, OrderWrap>,
    pub orders_filtered: MAP<String, (bool, bool)>,
}

impl<'a> From<&State<'a>> for StateDeserialize {
    fn from(value: &State) -> Self {
        StateDeserialize {
            indications: value
                .indications
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            signals_train: value
                .signals_train
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            signals: value
                .signals
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            utils_state: value
                .utils_state
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            orders: value
                .orders
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
            orders_filtered: value
                .orders_filtered
                .iter()
                .map(|(k, v)| (k.to_string(), (v.0.is_some(), v.1)))
                .collect(),
        }
    }
}

pub trait TransformToMapf64 {
    fn transform(&self, map: &mut MAP<String, f64>);
}

impl TransformToMapf64 for &[f64] {
    fn transform(&self, map: &mut MAP<String, f64>) {
        for (i, v) in self.iter().enumerate() {
            map.insert(format!("src_{i}",), *v);
        }
    }
}

impl TransformToMapf64 for StateDeserialize {
    fn transform(&self, map: &mut MAP<String, f64>) {
        for (k, v) in self.indications.iter() {
            map.insert(format!("ind_{k}",), *v);
        }
        for (k, v) in self.signals_train.iter() {
            map.insert(format!("signtr_{k}",), *v);
        }
        for (k, v) in self.signals.iter() {
            map.insert(format!("sign&sign_{k}",), v.signal);
            map.insert(format!("sign&prob_{k}",), v.probability);
        }
        for (k, v) in self.utils_state.iter() {
            map.insert(format!("utilsstate_{k}",), *v);
        }
        for (k, v) in self.orders.iter() {
            map.insert(format!("ordercreate&qty_{}", k), v.order.qty);
            map.insert(format!("ordercreate&commission_{}", k), v.order.commission);
            map.insert(format!("ordercreate&leverage_{}", k), v.order.leverage);
            map.insert(
                format!("ordercreate&istrigger_{}", k),
                v.is_trigger as i8 as f64,
            );
            let trigger = v.trigger.clone().unwrap_or_default();
            map.insert(
                format!("ordercreate&triggerdirection_{}", k),
                trigger.direction as f64,
            );
            map.insert(format!("ordercreate&pricetrigger_{}", k), trigger.price);
        }
        for (k, v) in self.orders_filtered.iter() {
            map.insert(format!("orderfilter&ispassed_{}", k), v.0 as i8 as f64);
            map.insert(format!("orderfilter&istrade_{}", k), v.1 as i8 as f64);
        }
    }
}

impl<'a> TransformToMapf64 for TradeState<'a> {
    fn transform(&self, map: &mut MAP<String, f64>) {
        for i in 0..2usize {
            let position = self.positions.borrow().get(&i).cloned().unwrap_or_default();
            map.insert(format!("tradestate&position&qty_{i}",), position.qty);
            map.insert(
                format!("tradestate&position&leverage_{i}",),
                position.leverage,
            );
            map.insert(
                format!("tradestate&position&avgopenprice_{i}",),
                position.avg_open_price,
            );
            map.insert(
                format!("tradestate&position&positionidx_{i}",),
                position.position_idx as f64,
            );
            map.insert(
                format!("tradestate&position&pnlpercent_{i}",),
                position.pnl_percent,
            );
            map.insert(format!("tradestate&position&pnlqty_{i}",), position.pnl_qty);
            map.insert(
                format!("tradestate&position&isactive_{i}",),
                position.is_active as i8 as f64,
            );
        }
        map.insert("tradestate&capital".to_string(), self.capital.0);
        map.insert(
            "tradestate&ordertriggerslen".to_string(),
            self.orders_trigger
                .borrow()
                .values()
                .map(|bind| bind.len())
                .sum::<usize>() as f64,
        );
        map.insert(
            "tradestate&ordernsum".to_string(),
            self.orders
                .borrow()
                .values()
                .map(|bind| bind.len())
                .sum::<usize>() as f64,
        );
        map.insert(
            "tradestate&ordernsum".to_string(),
            self.positions.borrow().values().count() as f64,
        );
    }
}
