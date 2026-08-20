#![allow(warnings)]

use perceus_ptr::PerceusPtr;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Number(f64),
    Bool(bool),
    String(String),
    Char(char),
    Array(std::rc::Rc<Vec<UnknownType>>),
    Func(std::rc::Rc<dyn Fn(UnknownType) -> UnknownType>),
    Record(perceus_ptr::PerceusPtr<Record_a>),
    Class(std::rc::Rc<dyn std::any::Any>),
}

impl Value {
    pub fn unwrap_int(&self) -> i64 {
        if let Value::Int(v) = self { *v } else { panic!("Expected Int"); }
    }
    pub fn unwrap_number(&self) -> f64 {
        if let Value::Number(v) = self { *v } else { panic!("Expected Number"); }
    }
    pub fn unwrap_bool(&self) -> bool {
        if let Value::Bool(v) = self { *v } else { panic!("Expected Bool"); }
    }
    pub fn unwrap_string(&self) -> String {
        if let Value::String(v) = self { v.clone() } else { panic!("Expected String"); }
    }
    pub fn unwrap_char(&self) -> char {
        if let Value::Char(v) = self { *v } else { panic!("Expected Char"); }
    }
    pub fn unwrap_array(&self) -> std::rc::Rc<Vec<UnknownType>> {
        if let Value::Array(v) = self { v.clone() } else { panic!("Expected Array"); }
    }
    pub fn unwrap_func(&self) -> std::rc::Rc<dyn Fn(UnknownType) -> UnknownType> {
        if let Value::Func(v) = self { v.clone() } else if let Value::Record(v) = self { v.call.clone().unwrap() } else { panic!("Expected Func"); }
    }
    pub fn unwrap_record(&self) -> perceus_ptr::PerceusPtr<Record_a> {
        if let Value::Record(v) = self { v.clone() } else { panic!("Expected Record"); }
    }
    pub fn as_record_mut(&mut self) -> &mut perceus_ptr::PerceusPtr<Record_a> {
        if let Value::Record(v) = self { v } else { panic!("Expected Record"); }
    }
    pub fn unwrap_class<T: 'static>(&self) -> &T {
        if let Value::Class(v) = self { v.downcast_ref::<T>().unwrap() } else { panic!("Expected Class"); }
    }
    pub fn drop_explicit(self) {
        if let Value::Record(v) = self { v.drop_explicit(); }
    }
    pub fn new(r: Record_a) -> Self {
        Value::Record(perceus_ptr::PerceusPtr::new(r))
    }
}

pub type UnknownType = Value;

pub fn mk_int(val: i64) -> UnknownType { Value::Int(val) }
pub fn mk_bool(val: bool) -> UnknownType { Value::Bool(val) }
pub fn mk_number(val: f64) -> UnknownType { Value::Number(val) }
pub fn mk_string(val: &str) -> UnknownType { Value::String(val.to_string()) }
pub fn mk_char(val: char) -> UnknownType { Value::Char(val) }
pub fn mk_array(val: Vec<UnknownType>) -> UnknownType { Value::Array(std::rc::Rc::new(val)) }

#[derive(Clone, Default)]
pub struct Record_a {
    pub tag: &'static str,
    pub vals: Option<std::rc::Rc<Vec<UnknownType>>>,
    pub call: Option<std::rc::Rc<dyn Fn(UnknownType) -> UnknownType>>,
    pub Applicative0: Option<UnknownType>,
    pub Apply0: Option<UnknownType>,
    pub Bind1: Option<UnknownType>,
    pub CommutativeRing0: Option<UnknownType>,
    pub DivisionRing1: Option<UnknownType>,
    pub Eq0: Option<UnknownType>,
    pub Eq10: Option<UnknownType>,
    pub EqRecord0: Option<UnknownType>,
    pub EuclideanRing0: Option<UnknownType>,
    pub Functor0: Option<UnknownType>,
    pub HeytingAlgebra0: Option<UnknownType>,
    pub HeytingAlgebraRecord0: Option<UnknownType>,
    pub Monad0: Option<UnknownType>,
    pub Ord0: Option<UnknownType>,
    pub OrdRecord0: Option<UnknownType>,
    pub Ring0: Option<UnknownType>,
    pub RingRecord0: Option<UnknownType>,
    pub Semigroup0: Option<UnknownType>,
    pub SemigroupRecord0: Option<UnknownType>,
    pub Semigroupoid0: Option<UnknownType>,
    pub Semiring0: Option<UnknownType>,
    pub SemiringRecord0: Option<UnknownType>,
    pub add: Option<UnknownType>,
    pub addRecord: Option<UnknownType>,
    pub append: Option<UnknownType>,
    pub appendRecord: Option<UnknownType>,
    pub apply: Option<UnknownType>,
    pub bind: Option<UnknownType>,
    pub bottom: Option<UnknownType>,
    pub bottomRecord: Option<UnknownType>,
    pub compare: Option<UnknownType>,
    pub compare1: Option<UnknownType>,
    pub compareRecord: Option<UnknownType>,
    pub compose: Option<UnknownType>,
    pub conj: Option<UnknownType>,
    pub conjRecord: Option<UnknownType>,
    pub degree: Option<UnknownType>,
    pub discard: Option<UnknownType>,
    pub disj: Option<UnknownType>,
    pub disjRecord: Option<UnknownType>,
    pub div: Option<UnknownType>,
    pub eq: Option<UnknownType>,
    pub eq1: Option<UnknownType>,
    pub eqRecord: Option<UnknownType>,
    pub ff: Option<UnknownType>,
    pub ffRecord: Option<UnknownType>,
    pub from: Option<UnknownType>,
    pub genericAdd_prime: Option<UnknownType>,
    pub genericAppend_prime: Option<UnknownType>,
    pub genericBottom_prime: Option<UnknownType>,
    pub genericCompare_prime: Option<UnknownType>,
    pub genericConj_prime: Option<UnknownType>,
    pub genericDisj_prime: Option<UnknownType>,
    pub genericEq_prime: Option<UnknownType>,
    pub genericFF_prime: Option<UnknownType>,
    pub genericImplies_prime: Option<UnknownType>,
    pub genericMempty_prime: Option<UnknownType>,
    pub genericMul_prime: Option<UnknownType>,
    pub genericNot_prime: Option<UnknownType>,
    pub genericOne_prime: Option<UnknownType>,
    pub genericShow_prime: Option<UnknownType>,
    pub genericShowArgs: Option<UnknownType>,
    pub genericSub_prime: Option<UnknownType>,
    pub genericTT_prime: Option<UnknownType>,
    pub genericTop_prime: Option<UnknownType>,
    pub genericZero_prime: Option<UnknownType>,
    pub identity: Option<UnknownType>,
    pub implies: Option<UnknownType>,
    pub impliesRecord: Option<UnknownType>,
    pub liftEffect: Option<UnknownType>,
    pub map: Option<UnknownType>,
    pub mempty: Option<UnknownType>,
    pub memptyRecord: Option<UnknownType>,
    pub mod_kw: Option<UnknownType>,
    pub mul: Option<UnknownType>,
    pub mulRecord: Option<UnknownType>,
    pub not: Option<UnknownType>,
    pub notRecord: Option<UnknownType>,
    pub one: Option<UnknownType>,
    pub oneRecord: Option<UnknownType>,
    pub pure: Option<UnknownType>,
    pub purust_minus_hello_minus_world: Option<UnknownType>,
    pub recip: Option<UnknownType>,
    pub reflectSymbol: Option<UnknownType>,
    pub reflectType: Option<UnknownType>,
    pub show: Option<UnknownType>,
    pub showRecordFields: Option<UnknownType>,
    pub sub: Option<UnknownType>,
    pub subRecord: Option<UnknownType>,
    pub to: Option<UnknownType>,
    pub top: Option<UnknownType>,
    pub topRecord: Option<UnknownType>,
    pub tt: Option<UnknownType>,
    pub ttRecord: Option<UnknownType>,
    pub zero: Option<UnknownType>,
    pub zeroRecord: Option<UnknownType>,
}

