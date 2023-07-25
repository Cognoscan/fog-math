use std::sync::OnceLock;

use fog_pack::{
    document::Document,
    schema::{Schema, SchemaBuilder},
    validator::*,
};

static SCHEMA_DOC: OnceLock<Document> = OnceLock::new();
static SCHEMA: OnceLock<Schema> = OnceLock::new();

pub fn schema() -> &'static Schema {
    SCHEMA.get_or_init(|| Schema::from_doc(schema_doc()).unwrap())
}

pub fn schema_doc() -> &'static Document {
    SCHEMA_DOC.get_or_init(|| {
        SchemaBuilder::new(
            ArrayValidator::new()
                .items(EnumValidator::new().build())
                .build(),
        )
        .type_add(
            "OpForm",
            EnumValidator::new()
                .insert("Prefix", None)
                .insert("Postfix", None)
                .insert("Infix", None)
                .build(),
        )
        .type_add(
            "Attributes",
            MapValidator::new()
                .opt_add(
                    "class",
                    ArrayValidator::new()
                        .items(StrValidator::new().build())
                        .build(),
                )
                .opt_add(
                    "rtl",
                    BoolValidator::new()
                        .comment("Set for right-to-left directionality")
                        .build(),
                )
                .opt_add("display_style", BoolValidator::new().build())
                .opt_add(
                    "variant",
                    StrValidator::new()
                        .in_add("Normal")
                        .in_add("Bold")
                        .in_add("Italic")
                        .in_add("BoldItalic")
                        .in_add("DoubleStruck")
                        .in_add("BoldFraktur")
                        .in_add("Script")
                        .in_add("BoldScript")
                        .in_add("Fraktur")
                        .in_add("SansSerif")
                        .in_add("BoldSansSerif")
                        .in_add("SansSerifItalic")
                        .in_add("SansSerifBoldItalic")
                        .in_add("Monospace")
                        .in_add("Initial")
                        .in_add("Tailed")
                        .in_add("Looped")
                        .in_add("Stretched")
                        .build(),
                )
                .opt_add(
                    "script_level",
                    EnumValidator::new()
                        .insert(
                            "Set",
                            Some(IntValidator::new().min(u32::MIN).max(u32::MAX).build()),
                        )
                        .insert(
                            "Add",
                            Some(IntValidator::new().min(i32::MIN).max(i32::MAX).build()),
                        )
                        .build(),
                )
                .opt_add(
                    "data",
                    MapValidator::new().values(Validator::new_any()).build(),
                )
                .build(),
        )
        .type_add(
            "Length",
            EnumValidator::new()
                .insert("Em", Some(Validator::F32(F32Validator::new())))
                .insert("Ex", Some(Validator::F32(F32Validator::new())))
                .build(),
        )
        .type_add(
            "LengthOrFraction",
            EnumValidator::new()
                .insert("Em", Some(Validator::F32(F32Validator::new())))
                .insert("Ex", Some(Validator::F32(F32Validator::new())))
                .insert("Frac", Some(Validator::F32(F32Validator::new())))
                .build(),
        )
        .type_add(
            "Pair",
            MapValidator::new()
                .req_add("sup", Validator::new_ref("Element"))
                .req_add("sub", Validator::new_ref("Element"))
                .build(),
        )
        .type_add(
            "Element",
            MapValidator::new()
                .opt_add("a", Validator::new_ref("Attributes"))
                .req_add(
                    "e",
                    EnumValidator::new()
                        .insert("Op", Some(StrValidator::new().max_char(1).build()))
                        .insert(
                            "Oper",
                            Some(
                                MapValidator::new()
                                    .req_add("t", StrValidator::new().max_char(1).build())
                                    .opt_add(
                                        "form",
                                        EnumValidator::new()
                                            .insert("Prefix", None)
                                            .insert("Postfix", None)
                                            .insert("Infix", None)
                                            .build(),
                                    )
                                    .opt_add("max_size", Validator::new_ref("LengthOrFraction"))
                                    .opt_add("min_size", Validator::new_ref("LengthOrFraction"))
                                    .opt_add("lspace", Validator::new_ref("LengthOrFraction"))
                                    .opt_add("rspace", Validator::new_ref("LengthOrFraction"))
                                    .opt_add("stretchy", BoolValidator::new().build())
                                    .opt_add("symmetric", BoolValidator::new().build())
                                    .opt_add("large_op", BoolValidator::new().build())
                                    .opt_add("movable_limits", BoolValidator::new().build())
                                    .opt_add("separator", BoolValidator::new().build())
                                    .opt_add("fence", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert(
                            "ResolvedOper",
                            Some(
                                MapValidator::new()
                                    .req_add("t", StrValidator::new().max_char(1).build())
                                    .req_add(
                                        "form",
                                        EnumValidator::new()
                                            .insert("Prefix", None)
                                            .insert("Postfix", None)
                                            .insert("Infix", None)
                                            .build(),
                                    )
                                    .req_add("max_size", Validator::new_ref("Length"))
                                    .req_add("min_size", Validator::new_ref("Length"))
                                    .req_add("lspace", Validator::new_ref("Length"))
                                    .req_add("rspace", Validator::new_ref("Length"))
                                    .req_add("stretchy", BoolValidator::new().build())
                                    .req_add("symmetric", BoolValidator::new().build())
                                    .req_add("large_op", BoolValidator::new().build())
                                    .req_add("movable_limits", BoolValidator::new().build())
                                    .req_add("separator", BoolValidator::new().build())
                                    .req_add("fence", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert("Text", Some(StrValidator::new().build()))
                        .insert(
                            "Id",
                            Some(
                                MapValidator::new()
                                    .req_add("t", StrValidator::new().build())
                                    .opt_add("normal", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert("Num", Some(StrValidator::new().build()))
                        .insert("Err", Some(StrValidator::new().build()))
                        .insert(
                            "Space",
                            Some(
                                MapValidator::new()
                                    .opt_add("width", Validator::new_ref("Length"))
                                    .opt_add("height", Validator::new_ref("Length"))
                                    .opt_add("depth", Validator::new_ref("Length"))
                                    .build(),
                            ),
                        )
                        .insert("Str", Some(StrValidator::new().build()))
                        .insert(
                            "Phantom",
                            Some(
                                ArrayValidator::new()
                                    .items(Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Row",
                            Some(
                                ArrayValidator::new()
                                    .items(Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Padding",
                            Some(
                                MapValidator::new()
                                    .opt_add(
                                        "elems",
                                        ArrayValidator::new()
                                            .items(Validator::new_ref("Element"))
                                            .build(),
                                    )
                                    .opt_add("width", Validator::new_ref("Length"))
                                    .opt_add("height", Validator::new_ref("Length"))
                                    .opt_add("depth", Validator::new_ref("Length"))
                                    .opt_add("lspace", Validator::new_ref("Length"))
                                    .opt_add("voffset", Validator::new_ref("Length"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Frac",
                            Some(
                                MapValidator::new()
                                    .req_add("num", Validator::new_ref("Element"))
                                    .req_add("den", Validator::new_ref("Element"))
                                    .opt_add("line_thickness", Validator::F32(F32Validator::new()))
                                    .build(),
                            ),
                        )
                        .insert("Sqrt", Some(Validator::new_ref("Element")))
                        .insert(
                            "Root",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("index", Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Sup",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("sup", Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Sub",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("sub", Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "SubSup",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("sup", Validator::new_ref("Element"))
                                    .req_add("sub", Validator::new_ref("Element"))
                                    .build(),
                            ),
                        )
                        .insert(
                            "Over",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("over", Validator::new_ref("Element"))
                                    .opt_add("accent", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert(
                            "Under",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("under", Validator::new_ref("Element"))
                                    .opt_add("accent_under", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert(
                            "UnderOver",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .req_add("under", Validator::new_ref("Element"))
                                    .req_add("over", Validator::new_ref("Element"))
                                    .opt_add("accent", BoolValidator::new().build())
                                    .opt_add("accent_under", BoolValidator::new().build())
                                    .build(),
                            ),
                        )
                        .insert(
                            "MultiScript",
                            Some(
                                MapValidator::new()
                                    .req_add("base", Validator::new_ref("Element"))
                                    .opt_add(
                                        "post",
                                        ArrayValidator::new()
                                            .items(Validator::new_ref("Pair"))
                                            .build(),
                                    )
                                    .opt_add(
                                        "pre",
                                        ArrayValidator::new()
                                            .items(Validator::new_ref("Pair"))
                                            .build(),
                                    )
                                    .build(),
                            ),
                        )
                        .insert(
                            "Table",
                            Some(
                                ArrayValidator::new()
                                    .items(
                                        MapValidator::new()
                                            .opt_add("a", Validator::new_ref("Attributes"))
                                            .opt_add(
                                                "cells",
                                                ArrayValidator::new()
                                                    .items(Validator::new_ref("TableCell"))
                                                    .build(),
                                            )
                                            .build(),
                                    )
                                    .build(),
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .type_add(
            "TableCell",
            MapValidator::new()
                .opt_add("col_span", IntValidator::new().min(0).max(u32::MAX).build())
                .opt_add("row_span", IntValidator::new().min(0).max(u32::MAX).build())
                .opt_add(
                    "elems",
                    ArrayValidator::new()
                        .items(Validator::new_ref("Element"))
                        .build(),
                )
                .opt_add("a", Validator::new_ref("Attributes"))
                .build(),
        )
        .name("fog-math")
        .version(1)
        .description("Formatted math, closely matching MathML.")
        .build()
        .unwrap()
    })
}
