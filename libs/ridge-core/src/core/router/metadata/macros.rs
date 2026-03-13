






#[macro_export]
macro_rules! meta_tags {
    ($buffer:expr, $( 
        // Pattern No 1: Meta tags (tag, attribute, name, value)
        ($tag:literal, $attr:literal, $name:expr, $value:expr) 
    ),* $(,)?) => {
        $(
            if let Some(v) = $value {
                $buffer.push_str("<");
                $buffer.push_str($tag);
                $buffer.push_str(" ");
                $buffer.push_str($attr);
                $buffer.push_str("=\"");
                $buffer.push_str($name);
                $buffer.push_str("\" content=\"");
                $buffer.push_str(v);
                $buffer.push_str("\" />\n");
            }
        )*
    };

    // Pattern No 2
    // Extension: Overload to handle Link tags specifically if needed
    (link: $buffer:expr, $( ($tag:literal, $attr:literal, $name:expr, $value:expr) ),* $(,)?) => {
        $(
            if let Some(v) = $value {
                $buffer.push_str("<");
                $buffer.push_str($tag);
                $buffer.push_str(" ");
                $buffer.push_str($attr);
                $buffer.push_str("=\"");
                $buffer.push_str($name);
                $buffer.push_str("\" href=\"");
                $buffer.push_str(v);
                $buffer.push_str("\" />\n");
            }
        )*
    };
}