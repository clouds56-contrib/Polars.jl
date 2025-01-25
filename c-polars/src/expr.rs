use jlrs::{data::managed::{ccall_ref::CCallRefRet, value::typed::TypedValue}, error::JlrsResult, prelude::{Bool, JuliaString}, weak_handle};
use polars::{
    lazy::dsl::{string::StringNameSpace, ListNameSpace},
    prelude::*,
};

use crate::polars_expr_t;

fn jlrs_make_expr(expr: Expr) -> CCallRefRet<polars_expr_t> {
    match weak_handle!() {
        Ok(handle) => CCallRefRet::new(TypedValue::new(handle, polars_expr_t { inner: expr }).leak()),
        Err(_) => panic!("not called from Julia"),
    }
}

macro_rules! gen_impl_expr {
    ($n: ident, $ns: expr, $t: expr) => {
        pub fn $n(&self) -> CCallRefRet<Self> {
            let expr = &self.inner;
            let out_expr = $t($ns(expr.clone()));
            jlrs_make_expr(out_expr)
        }
    };
    ($n: ident, $t: expr) => {
        pub fn $n(&self) -> CCallRefRet<Self> {
            let expr = &self.inner;
            let out_expr = $t(expr.clone());
            jlrs_make_expr(out_expr)
        }
    };

    ($($n: ident)*) => {
        $(gen_impl_expr!($n, Expr::$n);)*
    };
    ($ns:ident@$ns_trait:ty: $($n: ident)*) => {
        paste::paste!{
            $(gen_impl_expr!([<$ns _ $n>], Expr::$ns, $ns_trait::$n);)*
        }
    };
}

macro_rules! gen_impl_expr_binary {
    ($n: ident, $ns: expr, $t: expr) => {
        pub fn $n(
            &self,
            b: &Self,
        ) -> CCallRefRet<Self> {
            let a = &self.inner;
            let b = &b.inner;
            let out_expr = $t($ns(a.clone()), b.clone());
            jlrs_make_expr(out_expr)
        }
    };
    ($n: ident, $t: expr) => {
        pub fn $n(
            &self,
            b: &Self,
        ) -> CCallRefRet<Self> {
            let a = &self.inner;
            let b = &b.inner;
            let out_expr = $t(a.clone(), b.clone());
            jlrs_make_expr(out_expr)
        }
    };

    ($($n: ident)*) => {
        $(gen_impl_expr_binary!($n, Expr::$n);)*
    };
    ($ns:ident@$ns_trait:ty: $($n: ident)*) => {
        paste::paste!{
            $(gen_impl_expr_binary!([<$ns _ $n>], Expr::$ns, $ns_trait::$n);)*
        }
    };
}

impl polars_expr_t {
    pub fn literal_bool(value: Bool) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Boolean(value.as_bool())))
    }

    pub fn literal_null() -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Null))
    }

    pub fn literal_i32(value: i32) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Int32(value)))
    }

    pub fn literal_i64(value: i64) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Int64(value)))
    }

    pub fn literal_u32(value: u32) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::UInt32(value)))
    }

    pub fn literal_u64(value: u64) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::UInt64(value)))
    }

    pub fn literal_f32(value: f32) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Float32(value)))
    }

    pub fn literal_f64(value: f64) -> CCallRefRet<Self> {
        jlrs_make_expr(Expr::Literal(LiteralValue::Float64(value)))
    }

    pub fn literal_utf8(s: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(Expr::Literal(LiteralValue::String(s.as_str()?.into()))))
    }

    pub fn col(name: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(Expr::Column(name.as_str()?.into())))
    }

    pub fn alias(&self, name: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(self.inner.clone().alias(name.as_str()?)))
    }

    pub fn prefix(&self, name: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(self.inner.clone().name().prefix(name.as_str()?)))
    }

    pub fn suffix(&self, name: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(self.inner.clone().name().suffix(name.as_str()?)))
    }

    pub fn keep_name(&self) -> CCallRefRet<Self> {
        jlrs_make_expr(self.inner.clone().name().keep())
    }

    gen_impl_expr!(sum product mean median min max arg_min arg_max nan_min nan_max);
    gen_impl_expr!(floor ceil abs cos sin tan cosh sinh tanh);
    gen_impl_expr!(n_unique unique count first last);
    gen_impl_expr!(not is_finite is_infinite is_nan is_null is_not_null null_count drop_nans drop_nulls);
    gen_impl_expr!(implode flatten reverse);

    gen_impl_expr_binary!(eq lt gt or xor and pow);
    gen_impl_expr_binary!(add, core::ops::Add::add);
    gen_impl_expr_binary!(sub, core::ops::Sub::sub);
    gen_impl_expr_binary!(mul, core::ops::Mul::mul);
    gen_impl_expr_binary!(div, core::ops::Div::div);

    gen_impl_expr!(list@ListNameSpace: len max min arg_max arg_min sum mean reverse unique unique_stable first last);
    pub fn list_get(&self, b: &Self, null_on_oob: bool) -> CCallRefRet<Self> {
        jlrs_make_expr(ListNameSpace::get(self.inner.clone().list(), b.inner.clone(), null_on_oob))
    }
    gen_impl_expr_binary!(list@ListNameSpace: contains head);

    gen_impl_expr!(str@StringNameSpace: to_uppercase to_lowercase len_chars len_bytes);
    #[cfg(feature = "nightly")]
    gen_impl_expr!(str@StringNameSpace: to_titlecase);
    gen_impl_expr_binary!(str@StringNameSpace: starts_with ends_with contains_literal);

    pub fn struct_field_by_name(&self, name: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        Ok(jlrs_make_expr(self.inner.clone().struct_().field_by_name(name.as_str()?)))
    }

    pub fn struct_field_by_index(&self, fieldidx: i64) -> CCallRefRet<Self> {
        jlrs_make_expr(self.inner.clone().struct_().field_by_index(fieldidx))
    }

    pub fn struct_rename_fields(&self, names: Vec<JuliaString>) -> JlrsResult<CCallRefRet<Self>> {
        let names = names.iter().map(|s| s.as_str()).collect::<JlrsResult<Vec<_>>>()?;
        Ok(jlrs_make_expr(self.inner.clone().struct_().rename_fields(names)))
    }
}
