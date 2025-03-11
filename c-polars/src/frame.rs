use std::ffi::c_void;
use std::io::Write;

use jlrs::{data::managed::{ccall_ref::CCallRefRet, value::typed::TypedValue}, error::{JlrsError, JlrsResult}, prelude::{Bool, JuliaString}, weak_handle};
use polars::{frame::DataFrame, io::SerReader as _, prelude::{IntoLazy as _, LazyFrame, ParquetReader, ParquetWriter, SortMultipleOptions}};

use crate::{IOCallback, UserIOCallback};
use crate::{julia::{TypedVec, TypedVecExt as _}, polars_dataframe_t, polars_expr_t, polars_lazy_frame_t, polars_series_t, series::jlrs_make_series};

fn jlrs_make_dataframe(expr: DataFrame) -> CCallRefRet<polars_dataframe_t> {
    match weak_handle!() {
        Ok(handle) => CCallRefRet::new(TypedValue::new(handle, polars_dataframe_t { inner: expr }).leak()),
        Err(_) => panic!("not called from Julia"),
    }
}

impl polars_dataframe_t {
    pub fn new_from_series(series: TypedVec<polars_series_t>) -> JlrsResult<CCallRefRet<Self>> {
        let columns = series.as_slice().iter().map(|s| s.inner.clone()).collect();
        let df = DataFrame::new(columns).map_err(JlrsError::other)?;
        Ok(jlrs_make_dataframe(df))
    }

    pub fn write_parquet(&mut self, io: *const c_void, callback: IOCallback) -> JlrsResult<()> {
        let df = &mut self.inner;
        let w = UserIOCallback(callback, io);
        ParquetWriter::new(w).finish(df).map_err(JlrsError::other)?;
        Ok(())
    }

    pub fn read_parquet(path: JuliaString) -> JlrsResult<CCallRefRet<Self>> {
        let path = path.as_str().map_err(JlrsError::other)?;
        let file = std::fs::File::open(path).map_err(JlrsError::other)?;
        let df = ParquetReader::new(file).finish().map_err(JlrsError::other)?;
        Ok(jlrs_make_dataframe(df))
    }

    pub fn show(&mut self, io: *const c_void, callback: IOCallback) -> JlrsResult<()> {
        let df = &self.inner;
        let mut w = UserIOCallback(callback, io);
        write!(w, "{}", df).map_err(JlrsError::other)?;
        Ok(())
    }

    pub fn get(&mut self, name: JuliaString) -> JlrsResult<CCallRefRet<polars_series_t>> {
        let df = &mut self.inner;
        let name = name.as_str()?;
        let s = df.column(name).map_err(JlrsError::other)?.clone();
        Ok(jlrs_make_series(s))
    }

    pub fn lazy(&mut self) -> CCallRefRet<polars_lazy_frame_t> {
        jlrs_make_lazyframe(self.inner.clone().lazy())
    }
}

fn jlrs_make_lazyframe(expr: LazyFrame) -> CCallRefRet<polars_lazy_frame_t> {
    match weak_handle!() {
        Ok(handle) => CCallRefRet::new(TypedValue::new(handle, polars_lazy_frame_t { inner: expr }).leak()),
        Err(_) => panic!("not called from Julia"),
    }
}

impl polars_lazy_frame_t {
    pub fn clone(&self) -> CCallRefRet<Self> {
        jlrs_make_lazyframe(self.inner.clone())
    }

    pub fn sort(&mut self, exprs: TypedVec<polars_expr_t>, descending: TypedVec<Bool>, nulls_last: Bool, maintain_order: Bool) {
        let exprs = exprs.as_slice().iter().map(|s| s.inner.clone()).collect::<Vec<_>>();
        self.inner = std::mem::take(&mut self.inner).sort_by_exprs(&exprs, SortMultipleOptions::new()
            .with_order_descending_multi(descending.as_slice().iter().map(|b| b.as_bool()))
            .with_nulls_last(nulls_last.as_bool())
            .with_maintain_order(maintain_order.as_bool()));
    }

    pub fn select(&mut self, exprs: TypedVec<polars_expr_t>) {
        let exprs: Vec<_> = exprs.as_slice().iter().map(|expr| expr.inner.clone()).collect();
        self.inner = std::mem::take(&mut self.inner).select(exprs);
    }
}
