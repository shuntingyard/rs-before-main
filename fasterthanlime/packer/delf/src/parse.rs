pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

// My very first macro - thanks, Amos :D
#[macro_export]
macro_rules! impl_parse_for_enum {
    ($type :ident, $number_parser: ident) => {
        impl $type {
            fn parse(i: parse::Input) -> parse::Result<Self> {
                use nom::{
                    combinator::map_res,
                    error::{context, ErrorKind},
                    number::complete::le_u16,
                };
                context(
                    "Machine",
                    map_res(le_u16, |x| Self::try_from(x).map_err(|_| ErrorKind::Alt)),
                )(i)
            }
        }
    };
}
