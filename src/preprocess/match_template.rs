use std::{cell::RefCell, collections::HashMap, sync::Arc};

use multimap::MultiMap;

use crate::{error::SyntaxMatchError, values::*};

#[derive(Debug, Default)]
pub struct MatchRecord {
    pub maps: RefCell<HashMap<Arc<Symbol>, Value>>,
    pub extend_maps: RefCell<HashMap<Arc<Symbol>, NodeExtend>>,
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

        (Value::Pair(a), Value::Pair(b)) => {
            let a_lst: Vec<Value> = a.iter().collect();
            let b_lst: Vec<Value> = b.iter().collect();
            if !(!a_lst.is_empty()
                && !b_lst.is_empty()
                && a_lst.last().unwrap().clone() == Value::Sym(Arc::new(Symbol::new("..."))))
            {
                if a.len() != b.len() {
                    return Err(SyntaxMatchError::MatchListSizeError);
                }
                a.iter()
                    .zip(b.iter())
                    .try_for_each(|(x, y)| match_template(record, &x, &y))?;
                return Ok(());
            }

            println!("lens: {}, {}", a.len(), b.len());
            if a.len() - 2 > b.len() {
                return Err(SyntaxMatchError::MatchListSizeError);
            }
            let extend_temp = unsafe { a_lst.get_unchecked(a_lst.len() - 2) };
            let mut a_lst = a_lst[0..a_lst.len() - 2].iter();

            println!("a_lst len: {}", a_lst.len());
            a_lst.try_for_each(|x| {
                if *x == Value::Sym(Arc::new(Symbol::new("..."))) {
                    Err(SyntaxMatchError::ExtendInMiddleError(x.get_sym().unwrap()))
                } else {
                    Ok(())
                }
            })?;
            let b_expand_lst = b_lst[a_lst.len()..b_lst.len()].iter();
            let b_lst = b_lst[0..a_lst.len()].iter();

            println!("b_lst len: {}", b_lst.len());

            a_lst
                .zip(b_lst)
                .try_for_each(|(a, b)| match_template(record, &a, b))?;

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
                    rr.insert(k, v.into());
                }
            }

            for (k, v) in rr {
                let v: NodeExtend = v.iter().map(Value::clone).collect();
                record.extend_maps.get_mut().insert(k, v);
            }
        }
        (Value::Pair(a), b) => {
            let name = a.cdr().get_pair().unwrap().car();
            if !(a.len() == 2 && name.get_sym().is_some()) {
                return Err(SyntaxMatchError::MatchError);
            }
            if !(a.car().get_sym().is_some()) {
                return Err(SyntaxMatchError::MatchError);
            }
            let tp = a.car().get_sym().unwrap();
            let tpid = &*tp.id;
            if tpid == "quote" && name == b {
                return Ok(());
            }
            if !(tpid == "$sym" && b.get_sym().is_some()
                || tpid == "$str" && b.get_str().is_some()
                || tpid == "$int" && b.get_int().is_some()
                || tpid == "$uint" && b.get_uint().is_some()
                || tpid == "$bool" && b.get_bool().is_some()
                || tpid == "$char" && b.get_char().is_some()
                || tpid == "$list" && b.get_pair().is_some()
                || tpid == "$float" && b.get_float().is_some())
            {
                return Err(SyntaxMatchError::SExprTypeCheckError(tp.clone()));
            }
            let name = name.get_sym().unwrap();
            record
                .maps
                .get_mut()
                .insert(name.clone(), b)
                .map_or(Ok(()), |_| {
                    Err(SyntaxMatchError::RepeatedSymbol(name.clone()))
                })?;
        }
        _ => {
            if temp != inp {
                return Err(SyntaxMatchError::MatchError);
            }
        }
    }
    Ok(())
}
