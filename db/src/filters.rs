use std::marker::PhantomData;

use sqlx::{Encode, Postgres, Type};

type QueryBuilder<'args> = sqlx::QueryBuilder<'args, Postgres>;

// pub trait WithFilter<'args> {
//     fn with_filter<T>(&mut self, filter: T) -> &mut QueryBuilder<'args>
//     where
//         T: Expression;
// }
//
// impl<'args> WithFilter<'args> for QueryBuilder<'args> {
//     fn with_filter<T>(&mut self, filter: T) -> &mut QueryBuilder<'args>
//     where
//         T: Expression,
//     {
//         self
//     }
// }

type ExpressionFn<'a, Data> = fn(Data, &mut QueryBuilder<'a>);
type ExpressionTuple<'a, Data> = (ExpressionFn<'a, Data>, Data);

pub trait Expression<'a> {
    type Data;

    fn to_expression(self) -> Option<ExpressionTuple<'a, Self::Data>>;

    // fn and<R>(self, r: R) -> And<Self, R>
    // where
    //     Self: Sized,
    //     R: Filter,
    // {
    //     And(self, r)
    // }
    //
    // fn or<R>(self, r: R) -> Or<Self, R>
    // where
    //     Self: Sized,
    //     R: Filter,
    // {
    //     Or(self, r)
    // }
}

// This entire thing could be simplified when const generics support &str

// trait BinaryFilter {
//     type Left: Expression;
//     type Right: Expression;
//     const OP: &'static str;
//
//     fn inner_impl(data: BinaryFilterData<Self::Left, Self::Right>, builder: &mut QueryBuilder<'_>) {
//         match data {
//             BinaryFilterData::Left((f, d)) => f(d, builder),
//             BinaryFilterData::Right((f, d)) => f(d, builder),
//             BinaryFilterData::Both((lf, ld), (rf, rd)) => {
//                 lf(ld, builder);
//                 builder.push(Self::OP);
//                 rf(rd, builder);
//             }
//         }
//     }
//
//     fn get_data(
//         self,
//     ) -> (
//         Option<FilterTuple<<Self::Left as Expression>::Data>>,
//         Option<FilterTuple<<Self::Right as Expression>::Data>>,
//     );
// }
//
// enum BinaryFilterData<L, R>
// where
//     L: Expression,
//     R: Expression,
// {
//     Left(FilterTuple<L::Data>),
//     Right(FilterTuple<R::Data>),
//     Both(FilterTuple<L::Data>, FilterTuple<R::Data>),
// }
//
// impl<B, L, R> Expression for B
// where
//     B: BinaryFilter<Left = L, Right = R>,
//     L: Expression,
//     R: Expression,
// {
//     type Data = BinaryFilterData<B::Left, B::Right>;
//
//     fn to_expression(self) -> Option<(fn(Self::Data, &mut QueryBuilder<'_>), Self::Data)> {
//         match self.get_data() {
//             (Some(l), Some(r)) => Some((Self::inner_impl, BinaryFilterData::Both(l, r))),
//             (Some(l), None) => Some((Self::inner_impl, BinaryFilterData::Left(l))),
//             (None, Some(r)) => Some((Self::inner_impl, BinaryFilterData::Right(r))),
//             (None, None) => None,
//         }
//     }
// }
//
// pub struct And<L, R>(L, R)
// where
//     L: Expression,
//     R: Expression;
//
// impl<L, R> BinaryFilter for And<L, R>
// where
//     L: Expression,
//     R: Expression,
// {
//     type Left = L;
//     type Right = R;
//
//     const OP: &'static str = " and ";
//
//     fn get_data(
//         self,
//     ) -> (
//         Option<FilterTuple<<Self::Left as Expression>::Data>>,
//         Option<FilterTuple<<Self::Right as Expression>::Data>>,
//     ) {
//         (self.0.to_expression(), self.1.to_expression())
//     }
// }
//
// pub struct Or<L, R>(L, R)
// where
//     L: Expression,
//     R: Expression;
//
// impl<L, R> BinaryFilter for Or<L, R>
// where
//     L: Expression,
//     R: Expression,
// {
//     type Left = L;
//     type Right = R;
//
//     const OP: &'static str = " or ";
//
//     fn get_data(
//         self,
//     ) -> (
//         Option<FilterTuple<<Self::Left as Expression>::Data>>,
//         Option<FilterTuple<<Self::Right as Expression>::Data>>,
//     ) {
//         (self.0.to_expression(), self.1.to_expression())
//     }
// }

impl<'a, T> Expression<'a> for Option<T>
where
    T: Expression<'a>,
{
    type Data = T::Data;

    fn to_expression(self) -> Option<ExpressionTuple<'a, Self::Data>> {
        self.and_then(T::to_expression)
    }
}

pub trait ToSql<'a> {
    fn to_sql(self, builder: &mut QueryBuilder<'a>);
}

pub struct Comparison<'a, T, L, R>(L, R, CmpOperator, PhantomData<&'a T>)
where
    L: Comparable<'a, Type = T>,
    R: Comparable<'a, Type = T>;

impl<'a, T, L, R> ToSql<'a> for Comparison<'a, T, L, R>
where
    T: Encode<'a, Postgres>,
    L: Comparable<'a, Type = T>,
    R: Comparable<'a, Type = T>,
{
    fn to_sql(self, builder: &mut QueryBuilder<'a>) {
        let Self(l, r, op, _) = self;

        l.to_sql(builder);
        builder.push(op.to_sql());
        r.to_sql(builder);
    }
}

impl<'a, T, L, R> Expression<'a> for Comparison<'a, T, L, R>
where
    T: Encode<'a, Postgres>,
    L: Comparable<'a, Type = T>,
    R: Comparable<'a, Type = T>,
{
    type Data = Self;

    fn to_expression(self) -> Option<ExpressionTuple<'a, Self::Data>> {
        Some((Self::to_sql, self))
    }
}

pub trait SqlSymbol {
    fn to_sql(self) -> &'static str;
}

pub enum CmpOperator {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl SqlSymbol for CmpOperator {
    fn to_sql(self) -> &'static str {
        match self {
            CmpOperator::Eq => " = ",
            CmpOperator::Lt => " < ",
            CmpOperator::Gt => " > ",
            CmpOperator::Lte => " <= ",
            CmpOperator::Gte => " >= ",
        }
    }
}

pub struct Literal<'a, T>(&'a str, PhantomData<T>)
where
    T: Encode<'a, Postgres>;

pub struct Value<'a, T>(T, PhantomData<&'a ()>)
where
    T: Encode<'a, Postgres> + Send;

impl<'a, T> Expression<'a> for Literal<'a, T>
where
    T: Encode<'a, Postgres>,
{
    type Data = Self;

    fn to_expression(self) -> Option<ExpressionTuple<'a, Self::Data>> {
        Some((Self::to_sql, self))
    }
}

pub trait Comparable<'a>: ToSql<'a> + Sized {
    type Type;

    fn eq<R>(self, r: R) -> Comparison<'a, Self::Type, Self, R>
    where
        R: Comparable<'a, Type = Self::Type>,
    {
        Comparison(self, r, CmpOperator::Eq, PhantomData {})
    }
}

impl<'a, T> ToSql<'a> for Literal<'a, T>
where
    T: Encode<'a, Postgres>,
{
    fn to_sql(self, builder: &mut QueryBuilder<'a>) {
        builder.push(self.0);
    }
}

impl<'a, T> Comparable<'a> for Literal<'a, T>
where
    T: Encode<'a, Postgres>,
{
    type Type = T;
}

impl<'a, T> ToSql<'a> for Value<'a, T>
where
    T: 'a + Encode<'a, Postgres> + sqlx::Type<sqlx::Postgres> + Send,
{
    fn to_sql(self, builder: &mut QueryBuilder<'a>) {
        builder.push_bind(self.0);
    }
}

impl<'a, T> Comparable<'a> for Value<'a, T>
where
    T: 'a + Encode<'a, Postgres> + sqlx::Type<sqlx::Postgres> + Send,
{
    type Type = T;
}

fn literal<'a, T>(value: &'a str) -> Literal<'a, T>
where
    T: Encode<'a, Postgres>,
{
    Literal(value, PhantomData {})
}

fn value<'a, T>(value: T) -> Value<'a, T>
where
    T: 'a + Encode<'a, Postgres> + sqlx::Type<sqlx::Postgres> + Send,
{
    Value(value, PhantomData {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut builder = QueryBuilder::new("");
        value(10).eq(value(10)).to_sql(&mut builder);

        println!("{}", builder.sql());
    }
}
