use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, multispace0, multispace1, char, digit1},
    sequence::{delimited, preceded, tuple},
    combinator::{map, recognize},
    multi::many0,
};

use crate::ir::{SignalType, Signal, Module, Assignment};

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        tuple((alpha1, take_while(is_identifier_char))),
    )(input)
}

fn parse_signal_type(input: &str) -> IResult<&str, SignalType> {
    let (input, typ) = preceded(multispace0, alt((
        map(tag("input"), |_| SignalType::Input),
        map(tag("output"), |_| SignalType::Output),
        map(tag("reg"), |_| SignalType::Reg),
    )))(input)?;
    Ok((input, typ))
}

fn parse_type_width(input: &str) -> IResult<&str, u8> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("u")(input)?;
    let (input, num) = digit1(input)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_signal_decl(input: &str) -> IResult<&str, Signal> {
    let (input, sig_type) = parse_signal_type(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, width) = parse_type_width(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, Signal {
        name: name.to_string(),
        sig_type,
        width,
    }))
}

fn parse_assignment(input: &str) -> IResult<&str, Assignment> {
    let (input, lhs) = preceded(multispace0, parse_identifier)(input)?;
    let (input, _) = preceded(multispace0, tag("="))(input)?;
    let (input, rhs1) = preceded(multispace0, parse_identifier)(input)?;
    let (input, _) = preceded(multispace0, tag("+"))(input)?;
    let (input, rhs2) = preceded(multispace0, parse_identifier)(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, Assignment {
        lhs: lhs.to_string(),
        rhs: format!("{} + {}", rhs1, rhs2),
    }))
}

pub fn parse_module(input: &str) -> Result<Module, String> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("module")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;

    let (input, decls) = many0(delimited(multispace0, parse_signal_decl, multispace0))(input)?;
    let (input, assigns) = many0(delimited(multispace0, parse_assignment, multispace0))(input)?;
    let (input, _) = tag("}")(input)?;

    Ok(Module {
        name: name.to_string(),
        signals: decls,
        assignments: assigns,
    })
}
