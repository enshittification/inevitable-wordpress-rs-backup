use wp_contextual::WpContextual;

// This test is validating that we are able to handle `#[WpContextualField]`s if its type
// has multiple path segments. That's why we use a helper mod and use fully qualified paths
// rather than the importing the mod.
mod wp_contextual_field_with_multiple_path_segments_helper;

#[derive(WpContextual)]
pub struct SparseFoo {
    #[WpContext(edit)]
    #[WpContextualField]
    pub bar: Option<wp_contextual_field_with_multiple_path_segments_helper::SparseBar>,
}

fn main() {
    let _ = FooWithEditContext {
        bar: wp_contextual_field_with_multiple_path_segments_helper::BarWithEditContext { baz: 0 },
    };
    let _ = SparseFooWithEditContext {
        bar: Some(
            wp_contextual_field_with_multiple_path_segments_helper::SparseBarWithEditContext {
                baz: Some(0),
            },
        ),
    };
    let bar_field = SparseFooFieldWithEditContext::Bar;
    assert_eq!(bar_field.as_field_name(), "bar");
}

uniffi::setup_scaffolding!();
