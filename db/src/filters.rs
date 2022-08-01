use sqlx::{types::time::OffsetDateTime, Encode, Postgres, Type};

type QueryBuilder<'args> = sqlx::QueryBuilder<'args, Postgres>;

pub trait WithFilter<'args> {
    fn with_filter<T>(&mut self, filter: T) -> &mut QueryBuilder<'args>
    where
        T: Filter;
}

impl<'args> WithFilter<'args> for QueryBuilder<'args> {
    fn with_filter<T>(&mut self, filter: T) -> &mut QueryBuilder<'args>
    where
        T: Filter,
    {
        self
    }
}

type FilterFn<Data> = fn(Data, &mut QueryBuilder<'_>);
type FilterTuple<Data> = (FilterFn<Data>, Data);

pub trait Filter {
    type Data;

    fn to_sql(self) -> Option<(FilterFn<Self::Data>, Self::Data)>;

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

trait BinaryFilter {
    type Left: Filter;
    type Right: Filter;
    const OP: &'static str;

    fn inner_impl(data: BinaryFilterData<Self::Left, Self::Right>, builder: &mut QueryBuilder<'_>) {
        match data {
            BinaryFilterData::Left((f, d)) => f(d, builder),
            BinaryFilterData::Right((f, d)) => f(d, builder),
            BinaryFilterData::Both((lf, ld), (rf, rd)) => {
                lf(ld, builder);
                builder.push(Self::OP);
                rf(rd, builder);
            }
        }
    }

    fn get_data(
        self,
    ) -> (
        Option<FilterTuple<<Self::Left as Filter>::Data>>,
        Option<FilterTuple<<Self::Right as Filter>::Data>>,
    );
}

enum BinaryFilterData<L, R>
where
    L: Filter,
    R: Filter,
{
    Left(FilterTuple<L::Data>),
    Right(FilterTuple<R::Data>),
    Both(FilterTuple<L::Data>, FilterTuple<R::Data>),
}

impl<B, L, R> Filter for B
where
    B: BinaryFilter<Left = L, Right = R>,
    L: Filter,
    R: Filter,
{
    type Data = BinaryFilterData<B::Left, B::Right>;

    fn to_sql(self) -> Option<(fn(Self::Data, &mut QueryBuilder<'_>), Self::Data)> {
        match self.get_data() {
            (Some(l), Some(r)) => Some((Self::inner_impl, BinaryFilterData::Both(l, r))),
            (Some(l), None) => Some((Self::inner_impl, BinaryFilterData::Left(l))),
            (None, Some(r)) => Some((Self::inner_impl, BinaryFilterData::Right(r))),
            (None, None) => None,
        }
    }
}

pub struct And<L, R>(L, R)
where
    L: Filter,
    R: Filter;

impl<L, R> BinaryFilter for And<L, R>
where
    L: Filter,
    R: Filter,
{
    type Left = L;
    type Right = R;

    const OP: &'static str = " and ";

    fn get_data(
        self,
    ) -> (
        Option<FilterTuple<<Self::Left as Filter>::Data>>,
        Option<FilterTuple<<Self::Right as Filter>::Data>>,
    ) {
        (self.0.to_sql(), self.1.to_sql())
    }
}

pub struct Or<L, R>(L, R)
where
    L: Filter,
    R: Filter;

impl<L, R> BinaryFilter for Or<L, R>
where
    L: Filter,
    R: Filter,
{
    type Left = L;
    type Right = R;

    const OP: &'static str = " or ";

    fn get_data(
        self,
    ) -> (
        Option<FilterTuple<<Self::Left as Filter>::Data>>,
        Option<FilterTuple<<Self::Right as Filter>::Data>>,
    ) {
        (self.0.to_sql(), self.1.to_sql())
    }
}

impl<T> Filter for Option<T>
where
    T: Filter,
{
    type Data = T::Data;

    fn to_sql(self) -> Option<(FilterFn<Self::Data>, Self::Data)> {
        self.and_then(T::to_sql)
    }
}
//
// pub enum DateFilter {
//     From(OffsetDateTime),
//     To(OffsetDateTime),
// }
//
// impl Filter for DateFilter {
//     fn to_sql(self) -> Option<String> {}
// }

pub struct CmpBuilder<'a> {
    name: &'a str,
}

pub fn cmp(name: &str) -> CmpBuilder {
    CmpBuilder { name }
}

pub struct Cmp<'a> {
    name: &'a str,
    op: &'static str,
}

#[cfg(test)]
mod tests {
    fn test() {}
}
