use crate::prelude::*;
use biome_formatter::write;
use biome_r_syntax::RParameters;
use biome_r_syntax::RParametersFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParameters;
impl FormatNodeRule<RParameters> for FormatRParameters {
    fn fmt_fields(&self, node: &RParameters, f: &mut RFormatter) -> FormatResult<()> {
        let RParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ]
        )
    }
}
