use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};
use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "via.pest"]
struct ViaParser;

pub fn parse_file(path: &Path) -> Result<Vec<Resource>> {
    let src = fs::read_to_string(path)
        .with_context(|| format!("Failed to read Via file at {}", path.display()))?;
    parse_str(&src, path)
}

pub fn parse_str(src: &str, path: &Path) -> Result<Vec<Resource>> {
    let pairs = ViaParser::parse(Rule::file, src).map_err(|err| {
        let path_display = path.to_string_lossy();
        anyhow!("{}", err.with_path(path_display.as_ref()))
    })?;

    let mut resources = Vec::new();
    let mut pairs_iter = pairs.into_iter();
    if let Some(file_pair) = pairs_iter.next() {
        if file_pair.as_rule() != Rule::file {
            return Err(anyhow!(
                "Expected file rule, found {:?}",
                file_pair.as_rule()
            ));
        }
        for pair in file_pair.into_inner() {
            match pair.as_rule() {
                Rule::resource => resources.push(parse_resource(pair, path)?),
                Rule::EOI => {}
                other => {
                    return Err(anyhow!("Unexpected rule inside file: {:?}", other));
                }
            }
        }
    }

    Ok(resources)
}

fn parse_resource(pair: pest::iterators::Pair<'_, Rule>, path: &Path) -> Result<Resource> {
    let mut inner = pair.into_inner();
    let name_pair = inner
        .next()
        .ok_or_else(|| anyhow!("Resource missing identifier"))?;
    let name = name_pair.as_str().to_owned();

    let mut model: Option<Model> = None;
    let mut controller: Option<Controller> = None;

    for item in inner {
        match item.as_rule() {
            Rule::model_section => {
                model = Some(parse_model(item)?);
            }
            Rule::controller_section => {
                controller = Some(parse_controller(item)?);
            }
            other => {
                return Err(anyhow!("Unsupported resource item: {:?}", other));
            }
        }
    }

    Ok(Resource {
        name,
        model,
        controller,
        file_path: path.to_string_lossy().into_owned(),
    })
}

fn parse_model(pair: pest::iterators::Pair<'_, Rule>) -> Result<Model> {
    let mut fields = Vec::new();

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::field_decl => fields.push(parse_field(item)?),
            other => return Err(anyhow!("Unsupported model item: {:?}", other)),
        }
    }

    Ok(Model { fields })
}

fn parse_field(pair: pest::iterators::Pair<'_, Rule>) -> Result<Field> {
    let mut inner = pair.into_inner();
    let name_pair = inner.next().ok_or_else(|| anyhow!("Field missing name"))?;
    let (name, opt_flag) = parse_name_opt(name_pair)?;
    let ty_pair = inner.next().ok_or_else(|| anyhow!("Field missing type"))?;
    let ty = parse_type(ty_pair)?;

    let mut attributes = FieldAttributes::default();
    for attr_pair in inner {
        parse_field_attr(attr_pair, &mut attributes)?;
    }

    Ok(Field {
        name,
        optional: opt_flag || ty.optional,
        ty,
        attributes,
    })
}

fn parse_field_attr(
    pair: pest::iterators::Pair<'_, Rule>,
    attrs: &mut FieldAttributes,
) -> Result<()> {
    match pair.as_rule() {
        Rule::field_attr => {
            for inner in pair.into_inner() {
                parse_field_attr(inner, attrs)?;
            }
            Ok(())
        }
        Rule::serialize_attr => {
            let mut inner = pair.into_inner();
            let value_pair = inner
                .next()
                .ok_or_else(|| anyhow!("serialize attribute missing value"))?;
            attrs.serialize = Some(parse_bool(value_pair)?);
            Ok(())
        }
        other => Err(anyhow!("Unsupported field attribute variant: {:?}", other)),
    }
}

fn parse_controller(pair: pest::iterators::Pair<'_, Rule>) -> Result<Controller> {
    let mut controller = Controller::default();
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::params_section => {
                controller.params = parse_params_section(item)?;
            }
            Rule::respond_with_section => {
                controller.respond_with = parse_respond_with(item)?;
            }
            Rule::actions_section => {
                controller.actions = ControllerActions::AutoCrud;
            }
            other => {
                return Err(anyhow!("Unsupported controller item: {:?}", other));
            }
        }
    }

    Ok(controller)
}

fn parse_params_section(pair: pest::iterators::Pair<'_, Rule>) -> Result<Vec<ParamsProfile>> {
    let mut profiles = Vec::new();
    for profile_pair in pair.into_inner() {
        match profile_pair.as_rule() {
            Rule::params_profile => profiles.push(parse_params_profile(profile_pair)?),
            other => return Err(anyhow!("Unsupported params profile: {:?}", other)),
        }
    }

    Ok(profiles)
}

fn parse_params_profile(pair: pest::iterators::Pair<'_, Rule>) -> Result<ParamsProfile> {
    let mut inner = pair.into_inner();
    let name_pair = inner
        .next()
        .ok_or_else(|| anyhow!("Params profile missing name"))?;
    let kind = match name_pair.as_str() {
        "editable" => ParamsKind::Editable,
        other => ParamsKind::Named(other.to_owned()),
    };

    let mut entries = Vec::new();
    if let Some(list_pair) = inner.next() {
        match list_pair.as_rule() {
            Rule::param_entry_list => {
                for entry_pair in list_pair.into_inner() {
                    entries.push(parse_param_entry(entry_pair)?);
                }
            }
            Rule::param_entry => {
                entries.push(parse_param_entry(list_pair)?);
            }
            _ => {}
        }
    }

    Ok(ParamsProfile {
        name: kind,
        entries,
    })
}

fn parse_param_entry(pair: pest::iterators::Pair<'_, Rule>) -> Result<ParamEntry> {
    let (name, optional) = parse_name_opt(pair)?;
    Ok(ParamEntry { name, optional })
}

fn parse_respond_with(pair: pest::iterators::Pair<'_, Rule>) -> Result<Vec<String>> {
    let mut formats = Vec::new();
    if let Some(list_pair) = pair.into_inner().next() {
        match list_pair.as_rule() {
            Rule::format_list => {
                for format_pair in list_pair.into_inner() {
                    formats.push(format_pair.as_str().to_owned());
                }
            }
            Rule::ident => formats.push(list_pair.as_str().to_owned()),
            _ => {}
        }
    }
    Ok(formats)
}

fn parse_type(pair: pest::iterators::Pair<'_, Rule>) -> Result<TypeRef> {
    let mut inner = pair.into_inner();
    let ident = inner
        .next()
        .ok_or_else(|| anyhow!("Type missing identifier"))?;
    let optional = inner
        .next()
        .map(|mark| mark.as_rule() == Rule::optional_mark)
        .unwrap_or(false);

    Ok(TypeRef {
        name: ident.as_str().to_owned(),
        optional,
    })
}

fn parse_name_opt(pair: pest::iterators::Pair<'_, Rule>) -> Result<(String, bool)> {
    let preview = pair.clone().into_inner();
    if preview.clone().count() == 0 {
        return Ok((pair.as_str().to_owned(), false));
    }

    let mut inner = pair.into_inner();
    let ident_pair = inner.next().ok_or_else(|| anyhow!("Missing identifier"))?;
    let optional = inner
        .next()
        .map(|mark| mark.as_rule() == Rule::optional_mark)
        .unwrap_or(false);

    Ok((ident_pair.as_str().to_owned(), optional))
}

fn parse_bool(pair: pest::iterators::Pair<'_, Rule>) -> Result<bool> {
    match pair.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(anyhow!("Unexpected bool literal: {other}")),
    }
}
