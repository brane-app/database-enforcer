use std::{fs, path::Component};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: Vec<String>,
    pub field_schemas: Vec<String>,
}

impl Table {
    pub fn parse(name: &str, content: &str) -> Self {
        let field_schemas = content
            .split('\n')
            .filter(|it| !it.is_empty())
            .map(|it| it.into())
            .collect::<Vec<String>>();

        Self {
            name: name.into(),
            fields: field_schemas
                .iter()
                .map(|it| it.split(' ').next().unwrap().into())
                .collect(),
            field_schemas,
        }
    }
}

pub fn load_schema(root: &str) -> Vec<Table> {
    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .map(|it| it.unwrap_or_else(|err| panic!("{}", err)).into_path())
        .filter(|it| it.is_file())
        .map(|it| {
            let component_clone = it.clone();

            Table::parse(
                component_clone
                    .components()
                    .collect::<Vec<Component>>()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap_or_else(|| panic!("Failed to into_string {:?}", it)),
                &fs::read_to_string(&it).unwrap_or_else(|err| panic!("{}", err)),
            )
        })
        .collect()
}
