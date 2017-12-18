#![cfg(test)]

#[macro_use]
extern crate serde_derive;
extern crate toml;


mod config;

use config::toml::{ Config, CONFIG };


#[test]
fn test_declarations()
{
    let _conf: &Config = &CONFIG;
}

#[test]
fn test_deserialization()
{
    let toml_source = include_str!("../config.toml");
    let conf: Config = toml::from_str(toml_source).unwrap();
    assert_eq!(conf.name, "Config name");
}

#[test]
fn test_simple_values()
{
    assert_eq!(CONFIG.name, "Config name");
    assert_eq!(CONFIG.number, 100);
    assert_eq!(CONFIG.is_config, true);
    assert_eq!(CONFIG.is_not_config, false);
    assert_eq!(CONFIG.one_point_zero, 1.0);
    assert_eq!(CONFIG.one_point_five, 1.5);
    assert_eq!(CONFIG.floaty, 123.456789);
}

#[test]
fn test_simple_array_values()
{
    assert_eq!(CONFIG.coord, [-5.0, 5.0].as_ref());
    assert_eq!(CONFIG.color, [0, 64, 128, 255].as_ref());
    assert_eq!(CONFIG.words, ["one", "two", "three"].as_ref());
    assert_eq!(CONFIG.points, [[1, 2].as_ref(), [3, 4].as_ref(), [5, 6].as_ref()].as_ref());
}

#[test]
fn test_table_values()
{
    assert_eq!(CONFIG.table.name, "A table");
    assert_eq!(CONFIG.table.magnitude, 1000000000);
}

#[test]
fn test_nested_tables()
{
    assert_eq!(CONFIG.table.table_again.name, "OK this is just getting ridiculous");
    assert_eq!(CONFIG.table.table_again.description, "getting ridiculous");
}

#[test]
fn test_array_of_tables()
{
    assert_eq!(CONFIG.arrayble[0].description, "just unbelievable");
    assert_eq!(CONFIG.arrayble[1].description, "what is this syntax");
}
