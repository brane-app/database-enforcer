use std::fs;
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
            Table::parse(
                &it.clone()
                    .into_os_string()
                    .into_string()
                    .unwrap_or_else(|os_string| panic!("Failed to into_string {:?}", os_string)),
                &fs::read_to_string(&it).unwrap_or_else(|err| panic!("{}", err)),
            )
        })
        .collect()
}
