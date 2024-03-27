use crate::{append_to_path, output::write_file};

pub fn copy_default_files(moho_path: &str) {
    write_file(
        append_to_path(moho_path, "base.rhai"),
        include_str!("default/base.rhai").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "AActor.rhai"),
        include_str!("default/AActor.rhai").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "aactor-template-cpp.txt"),
        include_str!("default/aactor-template-cpp.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "aactor-template-h.txt"),
        include_str!("default/aactor-template-h.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "field-declaration-template.txt"),
        include_str!("default/field-declaration-template.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "field-definition-template.txt"),
        include_str!("default/field-definition-template.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "field-name-template.txt"),
        include_str!("default/field-name-template.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-array.txt"),
        include_str!("default/type-template-array.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-bool.txt"),
        include_str!("default/type-template-bool.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-char.txt"),
        include_str!("default/type-template-char.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-class.txt"),
        include_str!("default/type-template-class.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-float.txt"),
        include_str!("default/type-template-float.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-integer.txt"),
        include_str!("default/type-template-integer.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-matrix.txt"),
        include_str!("default/type-template-matrix.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-pointer.txt"),
        include_str!("default/type-template-pointer.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-ref.txt"),
        include_str!("default/type-template-ref.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "type-template-string.txt"),
        include_str!("default/type-template-string.txt").to_owned(),
    );

    write_file(
        append_to_path(moho_path, "method-declaration-template.txt"),
        include_str!("default/method-declaration-template.txt").to_owned(),
    );
}
