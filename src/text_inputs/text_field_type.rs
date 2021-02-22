/// The `TextFieldType` type
#[derive(Debug, Clone)]
pub enum TextFieldType {
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DatetimeLocal,
    Number,
    Color,
}

impl ToString for TextFieldType {
    fn to_string(&self) -> String {
        use TextFieldType::*;
        match self {
            Text => "text",
            Search => "search",
            Tel => "tel",
            Url => "url",
            Email => "email",
            Password => "password",
            Date => "date",
            Month => "month",
            Week => "week",
            Time => "time",
            DatetimeLocal => "datetime-local",
            Number => "number",
            Color => "color",
        }
        .to_string()
    }
}
