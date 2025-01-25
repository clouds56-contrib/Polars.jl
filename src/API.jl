module API

using libpolars_jll
export libpolars_jll

using CEnum
using JlrsCore.Wrap

const libpolars_local = joinpath(@__DIR__, "../c-polars/target/debug/libpolars.so")
@static if isfile(libpolars_local)
    const libpolars = libpolars_local
end

@wrapmodule(libpolars, :polars_init)
function __init__()
    @initjlrs
end

struct ArrowSchema
    format::Cstring
    name::Cstring
    metadata::Cstring
    flags::Int64
    n_children::Int64
    children::Ptr{Ptr{ArrowSchema}}
    dictionary::Ptr{ArrowSchema}
    release::Ptr{Cvoid}
    private_data::Ptr{Cvoid}
end

struct ArrowArray
    length::Int64
    null_count::Int64
    offset::Int64
    n_buffers::Int64
    n_children::Int64
    buffers::Ptr{Ptr{Cvoid}}
    children::Ptr{Ptr{ArrowArray}}
    dictionary::Ptr{ArrowArray}
    release::Ptr{Cvoid}
    private_data::Ptr{Cvoid}
end

@cenum polars_value_type_t::UInt32 begin
    PolarsValueTypeNull = 0
    PolarsValueTypeBoolean = 1
    PolarsValueTypeUInt8 = 2
    PolarsValueTypeUInt16 = 3
    PolarsValueTypeUInt32 = 4
    PolarsValueTypeUInt64 = 5
    PolarsValueTypeInt8 = 6
    PolarsValueTypeInt16 = 7
    PolarsValueTypeInt32 = 8
    PolarsValueTypeInt64 = 9
    PolarsValueTypeFloat32 = 10
    PolarsValueTypeFloat64 = 11
    PolarsValueTypeList = 12
    PolarsValueTypeUtf8 = 13
    PolarsValueTypeStruct = 14
    PolarsValueTypeBinary = 15
    PolarsValueTypeUnknown = 16
end

mutable struct polars_dataframe_t end

mutable struct polars_error_t end

mutable struct polars_lazy_frame_t end

mutable struct polars_lazy_group_by_t end

mutable struct polars_series_t end

mutable struct polars_value_t end

# typedef intptr_t ( * IOCallback ) ( const void * user , const uint8_t * data , uintptr_t len )
"""
The callback provided for display functions, returns -1 on error.
"""
const IOCallback = Ptr{Cvoid}

function polars_version(out)
    @ccall libpolars.polars_version(out::Ptr{Ptr{UInt8}})::Csize_t
end

function polars_error_message(err, data)
    @ccall libpolars.polars_error_message(err::Ptr{polars_error_t}, data::Ptr{Ptr{UInt8}})::Csize_t
end

function polars_error_destroy(err)
    @ccall libpolars.polars_error_destroy(err::Ptr{polars_error_t})::Cvoid
end

function polars_dataframe_size(df, rows, cols)
    @ccall libpolars.polars_dataframe_size(df::Ptr{polars_dataframe_t}, rows::Ptr{Csize_t}, cols::Ptr{Csize_t})::Cvoid
end

"""
    polars_dataframe_new_from_carrow(cfield, carray)

Creates a DataFrame from a series of [`ArrowArray`](@ref) and [`ArrowSchema`](@ref) compatible the arrow C-ABI.

# Safety The field array should be valid [`ArrowSchema`](@ref) according to the C Data Interface. The array array should be valid [`ArrowArray`](@ref) according to the C Data Interface, this means that the memory ownership is transferred in the created arrow::Array. Therefore, the caller should *not* free the underlying memories for this arrow as this will be done through the release field of the array.

Returns null if something went wrong.
"""
function polars_dataframe_new_from_carrow(cfield, carray)
    @ccall libpolars.polars_dataframe_new_from_carrow(cfield::Ptr{ArrowSchema}, carray::ArrowArray)::Ptr{polars_dataframe_t}
end

"""
    polars_dataframe_schema(df)

Returns a [`ArrowSchema`](@ref) describing the dataframe's schema according to Arrow C Data interface.
"""
function polars_dataframe_schema(df)
    @ccall libpolars.polars_dataframe_schema(df::Ptr{polars_dataframe_t})::ArrowSchema
end

function polars_dataframe_new_from_series(series, nseries, out)
    @ccall libpolars.polars_dataframe_new_from_series(series::Ptr{Ptr{polars_series_t}}, nseries::Csize_t, out::Ptr{Ptr{polars_dataframe_t}})::Ptr{polars_error_t}
end

function polars_dataframe_destroy(df)
    @ccall libpolars.polars_dataframe_destroy(df::Ptr{polars_dataframe_t})::Cvoid
end

function polars_dataframe_write_parquet(df, user, callback)
    @ccall libpolars.polars_dataframe_write_parquet(df::Ptr{polars_dataframe_t}, user::Ptr{Cvoid}, callback::IOCallback)::Ptr{polars_error_t}
end

function polars_dataframe_read_parquet(path, pathlen, out)
    @ccall libpolars.polars_dataframe_read_parquet(path::Ptr{UInt8}, pathlen::Csize_t, out::Ptr{Ptr{polars_dataframe_t}})::Ptr{polars_error_t}
end

function polars_dataframe_show(df, user, callback)
    @ccall libpolars.polars_dataframe_show(df::Ptr{polars_dataframe_t}, user::Ptr{Cvoid}, callback::IOCallback)::Cvoid
end

function polars_dataframe_get(df, name, len, out)
    @ccall libpolars.polars_dataframe_get(df::Ptr{polars_dataframe_t}, name::Ptr{UInt8}, len::Csize_t, out::Ptr{Ptr{polars_series_t}})::Ptr{polars_error_t}
end

function polars_dataframe_lazy(df)
    @ccall libpolars.polars_dataframe_lazy(df::Ptr{polars_dataframe_t})::Ptr{polars_lazy_frame_t}
end

function polars_lazy_frame_destroy(df)
    @ccall libpolars.polars_lazy_frame_destroy(df::Ptr{polars_lazy_frame_t})::Cvoid
end

function polars_lazy_frame_clone(df)
    @ccall libpolars.polars_lazy_frame_clone(df::Ptr{polars_lazy_frame_t})::Ptr{polars_lazy_frame_t}
end

function polars_lazy_frame_sort(df, exprs, nexprs, descending, nulls_last, maintain_order)
    @ccall libpolars.polars_lazy_frame_sort(df::Ptr{polars_lazy_frame_t}, exprs::Ptr{Ptr{polars_expr_t}}, nexprs::Csize_t, descending::Ptr{Bool}, nulls_last::Bool, maintain_order::Bool)::Cvoid
end

function polars_lazy_frame_concat(lfs, n, out)
    @ccall libpolars.polars_lazy_frame_concat(lfs::Ptr{Ptr{polars_lazy_frame_t}}, n::Csize_t, out::Ptr{Ptr{polars_lazy_frame_t}})::Ptr{polars_error_t}
end

function polars_lazy_frame_with_columns(df, exprs, nexprs)
    @ccall libpolars.polars_lazy_frame_with_columns(df::Ptr{polars_lazy_frame_t}, exprs::Ptr{Ptr{polars_expr_t}}, nexprs::Csize_t)::Cvoid
end

function polars_lazy_frame_select(df, exprs, nexprs)
    @ccall libpolars.polars_lazy_frame_select(df::Ptr{polars_lazy_frame_t}, exprs::Ptr{Ptr{polars_expr_t}}, nexprs::Csize_t)::Cvoid
end

function polars_lazy_frame_filter(df, expr)
    @ccall libpolars.polars_lazy_frame_filter(df::Ptr{polars_lazy_frame_t}, expr::Ptr{polars_expr_t})::Cvoid
end

function polars_lazy_frame_collect(df, out)
    @ccall libpolars.polars_lazy_frame_collect(df::Ptr{polars_lazy_frame_t}, out::Ptr{Ptr{polars_dataframe_t}})::Ptr{polars_error_t}
end

function polars_lazy_frame_group_by(df, exprs, nexprs)
    @ccall libpolars.polars_lazy_frame_group_by(df::Ptr{polars_lazy_frame_t}, exprs::Ptr{Ptr{polars_expr_t}}, nexprs::Csize_t)::Ptr{polars_lazy_group_by_t}
end

function polars_lazy_frame_join_inner(a, b, exprs_a, exprs_a_len, exprs_b, exprs_b_len)
    @ccall libpolars.polars_lazy_frame_join_inner(a::Ptr{polars_lazy_frame_t}, b::Ptr{polars_lazy_frame_t}, exprs_a::Ptr{Ptr{polars_expr_t}}, exprs_a_len::Csize_t, exprs_b::Ptr{Ptr{polars_expr_t}}, exprs_b_len::Csize_t)::Ptr{polars_lazy_frame_t}
end

function polars_lazy_frame_fetch(df, n, out)
    @ccall libpolars.polars_lazy_frame_fetch(df::Ptr{polars_lazy_frame_t}, n::Csize_t, out::Ptr{Ptr{polars_dataframe_t}})::Ptr{polars_error_t}
end

function polars_lazy_group_by_destroy(gb)
    @ccall libpolars.polars_lazy_group_by_destroy(gb::Ptr{polars_lazy_group_by_t})::Cvoid
end

function polars_lazy_group_by_agg(gb, exprs, nexprs)
    @ccall libpolars.polars_lazy_group_by_agg(gb::Ptr{polars_lazy_group_by_t}, exprs::Ptr{Ptr{polars_expr_t}}, nexprs::Csize_t)::Ptr{polars_lazy_frame_t}
end

function polars_expr_cast(expr, dtype)
    @ccall libpolars.polars_expr_cast(expr::Ptr{polars_expr_t}, dtype::polars_value_type_t)::Ptr{polars_expr_t}
end

function polars_series_destroy(series)
    @ccall libpolars.polars_series_destroy(series::Ptr{polars_series_t})::Cvoid
end

function polars_series_type(series)
    @ccall libpolars.polars_series_type(series::Ptr{polars_series_t})::polars_value_type_t
end

function polars_series_length(series)
    @ccall libpolars.polars_series_length(series::Ptr{polars_series_t})::Csize_t
end

function polars_series_null_count(series)
    @ccall libpolars.polars_series_null_count(series::Ptr{polars_series_t})::Csize_t
end

function polars_series_schema(series)
    @ccall libpolars.polars_series_schema(series::Ptr{polars_series_t})::ArrowSchema
end

"""
    polars_series_is_null(series, index)

Returns whether or not the value at index `index` is null, return false if the index is out of bounds.
"""
function polars_series_is_null(series, index)
    @ccall libpolars.polars_series_is_null(series::Ptr{polars_series_t}, index::Csize_t)::Bool
end

function polars_series_name(series, out)
    @ccall libpolars.polars_series_name(series::Ptr{polars_series_t}, out::Ptr{Ptr{UInt8}})::Csize_t
end

function polars_series_get(series, index)
    @ccall libpolars.polars_series_get(series::Ptr{polars_series_t}, index::Csize_t)::Ptr{polars_value_t}
end

function polars_series_get_bool(series, index, out)
    @ccall libpolars.polars_series_get_bool(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Bool})::Ptr{polars_error_t}
end

function polars_series_get_u8(series, index, out)
    @ccall libpolars.polars_series_get_u8(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{UInt8})::Ptr{polars_error_t}
end

function polars_series_get_u16(series, index, out)
    @ccall libpolars.polars_series_get_u16(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{UInt16})::Ptr{polars_error_t}
end

function polars_series_get_u32(series, index, out)
    @ccall libpolars.polars_series_get_u32(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{UInt32})::Ptr{polars_error_t}
end

function polars_series_get_u64(series, index, out)
    @ccall libpolars.polars_series_get_u64(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{UInt64})::Ptr{polars_error_t}
end

function polars_series_get_i8(series, index, out)
    @ccall libpolars.polars_series_get_i8(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Int8})::Ptr{polars_error_t}
end

function polars_series_get_i16(series, index, out)
    @ccall libpolars.polars_series_get_i16(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Int16})::Ptr{polars_error_t}
end

function polars_series_get_i32(series, index, out)
    @ccall libpolars.polars_series_get_i32(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Int32})::Ptr{polars_error_t}
end

function polars_series_get_i64(series, index, out)
    @ccall libpolars.polars_series_get_i64(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Int64})::Ptr{polars_error_t}
end

function polars_series_get_f32(series, index, out)
    @ccall libpolars.polars_series_get_f32(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Cfloat})::Ptr{polars_error_t}
end

function polars_series_get_f64(series, index, out)
    @ccall libpolars.polars_series_get_f64(series::Ptr{polars_series_t}, index::Csize_t, out::Ptr{Cdouble})::Ptr{polars_error_t}
end

function polars_value_type(value)
    @ccall libpolars.polars_value_type(value::Ptr{polars_value_t})::polars_value_type_t
end

function polars_value_destroy(value)
    @ccall libpolars.polars_value_destroy(value::Ptr{polars_value_t})::Cvoid
end

function polars_value_get_bool(value, out)
    @ccall libpolars.polars_value_get_bool(value::Ptr{polars_value_t}, out::Ptr{Bool})::Ptr{polars_error_t}
end

function polars_value_get_u8(value, out)
    @ccall libpolars.polars_value_get_u8(value::Ptr{polars_value_t}, out::Ptr{UInt8})::Ptr{polars_error_t}
end

function polars_value_get_u16(value, out)
    @ccall libpolars.polars_value_get_u16(value::Ptr{polars_value_t}, out::Ptr{UInt16})::Ptr{polars_error_t}
end

function polars_value_get_u32(value, out)
    @ccall libpolars.polars_value_get_u32(value::Ptr{polars_value_t}, out::Ptr{UInt32})::Ptr{polars_error_t}
end

function polars_value_get_u64(value, out)
    @ccall libpolars.polars_value_get_u64(value::Ptr{polars_value_t}, out::Ptr{UInt64})::Ptr{polars_error_t}
end

function polars_value_get_i8(value, out)
    @ccall libpolars.polars_value_get_i8(value::Ptr{polars_value_t}, out::Ptr{Int8})::Ptr{polars_error_t}
end

function polars_value_get_i16(value, out)
    @ccall libpolars.polars_value_get_i16(value::Ptr{polars_value_t}, out::Ptr{Int16})::Ptr{polars_error_t}
end

function polars_value_get_i32(value, out)
    @ccall libpolars.polars_value_get_i32(value::Ptr{polars_value_t}, out::Ptr{Int32})::Ptr{polars_error_t}
end

function polars_value_get_i64(value, out)
    @ccall libpolars.polars_value_get_i64(value::Ptr{polars_value_t}, out::Ptr{Int64})::Ptr{polars_error_t}
end

function polars_value_get_f32(value, out)
    @ccall libpolars.polars_value_get_f32(value::Ptr{polars_value_t}, out::Ptr{Cfloat})::Ptr{polars_error_t}
end

function polars_value_get_f64(value, out)
    @ccall libpolars.polars_value_get_f64(value::Ptr{polars_value_t}, out::Ptr{Cdouble})::Ptr{polars_error_t}
end

"""
    polars_value_list_get(value, out)

Returns the value as a Series when the dtype of the value is a list.
"""
function polars_value_list_get(value, out)
    @ccall libpolars.polars_value_list_get(value::Ptr{polars_value_t}, out::Ptr{Ptr{polars_series_t}})::Ptr{polars_error_t}
end

function polars_value_utf8_get(value, user, callback)
    @ccall libpolars.polars_value_utf8_get(value::Ptr{polars_value_t}, user::Ptr{Cvoid}, callback::IOCallback)::Ptr{polars_error_t}
end

function polars_value_binary_get(value, user, callback)
    @ccall libpolars.polars_value_binary_get(value::Ptr{polars_value_t}, user::Ptr{Cvoid}, callback::IOCallback)::Ptr{polars_error_t}
end

"""
    polars_value_struct_get(value, fieldidx, out)

Used to get value of of a Struct value fields.

NOTE: The value producing the new value must outlive the value from the field.

Safety: Values lifetimes must be valid and only support physical dtypes for now.
"""
function polars_value_struct_get(value, fieldidx, out)
    @ccall libpolars.polars_value_struct_get(value::Ptr{polars_value_t}, fieldidx::Csize_t, out::Ptr{Ptr{polars_value_t}})::Ptr{polars_error_t}
end

"""
    polars_value_list_type(value)

Returns the element type of the provided value which must be a list. The value type is PolarsValueTypeUnknown if the value is not a list so makes sure it is one otherwise, you cannot differentiate between list<unkown> and unkown.
"""
function polars_value_list_type(value)
    @ccall libpolars.polars_value_list_type(value::Ptr{polars_value_t})::polars_value_type_t
end

const ARROW_FLAG_DICTIONARY_ORDERED = 1

const ARROW_FLAG_NULLABLE = 2

const ARROW_FLAG_MAP_KEYS_SORTED = 4

# exports
const PREFIXES = ["polars_", "Polars"]
for name in names(@__MODULE__; all=true), prefix in PREFIXES
    if startswith(string(name), prefix)
        @eval export $name
    end
end

end # module
