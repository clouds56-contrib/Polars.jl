use jlrs::convert::ccall_types::CCallArg;
use jlrs::data::types::abstract_type::RefTypeConstructor;
use jlrs::prelude::*;
use jlrs::data::managed::ccall_ref::CCallRefRet;
use jlrs::data::managed::ccall_ref::CCallRef;
use jlrs::data::managed::value::typed::TypedValue;
use crate::polars_expr_t;

unsafe impl CCallArg for &'_ polars_expr_t {
    type CCallArgType = RefTypeConstructor<polars_expr_t>;
    type FunctionArgType = polars_expr_t;
}

julia_module!{
    become polars_init;

    struct polars_expr_t;
    in polars_expr_t fn literal_bool(b: Bool) -> CCallRefRet<polars_expr_t> as polars_expr_literal_bool;
    in polars_expr_t fn literal_null() -> CCallRefRet<polars_expr_t> as polars_expr_literal_null;
    in polars_expr_t fn literal_i32(i: i32) -> CCallRefRet<polars_expr_t> as polars_expr_literal_i32;
    in polars_expr_t fn literal_i64(i: i64) -> CCallRefRet<polars_expr_t> as polars_expr_literal_i64;
    in polars_expr_t fn literal_u32(u: u32) -> CCallRefRet<polars_expr_t> as polars_expr_literal_u32;
    in polars_expr_t fn literal_u64(u: u64) -> CCallRefRet<polars_expr_t> as polars_expr_literal_u64;
    in polars_expr_t fn literal_f32(f: f32) -> CCallRefRet<polars_expr_t> as polars_expr_literal_f32;
    in polars_expr_t fn literal_f64(f: f64) -> CCallRefRet<polars_expr_t> as polars_expr_literal_f64;
    in polars_expr_t fn literal_utf8(value: JuliaString) -> JlrsResult<CCallRefRet<polars_expr_t>> as polars_expr_literal_utf8;

    in polars_expr_t fn col(name: JuliaString) -> JlrsResult<CCallRefRet<polars_expr_t>> as polars_expr_col;
    in polars_expr_t fn alias(&self, name: JuliaString) -> JlrsResult<CCallRefRet<polars_expr_t>> as polars_expr_alias;
    in polars_expr_t fn prefix(&self, name: JuliaString) -> JlrsResult<CCallRefRet<polars_expr_t>> as polars_expr_prefix;
    in polars_expr_t fn suffix(&self, name: JuliaString) -> JlrsResult<CCallRefRet<polars_expr_t>> as polars_expr_suffix;
    in polars_expr_t fn keep_name(&self) -> CCallRefRet<polars_expr_t> as polars_expr_keep_name;

    in polars_expr_t fn sum(&self) -> CCallRefRet<polars_expr_t> as polars_expr_sum;
    in polars_expr_t fn product(&self) -> CCallRefRet<polars_expr_t> as polars_expr_product;
    in polars_expr_t fn mean(&self) -> CCallRefRet<polars_expr_t> as polars_expr_mean;
    in polars_expr_t fn median(&self) -> CCallRefRet<polars_expr_t> as polars_expr_median;
    in polars_expr_t fn min(&self) -> CCallRefRet<polars_expr_t> as polars_expr_min;
    in polars_expr_t fn max(&self) -> CCallRefRet<polars_expr_t> as polars_expr_max;
    in polars_expr_t fn arg_min(&self) -> CCallRefRet<polars_expr_t> as polars_expr_arg_min;
    in polars_expr_t fn arg_max(&self) -> CCallRefRet<polars_expr_t> as polars_expr_arg_max;
    in polars_expr_t fn nan_min(&self) -> CCallRefRet<polars_expr_t> as polars_expr_nan_min;
    in polars_expr_t fn nan_max(&self) -> CCallRefRet<polars_expr_t> as polars_expr_nan_max;
    in polars_expr_t fn floor(&self) -> CCallRefRet<polars_expr_t> as polars_expr_floor;
    in polars_expr_t fn ceil(&self) -> CCallRefRet<polars_expr_t> as polars_expr_ceil;
    in polars_expr_t fn abs(&self) -> CCallRefRet<polars_expr_t> as polars_expr_abs;
    in polars_expr_t fn cos(&self) -> CCallRefRet<polars_expr_t> as polars_expr_cos;
    in polars_expr_t fn sin(&self) -> CCallRefRet<polars_expr_t> as polars_expr_sin;
    in polars_expr_t fn tan(&self) -> CCallRefRet<polars_expr_t> as polars_expr_tan;
    in polars_expr_t fn cosh(&self) -> CCallRefRet<polars_expr_t> as polars_expr_cosh;
    in polars_expr_t fn sinh(&self) -> CCallRefRet<polars_expr_t> as polars_expr_sinh;
    in polars_expr_t fn tanh(&self) -> CCallRefRet<polars_expr_t> as polars_expr_tanh;
    in polars_expr_t fn n_unique(&self) -> CCallRefRet<polars_expr_t> as polars_expr_n_unique;
    in polars_expr_t fn unique(&self) -> CCallRefRet<polars_expr_t> as polars_expr_unique;
    in polars_expr_t fn count(&self) -> CCallRefRet<polars_expr_t> as polars_expr_count;
    in polars_expr_t fn first(&self) -> CCallRefRet<polars_expr_t> as polars_expr_first;
    in polars_expr_t fn last(&self) -> CCallRefRet<polars_expr_t> as polars_expr_last;
    in polars_expr_t fn not(&self) -> CCallRefRet<polars_expr_t> as polars_expr_not;
    in polars_expr_t fn is_finite(&self) -> CCallRefRet<polars_expr_t> as polars_expr_is_finite;
    in polars_expr_t fn is_infinite(&self) -> CCallRefRet<polars_expr_t> as polars_expr_is_infinite;
    in polars_expr_t fn is_nan(&self) -> CCallRefRet<polars_expr_t> as polars_expr_is_nan;
    in polars_expr_t fn is_null(&self) -> CCallRefRet<polars_expr_t> as polars_expr_is_null;
    in polars_expr_t fn is_not_null(&self) -> CCallRefRet<polars_expr_t> as polars_expr_is_not_null;
    in polars_expr_t fn null_count(&self) -> CCallRefRet<polars_expr_t> as polars_expr_null_count;
    in polars_expr_t fn drop_nans(&self) -> CCallRefRet<polars_expr_t> as polars_expr_drop_nans;
    in polars_expr_t fn drop_nulls(&self) -> CCallRefRet<polars_expr_t> as polars_expr_drop_nulls;
    in polars_expr_t fn implode(&self) -> CCallRefRet<polars_expr_t> as polars_expr_implode;
    in polars_expr_t fn flatten(&self) -> CCallRefRet<polars_expr_t> as polars_expr_flatten;
    in polars_expr_t fn reverse(&self) -> CCallRefRet<polars_expr_t> as polars_expr_reverse;

    in polars_expr_t fn eq(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_eq;
    in polars_expr_t fn gt(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_gt;
    in polars_expr_t fn lt(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_lt;
    in polars_expr_t fn or(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_or;
    in polars_expr_t fn xor(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_xor;
    in polars_expr_t fn and(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_and;
    in polars_expr_t fn pow(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_pow;

    in polars_expr_t fn add(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_add;
    in polars_expr_t fn sub(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_sub;
    in polars_expr_t fn mul(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_mul;
    in polars_expr_t fn div(&self, other: &polars_expr_t) -> CCallRefRet<polars_expr_t> as polars_expr_div;
}
