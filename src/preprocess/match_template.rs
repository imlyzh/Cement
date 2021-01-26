use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    sync::Arc,
};

use multimap::MultiMap;

use crate::{error::SyntaxMatchError, values::*};


#[derive(Debug, Default)]
pub struct MatchRecord {
    pub maps: RefCell<HashMap<Arc<Symbol>, Value>>,
    pub extend_maps: RefCell<HashMap<Arc<Symbol>, ListPia>>,
}

pub fn match_template(
    record: &mut MatchRecord,
    temp: &Value,
    inp: &Value,
) -> Result<(), SyntaxMatchError> {
    match (temp.clone(), inp.clone()) {
        (Value::Sym(id), v) => {
            record
                .maps
                .get_mut()
				.insert(id.clone(), v)
                .map_or(Ok(()), |_| Err(SyntaxMatchError::RepeatedSymbol(id)))?;
        }

        (Value::List(a), Value::List(b)) => {
            if !a.0.is_empty()
                && !b.0.is_empty()
                && *a.clone().0.back().unwrap() == Value::Sym(Arc::new(Symbol::new("...")))
            {
                if a.clone().0.len() - 1 > b.0.len() {
                    return Err(SyntaxMatchError::MatchListSizeError);
                }
                let mut a_lst = a.0.clone();
                a_lst.pop_back().unwrap();
                let extend_temp = a_lst.pop_back().unwrap();

                a_lst.iter().try_for_each(|x| {
                    if *x == Value::Sym(Arc::new(Symbol::new("..."))) {
                        Err(SyntaxMatchError::ExtendInMiddleError)
                    } else {
                        Ok(())
                    }
                })?;

                let b_lst: Vec<Value> = b.0.clone().iter().map(Value::clone).collect();
                let b_expand_lst = b_lst[a_lst.len()..b_lst.len()].iter();
                let b_lst = b_lst[0..a_lst.len()].iter();

                a_lst.iter()
                    .zip(b_lst)
                    .try_for_each(|(a, b)| match_template(record, a, b))?;

                let r = b_expand_lst
                    .map(|x| -> Result<MatchRecord, SyntaxMatchError> {
                        let mut new_ctx = MatchRecord::default();
                        match_template(&mut new_ctx, &extend_temp, x)?;
                        Ok(new_ctx)
                    })
                    .collect::<Result<Vec<MatchRecord>, SyntaxMatchError>>()?;
                let mut rr = MultiMap::new();
                for x in r {
                    let maps = x.maps.try_borrow().unwrap().clone();
                    for (k, v) in maps {
                        rr.insert(k, v);
                    }
                    let extends = x.extend_maps.try_borrow().unwrap().clone();
                    for (k, v) in extends {
                        rr.insert(k, Value::List(Arc::new(List(v))));
                    }
                }

                for (k, v) in rr {
                    let v: LinkedList<Value> = v.iter().map(Value::clone).collect();
                    record.extend_maps.get_mut().insert(k, v);
                }
            } else {
				if a.0.len() != b.0.len() {
                    return Err(SyntaxMatchError::MatchListSizeError);
                }
				a.as_ref()
                .0
                .iter()
                .zip(b.as_ref().0.iter())
                .try_for_each(|(x, y)| match_template(record, x, y))?;
			}
        }
        (Value::List(a), Value::Sym(b)) => {
            if a.0.len() == 2 {
                if let Value::Sym(k) = a.0.front().unwrap() {
                    if k.id == "quote" {
                        if *a.0.back().unwrap() == Value::Sym(b) {
                            return Ok(());
                        }
                    }
                }
            }
            return Err(SyntaxMatchError::MatchError);
        }
        _ => {
            if temp != inp {
                return Err(SyntaxMatchError::MatchError);
            }
        }
    }
    Ok(())
}
